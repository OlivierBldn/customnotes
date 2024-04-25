// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Write;

#[tauri::command]
fn save_note(note: String) {
 let mut file = std::fs::File::create("notes.txt").expect("create
failed");
 file.write_all(note.as_bytes()).expect("write failed");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}