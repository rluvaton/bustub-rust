mod cli;

use std::path::PathBuf;
use std::sync::Arc;
use clap::Parser;
use bustub_instance::BustubInstance;
use transaction::TransactionState;
use crate::cli::Args;


const DEFAULT_PROMPT: &'static str = "bustub> ";

// The bathtub emoji
const EMOJI_PROMPT: &'static str = "🛁> ";

fn main() {
    let args = Args::parse();

    let mut bustub = BustubInstance::from_file(PathBuf::from("test.db"), None);

    bustub.generate_mock_table();

    // if bustub.buffer_pool_manager.is_none() {
    //     bustub.generate_test_table();
    // }

    bustub.enable_managed_txn();

    println!("Welcome to the BusTub shell! Type \\help to learn more.\n");

    // linenoiseHistorySetMaxLen(1024);
    // linenoiseSetMultiLine(1);

    let prompt = (if args.emoji_prompt { EMOJI_PROMPT } else { DEFAULT_PROMPT }).to_string();

    loop {
        let mut query: String;

        let mut first_line = true;

        loop {
            let mut context_prompt = prompt.clone();

            if let Some(txn) = bustub.current_managed_txn().as_ref().cloned() {
                if txn.get_transaction_state() == TransactionState::Running {
                    context_prompt = format!("txn{} ({:?})> ", txn.get_transaction_id_human_readable(), txn.get_transaction_state());
                } else {
                    context_prompt = format!("txn{}> ", txn.get_transaction_id_human_readable());
                }
            }

            let line_prompt = if first_line { context_prompt } else { "... ".to_string() };

            if !args.disable_tty {
                // let query_string =
            }

        }
    }


}
