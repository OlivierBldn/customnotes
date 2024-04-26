// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use rusqlite::{params, Connection, Result};
use lazy_static::lazy_static;

use aws_sdk_s3 as s3;
use s3::types::CreateBucketConfiguration;

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

/// Creates a new Amazon S3 bucket.
///
/// # Operation
///
/// * A connection to the Amazon S3 service is established using the AWS SDK for Rust.
/// * The region for the S3 service is set to "eu-west-3".
/// * A new S3 bucket named "olivier-rust-custom-notes" is created in the "eu-west-3" region.
///
/// # Returns
///
/// * If the operation is successful, a `Result` containing a `String` with the message "Bucket created successfully" is returned.
/// * If the operation fails, a `Result` containing an `Err` with a `String` describing the error is returned.
///
/// # Errors
///
/// This function will return an error if the AWS SDK encounters an error when creating the bucket.
#[tauri::command]
async fn create_bucket() -> Result<String, String> {
    let myconfig = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("eu-west-3"))
        .load()
        .await;
    let s3_client = s3::Client::new(&myconfig);
    let constraint = s3::types::BucketLocationConstraint::from("eu-west-3");
    let bucket_config = CreateBucketConfiguration::builder().location_constraint(constraint).build();
    let bucket_creation = s3_client.create_bucket()
        .create_bucket_configuration(bucket_config)
        .bucket("olivier-rust-custom-notes")
        .send().await;
    match bucket_creation {
        Ok(_) => {
            let head_bucket_output = s3_client.head_bucket().bucket("olivier-rust-custom-notes").send().await;
            match head_bucket_output {
                Ok(_) => {
                    println!("Bucket created successfully");
                    Ok("Bucket created successfully".to_string())
                },
                Err(e) => {
                    println!("Bucket creation seemed successful, but the bucket does not exist: {:?}", e);
                    Err(format!("Bucket creation seemed successful, but the bucket does not exist: {:?}", e))
                },
            }
        },
        Err(e) => {
            println!("Bucket creation failed: {:?}", e);
            Err(format!("Bucket creation failed: {:?}", e))
        },
    }
}

/// Saves a note to an Amazon S3 bucket.
///
/// # Parameters
///
/// * `title`: The title of the note. This is used as the base name of the file in the S3 bucket.
/// * `content`: The content of the note. This is the text that will be saved in the file.
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
async fn save_note(title: String, content: String) -> Result<String, String> {
    let myconfig = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("eu-west-3"))
        .load()
        .await;
    let s3_client = s3::Client::new(&myconfig);
    let input_string = content.as_bytes().to_vec();
    let bytestream = s3::primitives::ByteStream::from(input_string);
    let filename = format!("{}.txt", title);
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

/// The main entry point of the application.
/// 
/// This function initializes the Tauri application and sets up the necessary database connection.
/// It registers the command handlers for creating, reading, updating, and deleting notes.
/// 
/// Executes the Tauri application and runs the event loop.
#[tokio::main]
async fn main() {

    create_bucket().await.unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_note, read_notes, update_note, delete_note, save_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}