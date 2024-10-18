mod cli;

use crate::cli::Args;
use bustub_instance::result_writer::ComfyTableWriter;
use bustub_instance::BustubInstance;
use clap::Parser;
use execution_common::CheckOptions;
use parking_lot::Mutex;
use rustyline::config::BellStyle;
use rustyline::history::{DefaultHistory, FileHistory, History};
use rustyline::{Config, DefaultEditor, Editor};
use std::path::PathBuf;
use std::sync::Arc;
use std::panic;
use transaction::TransactionState;

type Shell = Editor<(), DefaultHistory>;


const DEFAULT_PROMPT: &'static str = "bustub> ";

// The bathtub emoji
const EMOJI_PROMPT: &'static str = "🛁> ";

const HISTORY_FILE_PATH: &'static str = "bustub-shell-history.txt";

fn main() -> rustyline::Result<()> {
    let args = Args::parse();

    let shell = get_shell(&args)?;

    let mut bustub = BustubInstance::from_file(PathBuf::from("test.db"), None);

    bustub.generate_mock_table();

    // if bustub.buffer_pool_manager.is_none() {
    //     bustub.generate_test_table();
    // }

    bustub.enable_managed_txn();

    println!("Welcome to the BusTub shell! Type \\help to learn more.\n");

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

            // TODO - Do we need this `if` as it seems like rustyline support TTY and no TTY
            if !args.disable_tty {
                query.push_str(read_line(&shell, line_prompt)?.as_str());

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
        let result = bustub.execute_user_input(&query, &mut writer, CheckOptions::empty());

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

fn get_shell(args: &Args) -> rustyline::Result<Arc<Mutex<Shell>>> {
    let config = Config::builder()
        .max_history_size(1024)?
        .auto_add_history(!args.disable_tty)
        .bell_style(BellStyle::None)
        .build();

    let file_history = FileHistory::with_config(config);

    let mut shell = DefaultEditor::with_history(
        config,
        file_history
    )?;

    if shell.load_history(HISTORY_FILE_PATH).is_ok() {
        println!("History loaded");
    }

    let shell = Arc::new(Mutex::new(shell));

    // Save history on panic
    {
        let shell = shell.clone();
        let orig_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            let append_history_result = shell.lock().append_history(HISTORY_FILE_PATH);
            let _ = append_history_result.inspect_err(|err| {
                eprintln!("Failed to save history before exit")
            });

            // Call original hook
            orig_hook(panic_info);
        }));
    }

    Ok(shell)
}


fn read_line(shell: &Arc<Mutex<Shell>>, prompt: String) -> rustyline::Result<String> {
    let res = shell.lock().readline(prompt.as_str());

    res.inspect_err(|_| {
        // Trying to save history
        let _ = shell
            .lock()
            .append_history(HISTORY_FILE_PATH)
            .inspect_err(|append_history_error| eprintln!("Failed to save history to file {:?}", append_history_error));
    })
}
