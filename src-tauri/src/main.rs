#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::thread;

mod extension;
mod ui;

#[macro_use]
extern crate lazy_static;

fn main() {
    println!("G-Variables -> main.rs -> started!");
    let extension_thread_handle = thread::spawn(|| {
        extension::run();
    });

    ui::show();

    extension_thread_handle.join().unwrap();
}
