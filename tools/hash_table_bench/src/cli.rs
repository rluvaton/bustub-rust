use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
    /// run bench for n milliseconds
    #[arg(long, default_value_t = 30_000)]
    pub duration: u64,

    /// Number of read threads
    #[arg(long, default_value_t = 4)]
    pub read_thread_n: usize,

    /// Number of write threads
    #[arg(long, default_value_t = 2)]
    pub write_thread_n: usize,

    /// Buffer pool size
    #[arg(long, default_value_t = 256)]
    pub bpm_size: usize,

    /// LRU-K size
    #[arg(long, default_value_t = 4)]
    pub lru_k_size: usize,

    /// Number of keys in the hash table
    #[arg(long, default_value_t = 100_000)]
    pub total_keys: usize,

    #[arg(long, default_value_t = 2048)]
    pub key_modify_range: usize,


}
