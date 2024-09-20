use std::{panic, process};

/// Abort process on panic, this should be used in thread
pub fn abort_process_on_panic() {
    // This can lead to corruption, but if we panicked it is a bug in the db (I think)
    // TODO - maybe do not abort as the DB can be in the middle of other things that can lead to corruption
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        orig_hook(panic_info);
        process::exit(1);
    }));
}
