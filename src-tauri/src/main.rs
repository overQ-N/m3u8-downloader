// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod build_reqwest;
mod downloader;
mod parser;
mod response;
use std::{collections::HashMap, path};

use tauri::{api::file, Manager};

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
fn detect_file(download_path: &str, output_file: &str) -> response::Response {
    let full_path = format!("{download_path}/{output_file}");
    let p = path::Path::new(&full_path);
    let download_p = path::Path::new(download_path);

    let mut res = response::Response {
        code: 200,
        data: "".into(),
        msg: "".into(),
    };
    if !download_p.exists() {
        res.code = 400;
        res.msg = "文件路径不存在".into();
        return res;
    }
    if p.exists() {
        res.code = 409;
        res.msg = "文件已存在，是否覆盖？".into();
        return res;
    }
    return res;
}

#[tauri::command]
fn parser_curl(value: &str, download_path: &str, output_file: &str) -> response::Response {
    println!("{download_path}{output_file}");
    let full_path = format!("{download_path}/{output_file}");
    let p = path::Path::new(&full_path);
    let download_p = path::Path::new(download_path);

    let mut m = HashMap::new();
    m.insert(build_reqwest::Key::Curl, value);
    let br: build_reqwest::BuildReqwest = m.into();
    if br.url.is_empty() {
        return response::Response {
            code: 400,
            msg: "链接解析失败".into(),
            data: "".into(),
        };
    }

    response::Response {
        code: 200,
        msg: "Success".into(),
        data: "".into(),
    }
}

#[tauri::command]
fn get_user_downloads_path() -> Result<(), String> {
    // if let Some(path) = () {
    //     if let Some(parent) = path.parent() {
    //         return Some(parent.to_string_lossy().to_string());
    //     }
    // }

    // None
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            detect_file,
            parser_curl,
            get_user_downloads_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
