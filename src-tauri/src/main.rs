// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    //will only run tui until gui is called explicitly
    refresh_rate_helper_lib::run();
    println!("Hello World");
}
