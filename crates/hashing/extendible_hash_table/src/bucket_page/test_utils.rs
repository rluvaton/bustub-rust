use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Standard};
use common::{Comparator, PageKey, PageValue};
use crate::bucket_page::BucketPage;
use crate::bucket_page::errors::InsertionErrors;

pub(crate) fn insert_until_full<const ARRAY_SIZE: usize, Key: PageKey, Value: PageValue, KeyComparator: Comparator<Key> + Default>(bucket_page: &mut BucketPage<ARRAY_SIZE, Key, Value, KeyComparator>) -> Vec<(Key, Value)>
where
    Standard: Distribution<(Key, Value)>,
{
    let cmp = KeyComparator::default();
    let mut rng = thread_rng();

    let mut entries: Vec<(Key, Value)> = vec![];

    loop {
        let (key, value): (Key, Value) = rng.gen();

        match bucket_page.insert(&key, &value, &cmp) {
            Ok(_) => {
                entries.push((key, value));
                continue;
            }
            Err(err) => {
                match err {
                    // On bucket full we should stop
                    InsertionErrors::BucketIsFull => break,

                    // If already exists just try again
                    // this will not be an infinite loop as there is enough keys to fit in the page
                    InsertionErrors::KeyAlreadyExists => continue,
                }
            }
        }
    }

    assert!(bucket_page.is_full(), "bucket should be full");

    entries
}