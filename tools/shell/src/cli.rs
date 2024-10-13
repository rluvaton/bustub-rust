use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = false)]
    pub emoji_prompt: bool,

    #[arg(long, default_value_t = false)]
    pub disable_tty: bool,
}
