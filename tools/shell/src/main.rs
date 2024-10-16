mod cli;

use std::{panic, process};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use clap::Parser;
use rustyline::{Config, DefaultEditor};
use rustyline::history::{FileHistory, History};
use bustub_instance::BustubInstance;
use bustub_instance::result_writer::ComfyTableWriter;
use execution_common::CheckOptions;
use transaction::TransactionState;
use crate::cli::Args;


const DEFAULT_PROMPT: &'static str = "bustub> ";

// The bathtub emoji
const EMOJI_PROMPT: &'static str = "🛁> ";

fn main() -> rustyline::Result<()> {
    let args = Args::parse();

    let file_history = FileHistory::with_config(
        Config::builder()
            .max_history_size(1024)?
            .auto_add_history(!args.disable_tty)
            .build()

    );

    let mut rl = Arc::new(Mutex::new(DefaultEditor::with_history(
        Config::builder()
            .max_history_size(1024)?
            .auto_add_history(!args.disable_tty)
            .build(),
        file_history
    )?));
    if rl.lock().unwrap().load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let rl_clone = rl.clone();
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        orig_hook(panic_info);
        rl_clone.lock().unwrap().append_history("history.txt").expect("should append history");
        process::exit(1);
    }));


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
        let mut query: String = "".to_string();

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

            // TODO - Do we need this if as the line prompt should support it
            if !args.disable_tty {
                let res = rl.lock().unwrap().readline(line_prompt.as_str());

                let res = match res {
                    Ok(a) => Ok(a),
                    Err(err) => {
                        rl.lock().unwrap().append_history("history.txt").expect("should append history");
                        Err(err)
                    }
                }?;
                query.push_str(res.as_str());

                if query.ends_with(";") || query.starts_with("\\") {
                    break;
                }

                query.push_str(" ");
            } else {
                // line prompt should support when no tty
                unimplemented!()
            }

            first_line = false;
        }

        let mut writer = ComfyTableWriter::default();
        let result = bustub.execute_sql(&query, &mut writer, CheckOptions::empty());

        match result {
            Ok(_) => {
                for table in writer.get_tables() {
                    println!("{}", table);
                }
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
}
