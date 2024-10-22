use wasm_bindgen::prelude::*;
use bustub_instance::BustubInstance;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

use std::sync::{Arc, LazyLock, Mutex};
use execution_common::CheckOptions;
use transaction::Transaction;

static INSTANCE: LazyLock<Mutex<BustubInstance>> = LazyLock::new(|| Mutex::new(BustubInstance::in_memory(None)));

// TODO - https://stackoverflow.com/questions/47529643/how-to-return-a-string-or-similar-from-rust-in-webassembly

#[wasm_bindgen]
pub fn bus_tub_init() -> i32 {
    println!("Initialize BusTub...");

    let mut bustub = INSTANCE.lock().unwrap();
    bustub.generate_mock_table();
    
    // if bustub.buffer_pool_manager.is_some() {
        bustub.generate_test_table();
    // }
    
    bustub.enable_managed_txn();
    
    // Success
    0
}

#[wasm_bindgen]
pub fn bus_tub_execute_query(input: &str, prompt: &str, output: &str) -> i32 {
// pub fn bus_tub_execute_query(input: &str, prompt: &str, output: &str) -> (String, String) {
    let mut bustub = INSTANCE.lock().unwrap();
    println!("{}", input);
    
    let mut writer = bustub_instance::result_writer::HtmlWriter::default();
    let result = bustub.execute_user_input(input, &mut writer, CheckOptions::default());
    
    let output_string = match result {
        Ok(_) => writer.get_output().to_string(),
        Err(err) => format!("{:#?}", err),
    };
    
    let txn = bustub.current_managed_txn();
    
    let output_prompt = match txn {
        Some(txn) => {
            format!("txn{}", txn.get_transaction_id_human_readable())
        }
        None => prompt.to_string()
    };


    (output_string, output_prompt);
    
    0

}