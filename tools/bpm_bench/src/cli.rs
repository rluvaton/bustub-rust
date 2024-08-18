use clap::Parser;

/// Simple program to greet a person
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
    #[arg(long, default_value_t = 8)]
    pub scan_thread_n: usize,

    /// Number of lookup threads
    #[arg(long, default_value_t = 8)]
    pub get_thread_n: usize,

    /// Buffer pool size
    #[arg(long, default_value_t = 64)]
    pub bpm_size: usize,

    /// Number of pages
    #[arg(long, default_value_t = 6400)]
    pub db_size: usize,

    /// LRU-K size
    #[arg(long, default_value_t = 16)]
    pub lru_k_size: usize,
}
