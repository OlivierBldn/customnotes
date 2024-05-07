// local_operations.rs

use crate::models::Note;
use std::sync::Mutex;
use rusqlite::{params, Connection, Result};
use lazy_static::lazy_static;
use uuid::Uuid;
use dirs;
use notify_rust::Notification;
use ring::aead::{Aad, Nonce, LessSafeKey, UnboundKey, CHACHA20_POLY1305};
use ring::rand::{SecureRandom, SystemRandom};
use base64::{Engine as _, engine::general_purpose};


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
///   - "nonce" (TEXT): The nonce used for encryption. It can be null.
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
            nonce TEXT,
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

    // Generate a random nonce
    let rng = SystemRandom::new();
    let mut nonce = [0u8; 12];
    rng.fill(&mut nonce).unwrap();
    let nonce = Nonce::assume_unique_for_key(nonce);

    // Convert the nonce to a byte slice and then encode it
    let nonce_str = general_purpose::STANDARD.encode(nonce.as_ref());

    // Generate a random key
    let crypt_key = UnboundKey::new(&CHACHA20_POLY1305, &[0; 32]).unwrap();
    let crypt_key = LessSafeKey::new(crypt_key);

    // Encrypt the content
    let mut in_out = note.content.clone().into_bytes();
    crypt_key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
        .map_err(|_| "Encryption failed")?;
    let encrypted_content = general_purpose::STANDARD.encode(&in_out);

    let conn = CONNECTION.lock().unwrap();
    let now = chrono::Utc::now().timestamp();
    let uuid = Uuid::new_v4().to_string();
    let timestamp = Some(chrono::Utc::now().to_rfc3339());

    conn.execute(
        "INSERT INTO notes (uuid, title, content, nonce, created_at, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![uuid, note.title, encrypted_content, nonce_str, now, timestamp],
    ).map_err(|e| e.to_string())?;

    // Send a desktop notification
    Notification::new()
    .summary("New note created")
    .body(&format!("Note with title '{}' was created.", note.title))
    .show().unwrap();

    Ok(Note {
        id: None,
        uuid: Some(uuid),
        title: note.title,
        content: encrypted_content,
        nonce: Some(nonce_str),
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
pub async fn get_local_note(id: i64) -> Result<Note, anyhow::Error> {
    let conn = CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, uuid, title, content, nonce, created_at, updated_at, timestamp FROM notes WHERE id = ?1")?;
    let mut note_iter = stmt.query_map(params![id], |row| {

        let content_str: String = row.get(3)?;
        let nonce_str: String = row.get(4)?;

        // Decode the content
        let mut content_bytes = general_purpose::STANDARD.decode(&content_str).map_err(|_| rusqlite::Error::QueryReturnedNoRows)?;

        // Decode the nonce
        let nonce_bytes = general_purpose::STANDARD.decode(&nonce_str).map_err(|_| rusqlite::Error::QueryReturnedNoRows)?;
        if nonce_bytes.len() != 12 {
            eprintln!("Nonce has wrong length");
            return Err(rusqlite::Error::InvalidQuery.into());
        }
        let nonce_array: [u8; 12] = nonce_bytes.try_into().unwrap();
        let nonce = Nonce::assume_unique_for_key(nonce_array);

        // Generate the key
        let crypt_key = UnboundKey::new(&CHACHA20_POLY1305, &[0; 32]).unwrap();
        let crypt_key = LessSafeKey::new(crypt_key);

        // Decrypt the content
        let decrypted_content = crypt_key.open_in_place(nonce, Aad::empty(), &mut content_bytes).unwrap();

        // Convert the decrypted content to a string
        let content = String::from_utf8(decrypted_content.to_vec()).map_err(|_| rusqlite::Error::QueryReturnedNoRows)?;

        Ok(Note {
            id: row.get(0)?,
            uuid: row.get(1)?,
            title: row.get(2)?,
            content: content,
            nonce: Some(nonce_str),
            created_at: row.get::<_, i64>(5)?,
            updated_at: row.get::<_, Option<i64>>(6)?,
            timestamp: row.get(7)?,
        })
    })?;

    note_iter.next().transpose()?.ok_or_else(|| anyhow::anyhow!("Note not found"))
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

    // Generate a random nonce
    let rng = SystemRandom::new();
    let mut nonce = [0u8; 12];
    rng.fill(&mut nonce).unwrap();
    let nonce = Nonce::assume_unique_for_key(nonce);

    // Convert the nonce to a byte slice and then encode it
    let nonce_str = general_purpose::STANDARD.encode(nonce.as_ref());

    // Generate a random key
    let crypt_key = UnboundKey::new(&CHACHA20_POLY1305, &[0; 32]).unwrap();
    let crypt_key = LessSafeKey::new(crypt_key);

    // Encrypt the content
    let mut in_out = note.content.clone().into_bytes();
    crypt_key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
        .map_err(|_| "Encryption failed")?;
    let encrypted_content = general_purpose::STANDARD.encode(&in_out);

    let conn = CONNECTION.lock().unwrap();
    let now = chrono::Utc::now().timestamp();
    let timestamp = Some(chrono::Utc::now().to_rfc3339());

    conn.execute(
        "UPDATE notes SET title = ?1, content = ?2, nonce = ?3, updated_at = ?4, timestamp = ?5 WHERE id = ?6",
        params![note.title, encrypted_content, nonce_str, now, timestamp, note.id],
    ).map_err(|e| e.to_string())?;

    // Send a desktop notification
    Notification::new()
    .summary("Local note updated")
    .body(&format!("Note with title '{}' was updated.", note.title))
    .show().unwrap();

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

    // Send a desktop notification
    Notification::new()
    .summary("Local note deleted")
    .body(&format!("Note with id '{}' was deleted.", id))
    .show().unwrap();

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
pub async fn get_local_notes() -> Result<Vec<Note>, String> {
    let conn = CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, uuid, title, content, nonce, created_at, updated_at, timestamp FROM notes").map_err(|e| e.to_string())?;
    let note_iter = stmt.query_map([], |row| {
        let content_str: String = row.get(3)?;
        let nonce_str: String = row.get(4)?;

        // Decode the content
        let mut content_bytes = general_purpose::STANDARD.decode(&content_str).map_err(|_| rusqlite::Error::QueryReturnedNoRows)?;

        // Decode the nonce
        let nonce_bytes = general_purpose::STANDARD.decode(&nonce_str).map_err(|_| rusqlite::Error::QueryReturnedNoRows)?;
        if nonce_bytes.len() != 12 {
            eprintln!("Nonce has wrong length");
            return Err(rusqlite::Error::InvalidQuery.into());
        }
        let nonce_array: [u8; 12] = nonce_bytes.try_into().unwrap();
        let nonce = Nonce::assume_unique_for_key(nonce_array);

        // Generate the key
        let crypt_key = UnboundKey::new(&CHACHA20_POLY1305, &[0; 32]).unwrap();
        let crypt_key = LessSafeKey::new(crypt_key);

        // Decrypt the content
        let decrypted_content = crypt_key.open_in_place(nonce, Aad::empty(), &mut content_bytes).unwrap();

        // Convert the decrypted content to a string
        let content = String::from_utf8(decrypted_content.to_vec()).map_err(|_| rusqlite::Error::QueryReturnedNoRows)?;

        Ok(Note {
            id: row.get(0)?,
            uuid: row.get(1)?,
            title: row.get(2)?,
            content: content,
            nonce: Some(nonce_str),
            created_at: row.get::<_, i64>(5)?,
            updated_at: row.get::<_, Option<i64>>(6)?,
            timestamp: row.get(7)?,
        })
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

    // Send a desktop notification
    Notification::new()
    .summary("Local notes deleted")
    .body(&format!("Your local notes were deleted."))
    .show().unwrap();

        
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


// /// Derives the nonce from the note ID in the local database.
// /// 
// /// # Arguments
// /// 
// /// * `id` - The ID of the note to derive the nonce from.
// /// 
// /// # Returns
// /// 
// /// Returns a `Result` containing the derived nonce as a `String` if it exists, or an `Err` if the nonce is not found or an error occurs.
// ///
// /// # Errors
// ///
// /// This function will return an error if there is an issue with the database connection or if the note with the specified ID does not exist.
// pub async fn derive_nonce_from_id(id: i64) -> Result<String, anyhow::Error> {
//     let conn = CONNECTION.lock().unwrap();
//     let mut stmt = conn.prepare("SELECT nonce FROM notes WHERE id = ?1")?;
//     let mut nonce_iter = stmt.query_map(params![id], |row| {
//         Ok(row.get(0)?)
//     })?;

//     nonce_iter.next().transpose()?.ok_or_else(|| anyhow::anyhow!("Nonce not found"))
// }