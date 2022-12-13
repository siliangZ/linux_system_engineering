use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
fn main() {
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, term.clone()).unwrap();
    while !term.load(Ordering::Relaxed) {}
    println!("process is terminated by signal");
}
