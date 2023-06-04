#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::env;

fn main() {
    for value in env::args() {
        match &*value {
            "--cli" => {
                println!("Hellooo");
                return;
            }
            _ => continue
        }
    }

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
