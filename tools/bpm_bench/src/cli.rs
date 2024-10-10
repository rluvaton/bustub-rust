use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
    /// run bpm bench for n milliseconds
    #[arg(long, default_value_t = 30_000)]
    pub duration: u64,

    /// enable disk latency
    #[arg(long, default_value_t = false)]
    pub latency: bool,

    /// Number of scan threads
    /// If you use 128MB buffer pool size use 1000 threads
    #[arg(long, default_value_t = 8)]
    pub scan_thread_n: usize,

    /// Number of lookup threads
    /// If you use 128MB buffer pool size use 1000 threads
    #[arg(long, default_value_t = 8)]
    pub get_thread_n: usize,


    // 32 * 1024 for 128MB
    /// Buffer pool size
    /// If you want like Postgres Buffer size (128MB) use 32 * 1024 in buffer pool
    /// 128MB / 4KB pages = (128 * 1024 KB) / 4 KB = 32 * 1024 KB
    #[arg(long, default_value_t = 64)]
    pub bpm_size: usize,

    /// Number of pages
    /// If you use 128MB buffer pool size, use 5GB db size: 32 * 1024 * 40
    #[arg(long, default_value_t = 6400)]
    pub db_size: usize,

    /// LRU-K size
    #[arg(long, default_value_t = 16)]
    pub lru_k_size: usize,
}
