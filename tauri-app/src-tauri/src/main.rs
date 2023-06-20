// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod preload;

use tauri::{command, generate_context, generate_handler, Builder};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    Builder::default()
        .plugin(preload::PreloadPlugin::new())
        .invoke_handler(generate_handler![greet])
        .run(generate_context!())
        .expect("error while running tauri application")
}
