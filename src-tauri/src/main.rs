// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod build_reqwest;
mod downloader;
mod parser;
use std::collections::HashMap;

use crate::parser::Parser;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(curl: &str, name: &str, age: &str) -> (String, HashMap<String, String>) {
    println!(
        "Hello, {}! You've been greeted from Rust!,{},{}",
        curl, name, age
    );
    let parser = Parser {};
    parser.from_curl(curl)
    // format!(
    //     "Hello, {}! You've been greeted from Rust!,{},{}",
    //     curl, name, age
    // )
}

#[tauri::command]
fn parser_curl(value: &str) -> String {
    println!("Hello, {}! You've been greeted from Rust!", value);
    let mut m = HashMap::new();
    m.insert(build_reqwest::Key::Curl, value);
    let b: build_reqwest::BuildReqwest = m.into();
    println!("build reqwest :{:?}", b);
    "aa".to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, parser_curl])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
