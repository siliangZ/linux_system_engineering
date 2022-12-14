use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn term_signal_handler() {
    println!("received term signal");
}

fn register_signal_handler_unsafe() {
    unsafe {
        signal_hook::low_level::register(signal_hook::consts::SIGTERM, term_signal_handler)
            .unwrap();
    }
}

fn register_signal() {
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, term.clone()).unwrap();
}
fn main() {}
