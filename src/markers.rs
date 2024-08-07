use lazy_static::lazy_static;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::sync::Mutex;

fn generate_marker() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect()
}

lazy_static! {
    pub static ref START_MARKER: Mutex<String> = Mutex::new(generate_marker());
    pub static ref END_MARKER: Mutex<String> = Mutex::new(generate_marker());
}
