extern crate console_error_panic_hook;

use bustub_instance::BustubInstance;
use std::panic;
use wasm_bindgen::prelude::*;
use bustub_instance::result_writer::HtmlWriter;
use execution_common::CheckOptions;

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

#[wasm_bindgen]
#[no_mangle]
pub fn bus_tub_init() -> usize {
    console_log!("Initialize BusTub...");

    // Print to browser console on panic
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

    let bustub = Box::new(bustub);
    
    // Leak the pointer so can have multiple mutable references without dealing with rust
    let add = Box::leak(bustub);

    add as *mut BustubInstance as usize
}

#[wasm_bindgen]
pub fn bus_tub_execute_query(pointer: usize, input: &str, buffer_size: usize) -> Vec<String> {
    let mut bustub = unsafe { Box::from_raw(pointer as *mut BustubInstance) };
    
    let mut writer = HtmlWriter::default();
    let result = bustub.execute_user_input(input, &mut writer, CheckOptions::default());
    
    let has_error = result.is_err();
    
    let mut output_string = match result {
        Ok(_) => writer.get_output().to_string(),
        Err(err) => {
            let error_message = HtmlWriter::escape(err.to_string().as_str());
            
            // Format error message in red
            format!("<span style=\"color: red\">{}</span>", error_message)
        },
    };
    
    let txn = bustub.current_managed_txn();
    
    let output_prompt = match txn {
        Some(txn) => {
            format!("txn{}", txn.get_transaction_id_human_readable())
        }
        None => "".to_string()
    };
    
    if !has_error {
        // + 1 to indicate to JS that it was truncate
        output_string.truncate(buffer_size + 1);
    }


    // Leak again the instance to avoid the instance from being freed
    let new_leak = Box::leak(bustub);

    vec![(new_leak as *mut BustubInstance as usize).to_string(), output_string, output_prompt]
}

#[cfg(test)]
mod tests {
    use crate::bus_tub_init;

    #[test]
    fn init() {
        bus_tub_init();
    }
}
