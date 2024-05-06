// s3_operations.rs

use aws_sdk_s3 as s3;
use rusqlite::Result;
use s3::types::{ BucketLocationConstraint, CreateBucketConfiguration, Tag, Tagging };
use crate::{ local_operations, models::Note, models::BucketError };
use std::collections::HashMap;


/// Creates a new Amazon S3 bucket.
///
/// # Parameters
///
/// * `bucket_name` - The name of the bucket to create.
///
/// # Operation
///
/// * A connection to the Amazon S3 service is established using the AWS SDK for Rust.
/// * The region for the S3 service is set to "eu-west-3".
/// * A new S3 bucket with the specified `bucket_name` is created in the "eu-west-3" region.
/// * If the bucket already exists, an error of type `BucketError::BucketAlreadyExists` is returned.
/// * After creating the bucket, a tag with key "App" and value "RustCustomNotes" is added to the bucket.
///
/// # Returns
///
/// * If the operation is successful, `Ok(())` is returned.
/// * If the operation fails, an error of type `BucketError` is returned.
///
/// # Errors
///
/// This function will return an error if the AWS SDK encounters an error when creating the bucket or adding the tag.
pub async fn create_bucket(bucket_name: &str) -> Result<(), BucketError> {
    // Trim any surrounding double quotes from the bucket name
    let bucket_name = bucket_name.trim_matches('"');

    // Check if the bucket already exists
    if bucket_exists(bucket_name).await? {
        return Err(BucketError::BucketAlreadyExists);
    }

    // Create a new S3 client with the specified region
    let myconfig = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("eu-west-3"))
        .load()
        .await;
    let s3_client = s3::Client::new(&myconfig);

    // Get the region string from the client's configuration
    let region_string = s3_client.config().region().unwrap().as_ref().to_string();

    // Parse the region string into a BucketLocationConstraint
    let constraint = BucketLocationConstraint::try_parse(&region_string)
        .unwrap_or_else(|_| panic!("Invalid region: {}", region_string));

    // Build the bucket configuration with the location constraint
    let bucket_config = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();

    // Send the create bucket request
    let create_bucket_result = s3_client.create_bucket()
        .create_bucket_configuration(bucket_config)
        .bucket(bucket_name)
        .send()
        .await;

    // Handle the create bucket result
    match create_bucket_result {
        Ok(_) => (),
        Err(err) => return Err(BucketError::S3Error(Box::new(err))),
    }

    // Build the tag with key "App" and value "RustCustomNotes"
    let tag = Tag::builder()
        .key("App")
        .value("RustCustomNotes")
        .build()
        .map_err(|_| BucketError::TaggingError)?;

    // Build the tagging configuration with the tag
    let tagging_config = Tagging::builder()
        .tag_set(tag)
        .build()
        .map_err(|_| BucketError::TaggingError)?;

    // Send the put bucket tagging request
    let put_tagging_result = s3_client.put_bucket_tagging()
        .bucket(bucket_name)
        .tagging(tagging_config)
        .send()
        .await;

    // Handle the put bucket tagging result
    match put_tagging_result {
        Ok(_) => (),
        Err(err) => return Err(BucketError::S3Error(Box::new(err))),
    }

    Ok(())
}


/// Fetches the list of buckets that have the "App" tag set to "RustCustomNotes".
///
/// # Operation
///
/// * A connection to the Amazon S3 service is established using the AWS SDK for Rust.
/// * The region for the S3 service is set to "eu-west-3".
/// * The list of buckets is retrieved using the `list_buckets` API.
/// * For each bucket, the `get_bucket_tagging` API is called to retrieve the tags associated with the bucket.
/// * If the bucket has a tag with key "App" and value "RustCustomNotes", it is added to the list of buckets with the tag.
///
/// # Returns
///
/// * If the operation is successful, a `Result` containing a `Vec<String>` with the names of the buckets is returned.
/// * If the operation fails, a `Result` containing an `Err` with a `s3::Error` describing the error is returned.
///
/// # Errors
///
/// This function will return an error if the AWS SDK encounters an error when fetching the list of buckets or retrieving the tags.
pub async fn fetch_buckets() -> Result<Vec<String>, s3::Error> {
    // Establish a connection to the Amazon S3 service
    let myconfig = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("eu-west-3"))
        .load()
        .await;
    let s3_client = s3::Client::new(&myconfig);

    let mut buckets_with_tag = Vec::new();

    // Retrieve the list of buckets
    let list_buckets_output = s3_client.list_buckets().send().await?;

    for bucket in list_buckets_output.buckets.unwrap_or_default() {
        let bucket_name = bucket.name.unwrap_or_default();

        // Retrieve the tags associated with the bucket
        let get_bucket_tagging_output = s3_client.get_bucket_tagging()
            .bucket(&bucket_name)
            .send()
            .await;

        match get_bucket_tagging_output {
            Ok(output) => {
                // Check if the bucket has the "App" tag set to "RustCustomNotes"
                for tag_set in output.tag_set {
                    if tag_set.key == "App" && tag_set.value == "RustCustomNotes" {
                        buckets_with_tag.push(bucket_name);
                        break;
                    }
                }
            }
            Err(_) => continue,
        }
    }

    Ok(buckets_with_tag)
}


/// Checks if an Amazon S3 bucket exists.
///
/// # Parameters
///
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
pub async fn bucket_exists(bucket_name: &str) -> Result<bool, s3::Error> {
    // Create AWS configuration with the desired region
    let myconfig = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("eu-west-3"))
        .load()
        .await;

    // Create an S3 client using the AWS configuration
    let s3_client = s3::Client::new(&myconfig);

    // Send a HEAD request to check if the bucket exists
    match s3_client.head_bucket().bucket(bucket_name).send().await {
        Ok(_) => Ok(true), // Bucket exists
        Err(_) => Ok(false), // Bucket does not exist
    }
}


/// Deletes an Amazon S3 bucket.
///
/// # Parameters
///
/// * `bucket_name` - The name of the bucket to delete.
///
/// # Operation
///
/// * A connection to the Amazon S3 service is established using the AWS SDK for Rust.
/// * The region for the S3 service is set to "eu-west-3".
/// * The specified bucket is deleted from the S3 service.
///
/// # Returns
///
/// * If the operation is successful, `Ok(())` is returned.
/// * If the operation fails, an error of type `s3::Error` is returned.
///
/// # Errors
///
/// This function will return an error if the AWS SDK encounters an error when deleting the bucket.
pub async fn delete_bucket(bucket_name: &str) -> Result<(), s3::Error> {
    // Trim any surrounding quotes from the bucket name
    let bucket_name = bucket_name.trim_matches('"');

    // Configure the AWS SDK with the desired region
    let myconfig = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("eu-west-3"))
        .load()
        .await;

    // Create a new S3 client
    let s3_client = s3::Client::new(&myconfig);

    // Send a request to delete the specified bucket
    s3_client.delete_bucket().bucket(bucket_name).send().await?;

    Ok(())
}


/// Uploads a note to an Amazon S3 bucket.
///
/// # Parameters
///
/// * `bucket_name` - The name of the bucket to upload the note to.
/// * `note` - The note to upload. It should contain the title and content of the note.
///
/// # Operation
///
/// * A connection to the Amazon S3 service is established using the AWS SDK for Rust.
/// * The region for the S3 service is set to "eu-west-3".
/// * The content of the note is converted to bytes and then to a ByteStream.
/// * The title of the note is used as the base name of the file, with ".txt" appended to it.
/// * The file is uploaded to the specified S3 bucket.
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
pub async fn upload_note_to_bucket(bucket_name: &str, note: Note) -> Result<String, String> {
    // Validate the parameters of the note
    match local_operations::validate_params(note.clone()) {
        Ok(_) => {
            // Parameters are valid, continue with the upload
        },
        Err(e) => {
            // Parameters are invalid, return the error
            return Err(e);
        }
    }

    // Configure the AWS SDK with the desired region
    let myconfig = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("eu-west-3"))
        .load()
        .await;
    let s3_client = s3::Client::new(&myconfig);

    // Convert the content of the note to bytes and create a ByteStream
    let input_string = note.content.as_bytes().to_vec();
    let bytestream = s3::primitives::ByteStream::from(input_string);

    // Generate the filename for the note by appending ".txt" to the title
    let filename = format!("{}.txt", note.title);

    // Get the UUID of the note from the local storage
    let uuid = local_operations::get_local_note(note.id.unwrap()).await?.uuid.unwrap();

    // Get the current timestamp
    let timestamp = chrono::Utc::now().to_rfc3339();

    // Get the created_at and updated_at timestamps as strings
    let created_at = note.created_at.to_string();
    let updated_at = note.updated_at.unwrap_or(0).to_string();

    // Upload the note to the S3 bucket with the specified metadata
    let put_object = s3_client.put_object()
        .bucket(bucket_name)
        .key(&filename)
        .metadata("uuid", &uuid)
        .metadata("timestamp", &timestamp)
        .metadata("created_at", &created_at)
        .metadata("updated_at", &updated_at)
        .body(bytestream)
        .content_type("text/plain")
        .send().await;

    // Check if the upload was successful or return an error
    match put_object {
        Ok(_) => {
            Ok("Object uploaded successfully".to_string())
        },
        Err(e) => {
            Err(format!("Object upload failed: {:?}", e))
        },
    }
}


/// Fetches a note from an Amazon S3 bucket based on its UUID.
///
/// # Parameters
///
/// * `bucket` - The name of the bucket to fetch the note from.
/// * `uuid` - The UUID of the note to fetch.
///
/// # Returns
///
/// Returns a `Result` containing a `Note` if the note is found in the bucket.
/// If the note is not found, an `Err` with a `Box<dyn std::error::Error>` is returned.
///
/// # Errors
///
/// This function will return an error if the AWS SDK encounters an error when fetching the note or if the note is not found.
pub async fn fetch_bucket_note(bucket: &str, uuid: &str) -> Result<Note, Box<dyn std::error::Error>> {
    // Create AWS configuration with the specified region
    let myconfig = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("eu-west-3"))
        .load()
        .await;

    // Create an S3 client using the AWS configuration
    let client = s3::Client::new(&myconfig);

    // List objects in the bucket
    let list_objects_output = client.list_objects_v2()
        .bucket(bucket)
        .send()
        .await?;

    // Iterate over the objects in the bucket
    for object in list_objects_output.contents.unwrap_or_default() {
        let key = object.key.unwrap_or_default();

        // Retrieve the metadata of the object
        let head_object_output = client.head_object()
            .bucket(bucket)
            .key(&key)
            .send()
            .await?;

        // Check if the object has the specified UUID in its metadata
        if let Some(metadata) = head_object_output.metadata {
            if metadata.get("uuid").map(|s| s.as_str()) == Some(&uuid) {
                // Fetch the object and return the note
                let mut object = client.get_object()
                    .bucket(bucket)
                    .key(&key)
                    .send()
                    .await?;

                // Read the object's body and convert it to a string
                let mut body = Vec::new();
                while let Some(bytes) = object.body.try_next().await? {
                    body.extend_from_slice(&bytes);
                }
                let body_str = String::from_utf8(body)?;

                // Extract the creation timestamp from the metadata
                let created_at = metadata.get("created_at").unwrap_or(&String::from("")).clone();

                // Create a Note object with the fetched data
                let note = Note {
                    id: Some(1),
                    uuid: Some(uuid.to_string()),
                    title: key,
                    content: body_str,
                    created_at: created_at.parse::<i64>().unwrap_or(0),
                    updated_at: Some(chrono::Utc::now().timestamp()),
                    timestamp: metadata.get("timestamp").map(|s| s.to_string()),
                };

                return Ok(note);
            }
        }
    }

    // Return an error if the note is not found
    Err("Note not found".into())
}


/// Updates a note in an Amazon S3 bucket based on its UUID.
///
/// # Parameters
///
/// * `bucket` - The name of the bucket where the note is stored.
/// * `note` - The updated note to be stored in the bucket. It should contain the UUID, title, and content of the note.
///
/// # Operation
///
/// * A connection to the Amazon S3 service is established using the AWS SDK for Rust.
/// * The region for the S3 service is set to "eu-west-3".
/// * The list of objects in the bucket is retrieved using the `list_objects_v2` API.
/// * For each object, the `head_object` API is called to retrieve the metadata associated with the object.
/// * If the object has a metadata field with key "uuid" and value matching the UUID of the note, the object is considered as the note to be updated.
/// * The content of the note is converted to bytes and then to a `ByteStream`.
/// * The note is updated by uploading the new content to the object in the bucket.
/// * The metadata fields "uuid" and "timestamp" are updated with the UUID and current timestamp of the note.
///
/// # Returns
///
/// * If the operation is successful, `Ok(())` is returned.
/// * If the operation fails, an error of type `Box<dyn std::error::Error>` is returned.
///
/// # Errors
///
/// This function will return an error if the AWS SDK encounters an error when updating the note or if the note is not found.
pub async fn update_bucket_note (bucket: &str, note: Note) -> Result<(), Box<dyn std::error::Error>> {
    // Establish a connection to the Amazon S3 service
    let myconfig = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("eu-west-3"))
        .load()
        .await;
    let client = s3::Client::new(&myconfig);

    // Extract the UUID from the note
    let uuid = note.uuid.unwrap();

    // Retrieve the list of objects in the bucket
    let list_objects_output = client.list_objects_v2()
        .bucket(bucket)
        .send()
        .await?;

    // Iterate over each object in the bucket
    for object in list_objects_output.contents.unwrap_or_default() {
        let key = object.key.unwrap_or_default();

        // Retrieve the metadata associated with the object
        let head_object_output = client.head_object()
            .bucket(bucket)
            .key(&key)
            .send()
            .await?;

        // Check if the object has a metadata field with key "uuid" and value matching the UUID of the note
        if let Some(metadata) = head_object_output.metadata {
            if metadata.get("uuid").map(|s| s.as_str()) == Some(&uuid) {
                // Convert the content of the note to bytes and then to a ByteStream
                let input_string = note.content.as_bytes().to_vec();
                let bytestream = s3::primitives::ByteStream::from(input_string);

                // Get the current timestamp
                let timestamp = chrono::Utc::now().to_rfc3339();

                // Update the note by uploading the new content to the object in the bucket
                client.put_object()
                    .bucket(bucket)
                    .key(&key)
                    .metadata("uuid", &uuid)
                    .metadata("timestamp", &timestamp)
                    .body(bytestream)
                    .content_type("text/plain")
                    .send()
                    .await?;

                return Ok(());
            }
        }
    }

    // Return an error if the note is not found
    Err("Note not found".into())
}


/// Deletes a note from an Amazon S3 bucket based on its UUID.
///
/// # Parameters
///
/// * `bucket` - The name of the bucket where the note is stored.
/// * `uuid` - The UUID of the note to delete.
///
/// # Operation
///
/// * A connection to the Amazon S3 service is established using the AWS SDK for Rust.
/// * The region for the S3 service is set to "eu-west-3".
/// * The list of objects in the bucket is retrieved using the `list_objects_v2` API.
/// * For each object, the `head_object` API is called to retrieve the metadata associated with the object.
/// * If the object has a metadata field with key "uuid" and value matching the UUID of the note, the object is considered as the note to be deleted.
/// * The note is deleted by calling the `delete_object` API with the key of the object.
///
/// # Returns
///
/// * If the operation is successful, `Ok(())` is returned.
/// * If the operation fails, an error of type `Box<dyn std::error::Error>` is returned.
///
/// # Errors
///
/// This function will return an error if the AWS SDK encounters an error when deleting the note or if the note is not found.
pub async fn delete_bucket_note (bucket: &str, uuid: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Establish a connection to the Amazon S3 service
    let myconfig = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("eu-west-3"))
        .load()
        .await;
    let client = s3::Client::new(&myconfig);

    // Retrieve the list of objects in the bucket
    let list_objects_output = client.list_objects_v2()
        .bucket(bucket)
        .send()
        .await?;

    // Iterate over each object in the bucket
    for object in list_objects_output.contents.unwrap_or_default() {
        let key = object.key.unwrap_or_default();

        // Retrieve the metadata associated with the object
        let head_object_output = client.head_object()
            .bucket(bucket)
            .key(&key)
            .send()
            .await?;

        // Check if the object has a metadata field with key "uuid" and value matching the UUID of the note
        if let Some(metadata) = head_object_output.metadata {
            if metadata.get("uuid").map(|s| s.as_str()) == Some(&uuid) {
                // Delete the note by calling the `delete_object` API with the key of the object
                client.delete_object()
                    .bucket(bucket)
                    .key(&key)
                    .send()
                    .await?;
                return Ok(());
            }
        }
    }

    // Return an error if the note is not found
    Err("Note not found".into())
}



/// Fetches the notes from an Amazon S3 bucket.
///
/// # Parameters
///
/// * `bucket_name` - The name of the bucket to fetch the notes from.
///
/// # Returns
///
/// Returns a `Result` containing a vector of tuples with the following elements:
/// * The key of the note object in the bucket.
/// * The last modified timestamp of the note.
/// * The metadata associated with the note.
/// * The content of the note.
///
/// # Errors
///
/// This function will return an error if the AWS SDK encounters an error when fetching the notes or if there is an error in the response.
pub async fn fetch_bucket_notes(bucket_name: &str) -> Result<Vec<(String, Option<String>, Option<HashMap<String, String>>, String)>, Box<dyn std::error::Error>> {
    // Trim any surrounding quotes from the bucket name
    let bucket_name = bucket_name.trim_matches('"');

    // Create AWS configuration with the desired region
    let myconfig = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("eu-west-3"))
        .load()
        .await;

    // Create an S3 client using the configuration
    let client = s3::Client::new(&myconfig);

    // Send a request to list objects in the bucket
    let mut response = client
        .list_objects_v2()
        .bucket(bucket_name)
        .max_keys(10)
        .into_paginator()
        .send();

    let mut keys = Vec::new();

    // Iterate over the paginated response
    while let Some(result) = response.next().await {
        match result {
            Ok(output) => {
                // Process each object in the response
                for object in output.contents() {
                    if let Some(key) = object.key() {
                        // Send a request to get the object's metadata and content
                        let get_object_output = client
                            .get_object()
                            .bucket(bucket_name)
                            .key(key)
                            .send()
                            .await;

                        // Extract the last modified timestamp, metadata, and content from the response
                        let (last_modified, metadata, content) = match get_object_output {
                            Ok(get_object) => {
                                let last_modified = get_object.last_modified().cloned().map(|dt| dt.to_string());
                                let metadata = get_object.metadata().cloned();
                                let content = get_object.body.collect().await.unwrap().to_vec();
                                let content = String::from_utf8(content).unwrap_or_else(|_| String::new());
                                (last_modified, metadata, content)
                            },
                            Err(err) => {
                                return Err(Box::new(err));
                            }
                        };

                        // Add the note's key, last modified timestamp, metadata, and content to the result vector
                        keys.push((key.to_string(), last_modified, metadata, content));
                    }
                }
            }
            Err(err) => {
                return Err(Box::new(err));
            }
        }
    }

    Ok(keys)
}



/// Deletes all notes from an Amazon S3 bucket.
///
/// # Parameters
///
/// * `bucket_name` - The name of the bucket from which to delete the notes.
///
/// # Operation
///
/// * The `bucket_name` parameter is trimmed to remove any surrounding quotes.
/// * The `fetch_bucket_notes` function is called to retrieve the list of notes in the bucket.
/// * For each note, the `delete_bucket_note` function is called to delete the note from the bucket.
/// * If an error occurs while deleting a note, the error is printed to the standard error stream and returned.
///
/// # Returns
///
/// * If the operation is successful, `Ok(())` is returned.
/// * If the operation fails, an error of type `Box<dyn std::error::Error>` is returned.
///
/// # Errors
///
/// This function will return an error if the AWS SDK encounters an error when deleting a note or if there is an error in the response.
pub async fn delete_bucket_notes(bucket_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Trim the bucket name to remove any surrounding quotes
    let bucket_name = bucket_name.trim_matches('"');

    // Fetch the list of notes in the bucket
    let notes = fetch_bucket_notes(bucket_name).await?;

    // Iterate over each note and delete it from the bucket
    for (_, _, metadata_option, _) in notes {
        if let Some(metadata) = metadata_option {
            if let Some(uuid) = metadata.get("uuid") {
                // Delete the note from the bucket
                match delete_bucket_note(bucket_name, uuid).await {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(e);
                    },
                }
            }
        }
    }

    Ok(())
}