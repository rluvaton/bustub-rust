use pages::{PageId, PAGE_SIZE};
use common::Future;
use parking_lot::Mutex;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::time::Duration;
use error_utils::anyhow::anyhow;
use error_utils::ToAnyhowResult;
use super::utils::get_file_size;
use crate::DiskManager;

static BUFFER_USED: Mutex<Option<Vec<u8>>> = Mutex::new(None);

/**
 * DiskManager takes care of the allocation and deallocation of pages within a database. It performs the reading and
 * writing of pages to and from disk, providing a logical file layer within the context of a database management system.
 */
pub struct DefaultDiskManager {
    inner: Mutex<InnerDefaultDiskManager>,


    num_flushes: AtomicI32,
    num_writes: AtomicI32,
    flush_log: AtomicBool,
}

struct InnerDefaultDiskManager {
    // stream to write log file
    // std::fstream log_io_;
    log_io: File,
    log_name: PathBuf,
    // stream to write db file
    // std::fstream db_io_;
    // db_io: File,
    file_name: PathBuf,

    // With multiple buffer pool instances, need to protect file access
    db_io: File,

    // std::future<void> *flush_log_f_{nullptr};
    flush_log_f: Option<Future<()>>,
}

impl DefaultDiskManager {
    /**
     * Creates a new disk manager that writes to the specified database file.
     * @param db_file the file name of the database file to write to
     */
    pub fn new(db_file: PathBuf) -> error_utils::anyhow::Result<DefaultDiskManager> {
        if db_file.extension().is_none() {
            println!("wrong file format");

            return Err(anyhow!("wrong file format"));
        }

        let file_name = db_file;


        let mut log_name = file_name.clone();
        log_name.set_extension("log");

        let log_io = OpenOptions::new()
            .read(true) // std::ios::in
            .write(true) // std::ios::out
            .append(true) // std::ios::app
            // no std::ios::binary in rust
            .create(true) // if missing, create the file
            .open(&log_name)
            .to_anyhow()?;

        // directory or file does not exist
        // if (!log_io.is_open()) {
        //     log_io_.clear();
        //     // create a new file
        //     log_io_.open(log_name_, std::ios::binary | std::ios::trunc | std::ios::out | std::ios::in);
        //     if (!log_io_.is_open()) {
        //         throw Exception("can't open dblog file");
        //     }
        // }



        let db_io = OpenOptions::new()
            .read(true) // std::ios::in
            .write(true) // std::ios::out
            // no std::ios::binary in rust
            .create(true) // if missing, create the file
            .open(&file_name)
            .to_anyhow()?;


        // db_io_.open(db_file, std::ios::binary | std::ios::in | std::ios::out);
        // // directory or file does not exist
        // if (!db_io_.is_open()) {
        //     db_io_.clear();
        //     // create a new file
        //     db_io_.open(db_file, std::ios::binary | std::ios::trunc | std::ios::out | std::ios::in);
        //     if (!db_io_.is_open()) {
        //         throw Exception("can't open db file");
        //     }
        // }


        // std::scoped_lock scoped_db_io_latch(db_io_latch_);
        // No scoped lock in rust
        // let db_io_latch = Mutex::new(db_io);

        // BUFFER_USED = None;

        Ok(
            DefaultDiskManager {
                flush_log: AtomicBool::new(false),
                num_flushes: AtomicI32::new(0),
                num_writes: AtomicI32::new(0),
                inner: Mutex::new(InnerDefaultDiskManager {
                    file_name,
                    log_name,
                    log_io,
                    db_io,
                    // db_io,
                    // TODO - implement flush_log
                    flush_log_f: None,
                })
            }
        )
    }
}

impl DiskManager for DefaultDiskManager {



    /**
     * Shut down the disk manager and close all the file resources.
     *
     * Close all file streams
     */
    fn shut_down(&mut self) {

        // TODO - implement
        // {
        //     std::scoped_lock scoped_db_io_latch(db_io_latch_);
        //     db_io_.close();
        // }
        // log_io_.close();

    }


    /**
     * Write a page to the database file.
     * @param page_id id of the page
     * @param page_data raw page data
     *
     * Write the contents of the specified page into disk file
     */
    fn write_page(&self, page_id: PageId, page_data: &[u8]) {
        let mut inner = self.inner.lock();
        // std::scoped_lock scoped_db_io_latch(db_io_latch_);

        let offset = page_id as u64 * PAGE_SIZE as u64;
        // set write cursor to offset
        self.num_writes.fetch_add(1, Ordering::Relaxed);

        // TODO - cpp seekp is the same as seek from start?
        let seek_res = inner.db_io.seek(SeekFrom::Start(offset));

        if seek_res.is_err() {
            println!("I/O error while writing (in seek)");
            return;
        }
        let write_res = inner.db_io.write(&page_data[0..PAGE_SIZE]);

        // check for I/O error
        if write_res.is_err() {
            println!("I/O error while writing (in write)");
            return;
        }

        // needs to flush to keep disk file in sync
        inner.db_io.flush().expect("Flush should work");
    }

    /**
     * Read a page from the database file.
     * @param page_id id of the page
     * @param[out] page_data output buffer
     *
     * Read the contents of the specified page into the given memory area
     */
    fn read_page(&self, page_id: PageId, page_data: &mut [u8]) {
        let mut inner = self.inner.lock();

        // std::scoped_lock scoped_db_io_latch(db_io_latch_);


        let offset = page_id * PAGE_SIZE as i32;
        // check if read beyond file length
        if offset > get_file_size(inner.file_name.as_path()) {
            println!("I/O error reading past end of file");
            // std::cerr << "I/O error while reading" << std::endl;

            return;
        }
        // set read cursor to offset
        // db_io.seekp(offset);
        let seek_result = inner.db_io.seek(SeekFrom::Start(offset as u64));
        if seek_result.is_err() {
            eprintln!("I/O error while reading (seek)");
            return;
        }
        let read_res = inner.db_io.read(&mut page_data[0..PAGE_SIZE]);

        if read_res.is_err() {
            eprintln!("I/O error while reading (read)");
            return;
        }

        // if file ends before reading BUSTUB_PAGE_SIZE
        // let read_count = db_io_.gcount();
        let read_count = read_res.unwrap();
        if read_count < PAGE_SIZE {
            eprintln!("Read less than a page");

            // Set the rest of the to be 0
            page_data[read_count..(PAGE_SIZE) - read_count].fill(0);
        }
    }

    /**
    * Flush the entire log buffer into disk.
    * @param log_data raw log data
    * @param size size of log entry

    * Write the contents of the log into disk file
    * Only return when sync is done, and only perform sequence write
     */
    fn write_log(&self, log_data: &[u8], size: i32) {
        let mut inner = self.inner.lock();
        let mut buffer_used = BUFFER_USED.lock();
        // enforce swap log buffer
        // TODO - fix this as this is not true
        // if buffer_used.is_some() {
        //     assert_ne!(log_data, buffer_used.unwrap())
        // }
        // assert_ne!(log_data, buffer_used.unwrap());
        *buffer_used = Some(log_data.to_vec());

        if size == 0 {  // no effect on num_flushes_ if log buffer is empty
            return;
        }

        self.flush_log.store(true, Ordering::SeqCst);

        if let Some(flush_log_f) = &inner.flush_log_f {
            // used for checking non-blocking flushing
            assert_eq!(flush_log_f.wait_for(Duration::from_secs(10)), true);
        }

        self.num_flushes.fetch_add(1, Ordering::Relaxed);

        // sequence write
        let res = inner.log_io.write(&log_data[0..size as usize]);

        // check for I/O error
        if res.is_err() {
            println!("I/O error while writing log");
            return;
        }

        // needs to flush to keep disk file in sync
        inner.log_io.flush().expect("Flush should work");

        self.flush_log.store(false, Ordering::SeqCst);
    }

    /**
     * Read a log entry from the log file.
     * @param[out] log_data output buffer
     * @param size size of the log entry
     * @param offset offset of the log entry in the file
     * @return true if the read was successful, false otherwise
     *
     * Read the contents of the log into the given memory area
     * Always read from the beginning and perform sequence read
     * @return: false means already reach the end
     */
    fn read_log(&self, log_data: &mut [u8], size: i32, offset: i32) -> bool {
        let mut inner = self.inner.lock();

        if offset > get_file_size(inner.log_name.as_path()) {
            // LOG_DEBUG("end of log file");
            // LOG_DEBUG("file size is %d", GetFileSize(log_name_));
            return false;
        }

        // set read cursor to offset
        // log_io.seekp(offset);
        let seek_result = inner.log_io.seek(SeekFrom::Start(offset as u64));
        if seek_result.is_err() {
            println!("I/O error while reading log (seek)");
            return false;
        }
        let read_res = inner.log_io.read(&mut log_data[0..size as usize]);

        if read_res.is_err() {
            println!("I/O error while reading log (read)");
            return false;
        }

        // if log file ends before reading "size"
        let read_count = read_res.unwrap();
        if read_count < size as usize {
            println!("Read less than a page");

            // Set the rest of the to be 0
            log_data[read_count..(size as usize) - read_count].fill(0);
        }

        true
    }

    /**
    @return the number of disk flushes
    Returns number of flushes made so far
    */
    fn get_num_flushes(&self) -> i32 {
        self.num_flushes.load(Ordering::Relaxed)
    }

    /**
    @return true iff the in-memory content has not been flushed yet
    Returns true if the log is currently being flushed
    */
    fn get_flush_state(&self) -> bool {
        self.flush_log.load(Ordering::Relaxed)
    }

    /**
    @return the number of disk writes
    Returns number of Writes made so far
    */
    fn get_num_writes(&self) -> i32 {
        self.num_writes.load(Ordering::Relaxed)
    }

    /**
     * Sets the future which is used to check for non-blocking flushes.
     * @param f the non-blocking flush check
     */
    fn set_flush_log_future(&mut self, f: Option<Future<()>>) {
        // TODO - change this to not lock

        self.inner.lock().flush_log_f = f;
    }

    /** Checks if the non-blocking flush future was set. */
    fn has_flush_log_future(&self) -> bool {
        // return flush_log_f_ != nullptr;
        // TODO - change this to not lock
        self.inner.lock().flush_log_f.is_some()
    }
}
