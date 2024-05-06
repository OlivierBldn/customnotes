// main.rs

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod s3_operations;
mod local_operations;

use std::str;
use models::Note;
use tantivy::schema::{Schema, TEXT, STORED};
use tantivy::Index;
use tantivy::query::QueryParser;
use tantivy::TantivyDocument;
use tantivy::DocAddress;
use tantivy::Score;
use tantivy::collector::TopDocs;


/// Routes a command to the appropriate operation based on the command string and arguments.
///
/// # Arguments
///
/// * `command` - A string representing the command to be executed.
/// * `args` - A string representing the arguments for the command.
///
/// # Returns
///
/// A `Result` containing either the result of the operation as a string or an error message as a string.
async fn route_command(command: String, args: String) -> Result<String, String> {
    match command.as_str() {
        "create_local_note" => {
            let args_value: serde_json::Value = serde_json::from_str(&args)
                .map_err(|_| "Invalid JSON in args".to_string())?;
            let note_value = args_value.get("note")
                .ok_or("Missing 'note' key in args".to_string())?
                .to_string();
            let note: models::Note = serde_json::from_str(&note_value)
                .map_err(|_| "Invalid note in args".to_string())?;
            match local_operations::create_local_note(note).await {
                Ok(_) => Ok("Success".to_string()),
                Err(e) => Err(e.to_string()),
            }
        },
        "get_local_note" => {
            let args: serde_json::Value = serde_json::from_str(&args).map_err(|_| "Invalid args".to_string())?;
            let id = args["id"].as_i64().ok_or("Invalid id in args".to_string())?;
            match local_operations::get_local_note(id).await {
                Ok(note) => Ok(serde_json::to_string(&note).map_err(|e| e.to_string())?),
                Err(e) => Err(e.to_string()),
            }
        },
        "update_local_note" => {
            let args_value: serde_json::Value = serde_json::from_str(&args)
                .map_err(|_| "Invalid JSON in args".to_string())?;
            let note_value = args_value.get("note")
                .ok_or("Missing 'note' key in args".to_string())?
                .to_string();
            let note: models::Note = serde_json::from_str(&note_value)
                .map_err(|_| "Invalid note in args".to_string())?;
            match local_operations::update_local_note(note).await {
                Ok(note) => Ok(serde_json::to_string(&note).map_err(|e| e.to_string())?),
                Err(e) => Err(e.to_string()),
            }
        },
        "delete_local_note" => {
            let args_value: serde_json::Value = serde_json::from_str(&args)
                .map_err(|_| "Invalid JSON in args".to_string())?;
            let id_value = args_value.get("id")
                .ok_or("Missing 'id' key in args".to_string())?
                .to_string();
            let id: i64 = id_value.parse().map_err(|_| "Invalid id in args".to_string())?;
            match local_operations::delete_local_note(id) {
                Ok(_) => Ok("Success".to_string()),
                Err(e) => Err(e.to_string()),
            }
        },
        "get_local_notes" => {
            match local_operations::get_local_notes().await {
                Ok(notes) => Ok(serde_json::to_string(&notes).unwrap()),
                Err(e) => Err(e.to_string()),
            }
        },
        "create_bucket" => {
            let args_value: serde_json::Value = serde_json::from_str(&args)
                .map_err(|_| "Invalid JSON in args".to_string())?;
            let bucket_name = args_value.get("bucket_name")
                .ok_or("Missing 'bucket_name' key in args".to_string())?
                .to_string();
            match s3_operations::create_bucket(&bucket_name).await {
                Ok(_) => Ok("Success".to_string()),
                Err(e) => Err(e.to_string()),
            }
        },
        "fetch_buckets" => {
            let buckets = s3_operations::fetch_buckets().await.map_err(|e| e.to_string())?;
            Ok(serde_json::to_string(&buckets).map_err(|e| e.to_string())?)
        },
        "delete_bucket" => {
            let args_value: serde_json::Value = serde_json::from_str(&args)
                .map_err(|_| "Invalid JSON in args".to_string())?;
            let bucket_name = args_value.get("bucket_name")
                .ok_or("Missing 'bucket_name' key in args".to_string())?
                .to_string();
            match s3_operations::delete_bucket(&bucket_name).await {
                Ok(_) => Ok("Success".to_string()),
                Err(e) => Err(e.to_string()),
            }
        },
        "delete_all_local_notes" => {
           match local_operations::delete_all_local_notes().await {
                Ok(_) => Ok("Success".to_string()),
                Err(e) => Err(e.to_string()),
            }
        },
        "upload_note_to_bucket" => {
            let args_value: serde_json::Value = serde_json::from_str(&args)
                .map_err(|_| "Invalid JSON in args".to_string())?;
            let args_value = args_value.as_object()
                .ok_or("args should be a JSON object".to_string())?;
            let bucket_name = args_value.get("bucket_name")
                .ok_or("Missing 'bucket_name' key in args".to_string())?
                .as_str()
                .ok_or("bucket_name should be a string".to_string())?;
            let note_value = args_value.get("note")
                .ok_or("Missing 'note' key in args".to_string())?;
            let note: models::Note = serde_json::from_value(note_value.clone())
                .map_err(|_| "Invalid note in args".to_string())?;
            match s3_operations::upload_note_to_bucket(bucket_name, note).await {
                Ok(_) => Ok("Success".to_string()),
                Err(e) => Err(e.to_string()),
            }
        },
        "fetch_bucket_note" => {
            let args_value: serde_json::Value = serde_json::from_str(&args)
                .map_err(|_| "Invalid JSON in args".to_string())?;
            let args_value = args_value.as_object()
                .ok_or("args should be a JSON object".to_string())?;
            let bucket_name = args_value.get("bucket_name")
                .ok_or("Missing 'bucket_name' key in args".to_string())?
                .as_str()
                .ok_or("bucket_name should be a string".to_string())?;
            let uuid = args_value.get("uuid")
                .ok_or("Missing 'uuid' key in args".to_string())?
                .as_str()
                .ok_or("uuid should be a string".to_string())?;
            match s3_operations::fetch_bucket_note(bucket_name, uuid).await {
                Ok(note) => Ok(serde_json::to_string(&note).map_err(|e| e.to_string())?),
                Err(e) => Err(e.to_string()),
            }
        },
        "update_bucket_note" => {
            let args_value: serde_json::Value = serde_json::from_str(&args)
                .map_err(|_| "Invalid JSON in args".to_string())?;
            let args_value = args_value.as_object()
                .ok_or("args should be a JSON object".to_string())?;
            let bucket_name = args_value.get("bucket_name")
                .ok_or("Missing 'bucket_name' key in args".to_string())?
                .as_str()
                .ok_or("bucket_name should be a string".to_string())?;
            let note_value = args_value.get("note")
                .ok_or("Missing 'note' key in args".to_string())?;
            let note: models::Note = serde_json::from_value(note_value.clone())
                .map_err(|_| "Invalid note in args".to_string())?;
            match s3_operations::update_bucket_note(bucket_name, note).await {
                Ok(_) => Ok("Success".to_string()),
                Err(e) => Err(e.to_string()),
            }
        },
        "delete_bucket_note" => {
            let args_value: serde_json::Value = serde_json::from_str(&args)
                .map_err(|_| "Invalid JSON in args".to_string())?;
            let args_value = args_value.as_object()
                .ok_or("args should be a JSON object".to_string())?;
            let bucket_name = args_value.get("bucket_name")
                .ok_or("Missing 'bucket_name' key in args".to_string())?
                .as_str()
                .ok_or("bucket_name should be a string".to_string())?;
            let uuid = args_value.get("uuid")
                .ok_or("Missing 'uuid' key in args".to_string())?
                .as_str()
                .ok_or("uuid should be a string".to_string())?;
            match s3_operations::delete_bucket_note(bucket_name, uuid).await {
                Ok(_) => Ok("Success".to_string()),
                Err(e) => Err(e.to_string()),
            }
        },
        "fetch_bucket_notes" => {
            let args_value: serde_json::Value = serde_json::from_str(&args)
                .map_err(|_| "Invalid JSON in args".to_string())?;
            let bucket_name = args_value.get("bucket_name")
                .ok_or("Missing 'bucket_name' key in args".to_string())?
                .to_string();
            match s3_operations::fetch_bucket_notes(&bucket_name).await {
                Ok(notes) => Ok(serde_json::to_string(&notes).map_err(|e| e.to_string())?),
                Err(e) => Err(e.to_string()),
            }
        },
        "delete_bucket_notes" => {
            let args_value: serde_json::Value = serde_json::from_str(&args)
                .map_err(|_| "Invalid JSON in args".to_string())?;
            let bucket_name = args_value.get("bucket_name")
                .ok_or("Missing 'bucket_name' key in args".to_string())?
                .to_string();
            match s3_operations::delete_bucket_notes(&bucket_name).await {
                Ok(_) => Ok("Success".to_string()),
                Err(e) => Err(e.to_string()),
            }
        },
        "search_in_notes" => {
            let args_value: serde_json::Value = serde_json::from_str(&args)
                .map_err(|_| "Invalid JSON in args".to_string())?;
            let query = args_value.get("query")
                .ok_or("Missing 'query' key in args".to_string())?
                .to_string();
            let local = args_value.get("local")
                .ok_or("Missing 'local' key in args".to_string())?
                .as_bool()
                .ok_or("'local' key in args is not a boolean".to_string())?;
            let bucket_name = args_value.get("bucket_name")
                .map(|v| v.to_string());
            let bucket_name_option = if let Some(name) = &bucket_name {
                if name.is_empty() {
                    None
                } else {
                    Some(name.as_str())
                }
            } else {
                None
            };
            match search_in_notes(&query, local, bucket_name_option).await {
                Ok(notes) => Ok(serde_json::to_string(&notes).map_err(|e| e.to_string())?),
                Err(e) => Err(e.to_string()),
            }
        },
        _ => Err("Unknown command".to_string()),
    }
}

/// Routes a command and its arguments to the appropriate function and returns the result.
///
/// # Arguments
///
/// * `command` - A string representing the command to be executed.
/// * `args` - A string representing the arguments for the command.
///
/// # Returns
///
/// A `Result` containing either the success message as a `String` or an error message as a `String`.
#[tauri::command]
async fn execute_command(command: String, args: serde_json::Value) -> Result<String, String> {
    route_command(command, args.to_string()).await
}


    /// Retrieves notes based on the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `local` - A boolean indicating whether to fetch local notes or not.
    /// * `bucket_name` - An optional string representing the name of the bucket.
    ///
    /// # Returns
    ///
    /// A vector of tuples containing the retrieved notes. Each tuple consists of the following elements:
    /// * An integer representing the note's ID.
    /// * A string representing the note's UUID.
    /// * A string representing the note's title.
    /// * A string representing the note's content.
    /// * An integer representing the note's status.
    /// * An optional integer representing the note's last modified timestamp.
    /// * An optional string representing the note's timestamp.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the following conditions are met:
    /// * `local` is `true` and there was an error retrieving local notes.
    /// * `local` is `false` and `bucket_name` is not provided.
    /// * `local` is `false` and there was an error fetching bucket notes.
pub async fn search_in_notes(query_str: &str, local: bool, bucket_name: Option<&str>) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    // Define the schema for the index
    let mut schema_builder = Schema::builder();
    let title_field = schema_builder.add_text_field("title", TEXT | STORED);
    let content_field = schema_builder.add_text_field("content", TEXT | STORED);
    let id_field = schema_builder.add_i64_field("id", STORED);
    let uuid_field = schema_builder.add_text_field("uuid", TEXT | STORED);
    let created_at_field = schema_builder.add_i64_field("created_at", STORED);
    let updated_at_field = schema_builder.add_i64_field("updated_at", STORED);
    let timestamp_field = schema_builder.add_text_field("timestamp", TEXT | STORED);
    let schema = schema_builder.build();

    // Create a new index
    let index = Index::create_in_ram(schema.clone());

    // Get the index writer
    let mut index_writer = index.writer(100_000_000)?;

    // Get the notes
    let notes = if local {
        local_operations::get_local_notes().await?
    } else {
        let bucket_name = bucket_name
            .map(|name| name.trim_matches('"'))
            .ok_or("Bucket name is required when local is false")?;
        let bucket_notes = s3_operations::fetch_bucket_notes(bucket_name).await?;
        bucket_notes.into_iter().map(|(title, last_modified, metadata, content)| {
            let (uuid, timestamp) = metadata.map_or((String::new(), String::new()), |map| {
                let uuid = map.get("uuid").cloned().unwrap_or_else(String::new);
                let timestamp = map.get("timestamp").cloned().unwrap_or_else(String::new);
                (uuid, timestamp)
            });
            (0, uuid, title, content, 0, last_modified.map(|lm| lm.parse::<i64>().unwrap_or(0)), Some(timestamp))
        }).collect::<Vec<_>>()
    };

    // Index the notes
    for note in &notes {
        let mut doc = TantivyDocument::new();
        doc.add_text(title_field, &note.2);
        doc.add_text(content_field, &note.3);
        doc.add_i64(id_field, note.0);
        doc.add_text(uuid_field, &note.1);
        doc.add_i64(created_at_field, note.4);
        if let Some(updated_at) = note.5 {
            doc.add_i64(updated_at_field, updated_at);
        }
        if let Some(timestamp) = &note.6 {
            doc.add_text(timestamp_field, timestamp);
        }
        let _ = index_writer.add_document(doc);
    }

    // Commit the documents to the index
    index_writer.commit()?;

    // Create a reader and a searcher
    let reader = index.reader()?;
    let searcher = reader.searcher();

    // Create a query parser for the content field
    let query_parser = QueryParser::for_index(&index, vec![content_field]);

    // Parse the query
    let query = query_parser.parse_query(query_str)?;

    // Perform the search
    let top_docs: Vec<(Score, DocAddress)> = searcher.search(&query, &TopDocs::with_limit(10))?;

    // Retrieve the actual content of the documents
    let mut matching_notes = Vec::new();
    for (_score, doc_address) in top_docs {
        let retrieved_doc: tantivy::TantivyDocument = searcher.doc(doc_address)?;
        let title = retrieved_doc.get_first(title_field).and_then(|v| match v {
            tantivy::schema::OwnedValue::Str(t) => Some(t.to_string()),
            _ => None,
        }).unwrap_or_else(|| "".to_string());
        let content = retrieved_doc.get_first(content_field).and_then(|v| match v {
            tantivy::schema::OwnedValue::Str(t) => Some(t.to_string()),
            _ => None,
        }).unwrap_or_else(|| "".to_string());
        let schema = index.schema();
        let id_field = schema.get_field("id").unwrap();
        let uuid_field = schema.get_field("uuid").unwrap();
        let created_at_field = schema.get_field("created_at").unwrap();
        let updated_at_field = schema.get_field("updated_at").unwrap();
        let timestamp_field = schema.get_field("timestamp").unwrap();

        let id = retrieved_doc.get_first(id_field).and_then(|v| match v {
            tantivy::schema::OwnedValue::I64(t) => Some(*t),
            _ => None,
        });

        let uuid = retrieved_doc.get_first(uuid_field).and_then(|v| match v {
            tantivy::schema::OwnedValue::Str(t) => Some(t.to_string()),
            _ => None,
        });
        let created_at = retrieved_doc.get_first(created_at_field).and_then(|v| match v {
            tantivy::schema::OwnedValue::I64(t) => Some(*t),
            _ => None,
        }).unwrap_or_else(|| 0);

        let updated_at = retrieved_doc.get_first(updated_at_field).and_then(|v| match v {
            tantivy::schema::OwnedValue::I64(t) => Some(*t),
            _ => None,
        });
        
        let timestamp = retrieved_doc.get_first(timestamp_field).and_then(|v| match v {
            tantivy::schema::OwnedValue::Str(t) => Some(t.to_string()),
            _ => None,
        });
    
        matching_notes.push(Note {
            id,
            uuid,
            title,
            content,
            created_at,
            updated_at,
            timestamp,
        });
    }

    Ok(matching_notes)
}

/// The main entry point of the application.
/// 
/// This function initializes the Tauri application and sets up the necessary database connection.
/// It registers the command handlers for creating, reading, updating, and deleting notes.
/// 
/// Executes the Tauri application and runs the event loop.
#[tokio::main]
async fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        execute_command,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_transform_bucket_notes() {

        let mut map1 = HashMap::new();
        map1.insert("timestamp".to_string(), "2024-05-05T02:51:00.732617662+00:00".to_string());
        map1.insert("uuid".to_string(), "8d5572eb-b4a0-4697-b551-fff4de57f17e".to_string());

        let mut map2 = HashMap::new();
        map2.insert("timestamp".to_string(), "2024-05-05T02:19:16.798625250+00:00".to_string());
        map2.insert("uuid".to_string(), "da1417b4-17b9-47a6-84fe-ea049d223cc3".to_string());

        // Arrange
        let bucket_notes = vec![
            ("title1.txt".to_string(), Some("2024-05-05T02:51:01Z".to_string()), Some(map1), "content1".to_string()),
            ("title2.txt".to_string(), Some("2024-05-05T02:19:17Z".to_string()), Some(map2), "content2".to_string()),
        ];
        let expected_output = vec![
            (0, "title1.txt".to_string(), "content1".to_string(), String::new(), 0, None::<String>, None::<String>),
            (0, "title2.txt".to_string(), "content2".to_string(), String::new(), 0, None::<String>, None::<String>),
        ];

        // Act
        let output: Vec<_> = bucket_notes.into_iter().map(|(title, _, _, content)| {
            (0, title, content, String::new(), 0, None, None)
        }).collect();

        // Assert
        assert_eq!(output, expected_output);
    }
}