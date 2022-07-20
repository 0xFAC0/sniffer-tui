use std::thread;

use log::debug;

mod sniffer;
use rust_sniffer::{ui::start_ui, App};
use sniffer::sniff;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    debug!("Logger initialized");

    let (app, tx) = App::new();
    thread::spawn(move || sniff("enp0s3".to_string(), tx));
    start_ui(app).unwrap();
}
