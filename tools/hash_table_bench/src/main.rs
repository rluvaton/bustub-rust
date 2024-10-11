use crate::cli::Args;
use crate::metrics::metrics::Metrics;
use buffer_common::AccessType;
use buffer_pool_manager::{BufferPool, BufferPoolManager};

use clap::Parser;
use disk_storage::DiskManagerUnlimitedMemory;
use pages::PageId;
use parking_lot::{Mutex, RwLock};
use rand::distributions::Distribution;
use std::collections::HashMap;
use std::process::abort;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use crate::metrics::total_metrics::TotalMetrics;
use db_core::catalog::Schema;
use db_core::storage::{GenericComparator, GenericKey};
use extendible_hash_table::{bucket_array_size, DiskHashTable};
use hashing_common::DefaultKeyHasher;
use rid::RID;
#[cfg(feature = "tracing")]
use tracy_client::*;

// Tracking Memory usage
#[cfg(feature = "tracing-memory-allocation")]
#[global_allocator]
static GLOBAL: ProfiledAllocator<std::alloc::System> =
    ProfiledAllocator::new(std::alloc::System, 100);

mod cli;
mod metrics;

// Initial
// Release:
// <<< BEGIN
// write: 65645.28377298161
// read: 109707.93365307753
// >>> END


type Key = GenericKey<8>;
type Value = RID;

type CustomHashTable = DiskHashTable<
    { bucket_array_size::<Key, Value>() },
    Key,
    Value,
    GenericComparator<8>,
    DefaultKeyHasher
>;

// These keys will be deleted and inserted again
fn key_will_vanish(key: usize) -> bool {
    key % 7 == 0
}

// These keys will be overwritten to a new value
fn key_will_change(key: usize) -> bool {
    key % 5 == 0
}

fn main() {
    #[cfg(feature = "tracing")]
    let client = Client::start();

    let args = Args::parse();

    println!("args: {:?}", args);

    let duration_ms = args.duration;
    let write_thread_n = args.write_thread_n;
    let read_thread_n = args.read_thread_n;
    let bustub_bpm_size = args.bpm_size;
    let lru_k_size = args.lru_k_size;
    let total_keys = args.total_keys;
    let key_modify_range = args.key_modify_range;

    println!("[info] total_page={}, duration_ms={}, lru_k_size={}, bpm_size={}, write_thread_n={}, read_thread_n={}, key_modify_range={}",
             total_keys, duration_ms, lru_k_size, bustub_bpm_size, write_thread_n, read_thread_n, key_modify_range
    );

    let bpm = BufferPoolManager::builder()
        .with_pool_size(bustub_bpm_size)
        .with_disk_manager(DiskManagerUnlimitedMemory::new())
        .with_lru_k_eviction_policy(lru_k_size)
        .build_arc();

    let key_schema = Schema::parse_create_statement("a bigint").expect("Should create schema");
    let comparator = GenericComparator::<8>::from(key_schema);

    let index: Arc<CustomHashTable> = Arc::new(CustomHashTable::new(
        "foo_pk".to_string(),
        bpm,
        comparator,
        None,
        None,
        None,
    ).expect("Should be able to create disk based hash table"));

    init_hash_map(total_keys, &index);

    println!("[info] benchmark start");

    let total_metrics = Arc::new(Mutex::new(TotalMetrics::new()));

    total_metrics.lock().begin();

    let mut join_handles: Vec<JoinHandle<()>> = vec![];

    for thread_id in 0..read_thread_n {
        let index = Arc::clone(&index);
        let total_metrics = Arc::clone(&total_metrics);

        let t = thread::spawn(move || {
            let mut metrics = Metrics::new(format!("read {:>2}", thread_id), duration_ms);
            metrics.begin();

            let key_start = total_keys / read_thread_n * thread_id;
            let key_end = total_keys / read_thread_n * (thread_id + 1);

            let mut rng = rand::thread_rng();
            let dist = rand::distributions::Uniform::new(key_start, key_end - 1);

            let mut index_key: Key = Default::default();

            while !metrics.should_finish() {
                #[cfg(feature = "tracing")]
                let _read_iteration = span!("read thread");

                let base_key = dist.sample(&mut rng);
                let mut count = 0;

                for key in base_key..key_end {
                    if count >= key_modify_range {
                        break;
                    }

                    count += 1;

                    index_key.set_from_integer(key as i64);
                    let rids = {
                        #[cfg(feature = "tracing")]
                        let _fetch_page = span!("read thread - get_value");

                        index.get_value(&index_key, None).expect("must be able to get value")
                    };

                    assert!(key_will_vanish(key) || !rids.is_empty(), "Key {} not found", key);

                    if !key_will_vanish(key) && !key_will_change(key) {
                        assert_eq!(rids.len(), 1, "Key not found: {}", key);
                        assert_eq!(rids[0].get_page_id() as usize, key, "Invalid data: {} -> {}", key, rids[0].get());
                        assert_eq!(rids[0].get_slot_num() as usize, key, "Invalid data: {} -> {}", key, rids[0].get());
                    }

                    metrics.tick();
                    metrics.report();
                }
            }

            total_metrics.lock().report_read(metrics.count);
        });
        join_handles.push(t);
    }

    for thread_id in 0..write_thread_n {
        let index = Arc::clone(&index);
        let total_metrics = Arc::clone(&total_metrics);

        let t = thread::spawn(move || {
            let mut metrics = Metrics::new(format!("write {:>2}", thread_id), duration_ms);
            metrics.begin();

            let key_start = total_keys / write_thread_n * thread_id;
            let key_end = total_keys / write_thread_n * (thread_id + 1);

            let mut rng = rand::thread_rng();
            let dist = rand::distributions::Uniform::new(key_start, key_end - 1);

            let mut index_key: Key = Default::default();
            let mut rid: Value = RID::default();

            let mut do_insert = false;

            while !metrics.should_finish() {
                #[cfg(feature = "tracing")]
                let _read_iteration = span!("write thread");

                let base_key = dist.sample(&mut rng);
                let mut count = 0;

                for key in base_key..key_end {
                    if count >= key_modify_range {
                        break;
                    }

                    count += 1;

                    if key_will_vanish(key) {
                        let value = key;
                        rid.set(value as PageId, value as u32);
                        index_key.set_from_integer(key as i64);

                        if do_insert {
                            #[cfg(feature = "tracing")]
                            let _insert = span!("write thread - insert");

                            // This can fail when we're trying to insert key that is already exists
                            // it happens when we generate base random number in delete
                            // and then generate base random number in insert that is smaller than the delete one
                            let _ = index.insert(&index_key, &rid, None);
                        } else {
                            #[cfg(feature = "tracing")]
                            let _remove = span!("write thread - remove");

                            index.remove(&index_key, None).expect("Should remove when key will vanish");
                        }

                        metrics.tick();
                        metrics.report();
                    } else if key_will_change(key) {
                        let value = key as PageId;
                        rid.set(value, dist.sample(&mut rng) as u32);
                        index_key.set_from_integer(key as i64);
                        {
                            #[cfg(feature = "tracing")]
                            let _insert = span!("write thread - replace");

                            index.update(&index_key, &rid, None).expect("Should update value");
                        }

                        metrics.tick();
                        metrics.report();
                    }
                }

                do_insert = !do_insert;
            }

            total_metrics.lock().report_write(metrics.count);
        });

        join_handles.push(t);
    }

    for t in join_handles {
        t.join().unwrap();
    }

    total_metrics.lock().report();

    #[cfg(feature = "statistics")]
    {
        println!("\n\n");
        println!("###################");
        println!("       Stats       ");
        println!("###################");
        println!();
    }
}

fn init_hash_map(total_keys: usize, index: &CustomHashTable) {
    for key in 0..total_keys {
        let value: u32 = key as u32;

        let index_key = GenericKey::<8>::new_from_integer(key as i64);
        let rid = RID::new(value as PageId, value);

        index.insert(&index_key, &rid, None).expect(format!("Should insert key {}", key).as_str())
    }
}
