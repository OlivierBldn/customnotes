// local_operations.rs

use crate::models::Note;
use std::sync::Mutex;
use rusqlite::{params, Connection, Result};
use lazy_static::lazy_static;
use uuid::Uuid;
use dirs;

lazy_static! {
 /// Establishes a connection to a SQLite database and creates a table for notes if it doesn't exist.
///
/// # Initialization
///
/// * The connection is established to a SQLite database named "notes.db" located in the user's home directory. If the file does not exist, it will be created.
/// * A SQL statement is executed to create a new table named "notes" in the database if it does not already exist.
/// The table has the following columns:
///   - "id" (INTEGER): The primary key of the table.
///   - "uuid" (TEXT): The UUID of the note.
///   - "title" (TEXT): The title of the note. It cannot be null.
///   - "content" (TEXT): The content of the note. It cannot be null.
///   - "created_at" (INTEGER): The timestamp when the note was created.
///   - "updated_at" (INTEGER): The timestamp when the note was last updated. It can be null.
///   - "timestamp" (TEXT): The timestamp of the note in RFC 3339 format. It can be null.
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
        let mut db_path = dirs::home_dir().unwrap();
        db_path.push("notes.db");
        let conn = Connection::open(db_path).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            uuid TEXT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER,
            timestamp TEXT
            )",
            [],
        ).unwrap();
        Mutex::new(conn)
    };
}


    /// Creates a new note with the given title and content in the local database.
    /// 
    /// # Arguments
    /// 
    /// * `note` - The note to create. It should contain the title and content of the note.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(Note)` if the note is created successfully, or `Err(String)` if an error occurs.
    ///
    /// # Errors
    ///
    /// This function will return an error if the title is too long (more than 100 characters) or if the content is too long (more than 1,000,000 characters).
pub async fn create_local_note(note: Note) -> Result<Note, String> {

    match validate_params(note.clone()) {
        Ok(_) => {
        },
        Err(e) => {
            println!("Error: {}", e);
            return Err(e);
        }
    }

    let conn = CONNECTION.lock().unwrap();
    let now = chrono::Utc::now().timestamp();
    let uuid = Uuid::new_v4().to_string();
    let timestamp = Some(chrono::Utc::now().to_rfc3339());
    conn.execute(
        "INSERT INTO notes (uuid, title, content, created_at, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![uuid, note.title, note.content, now, timestamp],
    ).map_err(|e| e.to_string())?;
    Ok(Note {
        id: None,
        uuid: Some(uuid),
        title: note.title,
        content: note.content,
        created_at: now,
        updated_at: None,
        timestamp: timestamp,
    })
}



/// Retrieves a note from the local database based on its ID.
/// 
/// # Arguments
/// 
/// * `id` - The ID of the note to retrieve.
/// 
/// # Returns
/// 
/// Returns `Ok(Note)` if the note is found, or `Err(String)` if the note is not found or an error occurs.
///
/// # Errors
///
/// This function will return an error if there is an issue with the database connection or if the note with the specified ID does not exist.
pub async fn get_local_note(id: i64) -> Result<Note, String> {
    let conn = CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, uuid, title, content, created_at, updated_at, timestamp FROM notes WHERE id = ?1").map_err(|e| e.to_string())?;
    let mut note_iter = stmt.query_map(params![id], |row| {
        Ok(Note {
            id: row.get(0)?,
            uuid: row.get(1)?,
            title: row.get(2)?,
            content: row.get(3)?,
            created_at: row.get::<_, i64>(4)?,
            updated_at: row.get::<_, Option<i64>>(5)?,
            timestamp: row.get(6)?,
        })
    }).map_err(|e| e.to_string())?;

    note_iter.next().transpose().map_err(|e| e.to_string())?.ok_or("Note not found".to_string())
}


/// Updates the note with the given ID, title, and content in the local database.
/// 
/// # Arguments
/// 
/// * `note` - The note to update. It should contain the ID, title, and content of the note.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the note is updated successfully, or `Err(String)` if an error occurs.
///
/// # Errors
///
/// This function will return an error if the title is too long (more than 100 characters) or if the content is too long (more than 1,000,000 characters) or if the note with the specified ID does not exist.
pub async fn update_local_note(note: Note) -> Result<(), String> {

    match validate_params(note.clone()) {
        Ok(_) => {
        },
        Err(e) => {
            println!("Error: {}", e);
            return Err(e);
        }
    }

    let conn = CONNECTION.lock().unwrap();
    let now = chrono::Utc::now().timestamp();
    let timestamp = Some(chrono::Utc::now().to_rfc3339());

    conn.execute(
        "UPDATE notes SET title = ?1, content = ?2, updated_at = ?3, timestamp = ?4 WHERE id = ?5",
        params![note.title, note.content, now, timestamp, note.id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}


/// Deletes the note with the given ID from the local database.
/// 
/// # Arguments
/// 
/// * `id` - The ID of the note to delete.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the note is deleted successfully, or `Err(String)` if an error occurs.
///
/// # Errors
///
/// This function will return an error if there is an issue with the database connection or if the note with the specified ID does not exist.
pub fn delete_local_note(id: i64) -> Result<(), String> {
    let conn = CONNECTION.lock().unwrap();
    conn.execute(
        "DELETE FROM notes WHERE id = ?1",
        params![id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}


/// Retrieves all notes from the local database.
/// 
/// # Returns
/// 
/// Returns a vector of tuples containing the ID, UUID, title, content, created_at, updated_at, and timestamp of each note.
/// 
/// # Errors
///
/// This function will return an error if there is an issue with the database connection.
pub async fn get_local_notes() -> Result<Vec<(i64, String, String, String, i64, Option<i64>, Option<String>)>, String> {
    let conn = CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, uuid, title, content, created_at, updated_at, timestamp FROM notes").map_err(|e| e.to_string())?;
    let note_iter = stmt.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get::<_, i64>(4)?,
            row.get::<_, Option<i64>>(5)?,
            row.get(6)?,
        ))
    }).map_err(|e| e.to_string())?;
    let notes: Result<Vec<_>, _> = note_iter.collect();
    notes.map_err(|e| e.to_string())
}


/// Deletes all notes from the local database.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if all notes are deleted successfully, or `Err(String)` if an error occurs.
///
/// # Errors
///
/// This function will return an error if there is an issue with the database connection.
pub async fn delete_all_local_notes() -> Result<(), String> {
    let conn = CONNECTION.lock().unwrap();
    conn.execute(
        "DELETE FROM notes",
        [],
    ).map_err(|e| e.to_string())?;
    Ok(())
}


/// * `note` - The note to validate. It should contain the title and content of the note.
///
/// # Returns
///
/// Returns `Ok(())` if the parameters are valid, or `Err(String)` if an error occurs.
///
/// # Errors
///
/// This function will return an error if the title is too long (more than 100 characters) or if the content is too long (more than 1,000,000 characters).
pub fn validate_params(note: Note) -> Result<(), String> {
    if note.title.len() > 100 {
        return Err("Title too long".to_string());
    }

    if note.content.len() > 1000000 {
        return Err("Content too long".to_string());
    }

    Ok(())
}