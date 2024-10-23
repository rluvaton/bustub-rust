extern crate console_error_panic_hook;
use bustub_instance::BustubInstance;
use wasm_bindgen::prelude::*;


// https://rustwasm.github.io/wasm-bindgen/examples/console-log.html
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the Wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


use execution_common::CheckOptions;
use std::panic;
use std::sync::Mutex;


#[wasm_bindgen]
pub struct BustubInstanceShell {
    bustub: Mutex<BustubInstance>,
}

#[wasm_bindgen]
#[no_mangle]
pub fn bus_tub_init() -> BustubInstanceShell {
    console_log!("Initialize BusTub...");
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    console_log!("Acquiring bustub instance");
    let mut bustub = BustubInstance::in_memory(None);
    console_log!("Bustub instance acquired");

    console_log!("Generating mock table");
    bustub.generate_mock_table();
    console_log!("Mock table generated");

    // if bustub.buffer_pool_manager.is_some() {
    console_log!("Generating test table");
        bustub.generate_test_table();
    console_log!("Test table generated");
    // }

    console_log!("Enabling managed txn");
    bustub.enable_managed_txn();
    console_log!("Managed txn enabled");

    console_log!("Bustub initialized successfully");

    // Success
    BustubInstanceShell {
        bustub: Mutex::new(bustub)
    }
}




#[wasm_bindgen]
#[no_mangle]
pub fn bus_tub_execute_query(bustub: &BustubInstanceShell, input: &str, buffer_size: usize) -> Vec<String> {
    // pub fn bus_tub_execute_query(input: &str, prompt: &str, output: &str) -> (String, String) {
    println!("{}", input);

    let mut writer = bustub_instance::result_writer::HtmlWriter::default();
    let result = bustub.bustub.lock().unwrap().execute_user_input(input, &mut writer, CheckOptions::default());

    let mut output_string = match result {
        Ok(_) => writer.get_output().to_string(),
        Err(err) => format!("{:#?}", err),
    };

    let txn = bustub.bustub.lock().unwrap().current_managed_txn();

    let output_prompt = match txn {
        Some(txn) => {
            format!("txn{}", txn.get_transaction_id_human_readable())
        }
        None => "".to_string()
    };

    output_string.truncate(buffer_size + 1);

    // (output_string, output_prompt);
    
    vec![output_string, output_prompt]

}

#[cfg(test)]
mod tests {
    use crate::bus_tub_init;

    #[test]
    fn init() {
        bus_tub_init();
    }
}