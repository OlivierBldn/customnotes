// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use rusqlite::{params, Connection, Result};
use lazy_static::lazy_static;


lazy_static! {
/// Establishes a connection to a SQLite database and creates a table for notes if it doesn't exist.
///
/// # Initialization
///
/// * The connection is established to a SQLite database named "notes.db". If the file does not exist, it will be created.
/// * A SQL statement is executed to create a new table named "notes" in the database if it does not already exist.
/// The table has three columns: "id", "title", and "content".
/// The "id" column is an integer and is the primary key of the table. The "title" and "content" columns are text and cannot be null.
///
/// # Usage
///
/// This static reference to the database connection is used throughout the application to interact with the database.
/// It is wrapped in a Mutex for thread safety, allowing it to be shared across multiple threads.
///
/// # Panics
///
/// The program will panic and exit if an error occurs when opening the connection or executing the SQL statement.
    static ref CONNECTION: Mutex<Connection> = {
        let conn = Connection::open("../notes.db").unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL
            )",
            [],
        ).unwrap();
        Mutex::new(conn)
    };
}

/// Creates a new note with the given title and content.
/// 
/// # Arguments
/// 
/// * `title` - The title of the note.
/// * `content` - The content of the note.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the note is created successfully, or `Err(String)` if an error occurs.
#[tauri::command]
fn create_note(title: String, content: String) -> Result<(), String> {
    let conn = CONNECTION.lock().unwrap();
    conn.execute(
        "INSERT INTO notes (title, content) VALUES (?1, ?2)",
        params![title, content],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

/// Reads all the notes from the database.
/// 
/// # Returns
/// 
/// Returns a vector of tuples containing the ID, title, and content of each note.
/// Returns `Ok(Vec<(i64, String, String)>)` if the notes are read successfully, or `Err(String)` if an error occurs.
#[tauri::command]
fn read_notes() -> Result<Vec<(i64, String, String)>, String> {
    let conn = CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, title, content FROM notes").map_err(|e| e.to_string())?;
    let note_iter = stmt.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
        ))
    }).map_err(|e| e.to_string())?;
    let notes: Result<Vec<_>, _> = note_iter.collect();
    notes.map_err(|e| e.to_string())
}

/// Updates the note with the given ID, title, and content.
/// 
/// # Arguments
/// 
/// * `id` - The ID of the note to update.
/// * `title` - The new title of the note.
/// * `content` - The new content of the note.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the note is updated successfully, or `Err(String)` if an error occurs.
#[tauri::command]
fn update_note(id: i64, title: String, content: String) -> Result<(), String> {
    let conn = CONNECTION.lock().unwrap();
    conn.execute(
        "UPDATE notes SET title = ?1, content = ?2 WHERE id = ?3",
        params![title, content, id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

/// Deletes the note with the given ID.
/// 
/// # Arguments
/// 
/// * `id` - The ID of the note to delete.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the note is deleted successfully, or `Err(String)` if an error occurs.
#[tauri::command]
fn delete_note(id: i64) -> Result<(), String> {
    let conn = CONNECTION.lock().unwrap();
    conn.execute(
        "DELETE FROM notes WHERE id = ?1",
        params![id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}


/// The main entry point of the application.
/// 
/// This function initializes the Tauri application and sets up the necessary database connection.
/// It registers the command handlers for creating, reading, updating, and deleting notes.
/// 
/// Executes the Tauri application and runs the event loop.
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_note, read_notes, update_note, delete_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}