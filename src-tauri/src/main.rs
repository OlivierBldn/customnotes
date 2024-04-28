// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use rusqlite::{params, Connection, Result};
use lazy_static::lazy_static;

use aws_sdk_s3 as s3;
use s3::types::CreateBucketConfiguration;
use s3::types::BucketLocationConstraint;

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

/// Represents a note with an ID, title, and content.
#[derive(Debug, serde::Deserialize, Clone)]
pub struct Note {
    id: Option<i64>,
    title: String,
    content: String,
}

/// Creates a new note with the given title and content.
/// 
/// # Arguments
/// 
/// * `note` - The note to create. It should contain the title and content of the note.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the note is created successfully, or `Err(String)` if an error occurs.
#[tauri::command]
fn create_note(note: Note) -> Result<(), String> {
    let conn = CONNECTION.lock().unwrap();
    conn.execute(
        "INSERT INTO notes (title, content) VALUES (?1, ?2)",
        params![note.title, note.content],
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
/// * `note` - The note to update. It should contain the ID, title, and content of the note.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the note is updated successfully, or `Err(String)` if an error occurs.
#[tauri::command]
fn update_note(note: Note) -> Result<(), String> {
    let conn = CONNECTION.lock().unwrap();
    conn.execute(
        "UPDATE notes SET title = ?1, content = ?2 WHERE id = ?3",
        params![note.title, note.content, note.id],
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

/// Checks if an Amazon S3 bucket exists.
///
/// # Parameters
///
/// * `client` - A reference to the AWS S3 client.
/// * `bucket_name` - The name of the bucket to check.
///
/// # Returns
///
/// Returns a `Result` containing a boolean value:
/// * `Ok(true)` if the bucket exists.
/// * `Ok(false)` if the bucket does not exist.
/// * `Err(s3::Error)` if an error occurs while checking the bucket existence.
///
/// # Errors
///
/// This function will return an error if the AWS SDK encounters an error when checking the bucket existence.
async fn bucket_exists(client: &s3::Client, bucket_name: &str) -> Result<bool, s3::Error> {
    match client.head_bucket().bucket(bucket_name).send().await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Creates a new Amazon S3 bucket.
///
/// # Parameters
///
/// * `s3_client` - A reference to the AWS S3 client.
/// * `bucket_name` - The name of the bucket to create.
///
/// # Operation
///
/// * A connection to the Amazon S3 service is established using the AWS SDK for Rust.
/// * The region for the S3 service is set to "eu-west-3".
/// * A new S3 bucket with the specified `bucket_name` is created in the "eu-west-3" region.
///
/// # Returns
///
/// * If the operation is successful, `Ok(())` is returned.
/// * If the operation fails, `Err(s3::Error)` is returned.
///
/// # Errors
///
/// This function will return an error if the AWS SDK encounters an error when creating the bucket.
#[tauri::command]
async fn create_bucket(s3_client: &s3::Client, bucket_name: &str) -> Result<(), s3::Error> {
    let region_string = s3_client.config().region().unwrap().as_ref().to_string();
    let constraint = BucketLocationConstraint::try_parse(&region_string)
        .unwrap_or_else(|_| panic!("Invalid region: {}", region_string));
    let bucket_config = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();

    s3_client.create_bucket()
        .create_bucket_configuration(bucket_config)
        .bucket(bucket_name)
        .send()
        .await?;

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
fn validate_params(note: Note) -> Result<(), String> {
    if note.title.len() > 100 {
        return Err("Title too long".to_string());
    }

    if note.content.len() > 1000000 {
        return Err("Content too long".to_string());
    }

    Ok(())
}

/// Saves a note to an Amazon S3 bucket.
///
/// # Parameters
///
/// * `note` - The note to save. It should contain the title and content of the note.
///
/// # Operation
///
/// * A connection to the Amazon S3 service is established using the AWS SDK for Rust.
/// * The region for the S3 service is set to "eu-west-3".
/// * The content of the note is converted to bytes and then to a ByteStream.
/// * The title of the note is used as the base name of the file, with ".txt" appended to it.
/// * The file is uploaded to the S3 bucket named "olivier-rust-custom-notes".
/// * The content type of the file is set to "text/plain".
///
/// # Returns
///
/// * If the operation is successful, a `Result` containing a `String` with the message "Object uploaded successfully" is returned.
/// * If the operation fails, a `Result` containing an `Err` with a `String` describing the error is returned.
///
/// # Errors
///
/// This function will return an error if the AWS SDK encounters an error when uploading the file to the S3 bucket.
#[tauri::command]
async fn save_note(note: Note) -> Result<String, String> {

    match validate_params(note.clone()) {
        Ok(_) => {
        },
        Err(e) => {
            println!("Error: {}", e);
            return Err(e);
        }
    }

    let myconfig = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("eu-west-3"))
        .load()
        .await;
    let s3_client = s3::Client::new(&myconfig);
    let input_string = note.content.as_bytes().to_vec();
    let bytestream = s3::primitives::ByteStream::from(input_string);
    let filename = format!("{}.txt", note.title);
    let put_object = s3_client.put_object()
        .bucket("olivier-rust-custom-notes")
        .key(&filename)
        .body(bytestream)
        .content_type("text/plain")
        .send().await;

    match put_object {
        Ok(_) => {
            println!("Object uploaded successfully");
            Ok("Object uploaded successfully".to_string())
        },
        Err(e) => {
            println!("Object upload failed: {:?}", e);
            Err(format!("Object upload failed: {:?}", e))
        },
    }
}

/// Creates a new `Note` object with the provided values.
///
/// # Arguments
///
/// * `note.0` - The ID of the note.
/// * `note.1` - The title of the note.
/// * `note.2` - The content of the note.
///
/// # Returns
///
/// A new `Note` object with the provided values.
#[tauri::command]
async fn send_notes_to_cloud() -> Result<(), String> {
    let notes = read_notes().unwrap();

    // Create a vector to store any errors that occur during the sending process (allows process to continue for others)
    let mut errors = Vec::new();

    for note in notes {
        let note = Note {
            id: Some(note.0),
            title: note.1,
            content: note.2,
        };
        match validate_params(note.clone()) {
            Ok(_) => {
                if let Err(e) = save_note(note).await {
                    println!("Failed to save note: {}", e);
                    errors.push(e);
                }
            },
            Err(e) => {
                println!("Note validation failed for note with id {}: {}", note.id.unwrap(), e);
                errors.push(format!("Note validation failed for note with id {}: {}", note.id.unwrap(), e));
            }
        }
    }

    if !errors.is_empty() {
        return Err(errors.join(", "));
    }

    Ok(())
}

/// The main entry point of the application.
/// 
/// This function initializes the Tauri application and sets up the necessary database connection.
/// It registers the command handlers for creating, reading, updating, and deleting notes.
/// 
/// Executes the Tauri application and runs the event loop.
#[tokio::main]
async fn main() {

    let myconfig = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("eu-west-3"))
        .load()
        .await;
    let s3_client = s3::Client::new(&myconfig);
    let bucket_name = "olivier-rust-custom-notes";

    match bucket_exists(&s3_client, bucket_name).await {
        Ok(bucket_exists) => {
            if !bucket_exists {
                match create_bucket(&s3_client, bucket_name).await {
                    Ok(_) => {
                    },
                    Err(e) => {
                        println!("Failed to create bucket: {}", e);
                    }
                }
            }
        },
        Err(e) => {
            println!("Failed to check if bucket exists: {}", e);
        }
    }

    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        create_note, 
        read_notes, 
        update_note, 
        delete_note, 
        save_note, 
        send_notes_to_cloud
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}