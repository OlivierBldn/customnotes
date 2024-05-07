# Custom notes app written in Rust
## Tauri & Vanilla
This template is developped using Tauri in vanilla HTML, CSS and Javascript.
## Recommended IDE Setup
- [VS Code](https://code.visualstudio.com/)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Features Available in the Custom Notes App :

## Application Setup and Configuration
- **Windows Subsystem Configuration**: Prevents additional console windows from opening in release mode on Windows systems.
- **Module Organization**: Separation into distinct modules (`models`, `s3_operations`, `local_operations`) for better organization and functionality segmentation.

## Module Features
- **Models**: Handles data structures related to the application, likely involving note handling.
- **S3 Operations**: Functions for interacting with Amazon S3 for cloud storage solutions.
- **Local Operations**: Functions for handling local database operations with SQLite.

## Functional Capabilities
- **Full-Text Search**: Utilizing `tantivy` for full-text search capabilities within notes, involving complex querying and indexing.
- **Command Routing**: Ability to route commands appropriately based on the command string and arguments.
- **Tauri Commands**: Implementation of commands that interact with the Tauri framework for desktop capabilities.
- **Note Handling**: 
  - Searching notes based on queries and optional parameters like local or S3 bucket storage.
  - Ability to handle CRUD operations for notes both locally and on S3.
  - Error handling specific to note fetching and storage operations.
  - **Desktop Notifications**: Implementation of desktop notifications using `notify-rust` to improve user experience.
  - **Content Encryption**: Encryption of the content of the notes for more safety using `ring` and `base64`.


## Asynchronous Operations
- **Asynchronous Command Execution**: Commands are executed asynchronously, ensuring non-blocking operations that enhance performance.
- **Result Handling**: Use of Rust's `Result` type to handle success or error states effectively.

## Error Management
- **Comprehensive Error Handling**: Detailed error handling within operations to manage and report issues effectively, especially related to external operations like S3 interactions.

## Rust-Specific Features
- **Use of Rust Modules**: Leveraging Rust's module system for clean and maintainable code structure.
- **Conditional Compilation**: Specific compilation paths activated based on build settings, enhancing performance and customization of the build.

## Tauri Integration
- **Tauri Setup and Command Registration**: Integration with Tauri for desktop application capabilities, including setup and event loop execution.
- **Database Connection Initialization**: Setup and initialization of database connections crucial for local operations.

## Documentation Specifics
- Detailed documentation of every function, including parameters, returns, and error cases, ensuring clarity for maintenance and further development.
## Requirements
-   Rust (Created under version 1.77.2) : https://www.rust-lang.org/tools/install
-   Node.js (Created under version 20.12.2) : https://nodejs.org/en/download/
## How to ?
### 1 - Clone the project :
```bash
    git clone https://github.com/OlivierBldn/customnotes.git
```
### 2 - Navigate to the project repository :
```bash
    cd customnotes
```
### 3 - Check the requirements :
```bash
    rustc --version
```
```bash
    cargo --version
```
### 4 - Install Tauri's CLI :
Tauri : https://tauri.app/fr/v1/guides/getting-started/prerequisites
```bash
    cargo install tauri-cli --locked
```
### 5 - Launch the project :
#### Under development mode using cargo
- Place yourself in the custom_notes folder and run :
```bash
    cargo tauri dev
```
#### Under production mode using cargo :
- Place yourself in the custom_notes/src-tauri folder (containing the cargo.toml file) and run :
```bash
    cargo tauri build
```
- Then place yourself in the custom_notes/src-tauri/target/release folder and run :
```bash
    ./custom-notes
```

#### Sharing your app :
- If you wish to share your app, you can share the .AppImage file generated with the build as an executable

### 6 - Use Amazon s3 services :
#### AWS s3 user
- Go to https://aws.amazon.com/ and create an account.
- Then open the "Services" menu (top left corner) and search for IAM (Identity and Access Management)
- Go to "Users" and select "Create user"
- Keep default setting except for the "Permission options"
    -   Select "Attach policies directly"
    -   Search for AmazonS3FullAccess and check the checkbox
    -   Continue
- Once your user is created, select it and go to "Security Credentials" tab
- Click on "Create access key"
- Choose "Local code" and create your access key
- Don't forget to save your keys somewhere (you can download the csv file provided by AWS)

#### AWS s3 regions

- Go to https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Concepts.RegionsAndAvailabilityZones.html
- Choose the region you want to deploy your app in (e.g: eu-west-3)

#### Set your environment variables before running cargo tauri dev :

- Open a terminal in your project folder (basically customnotes)

```bash
    export AWS_ACCESS_KEY_ID=myawsaccesskeyid
```

```bash
    export AWS_SECRET_ACCESS_KEY=myawssecretaccesskey
```

```bash
    export AWS_REGION=myawsregion
```

### Congrats, you're all set to use the app !

---

# `Cargo.toml` Documentation

## Package Section

```plaintext
[package]
name = "custom_notes"
version = "0.0.1"
description = "A Custom Notes App"
authors = ["Olivier Blandin"]
edition = "2021"
```

This section defines the package information:

- `name`: The name of your package. Here it's "custom_notes".
- `version`: The current version of your package. It follows the semantic versioning rules.
- `description`: A short description of your package.
- `authors`: A list of the package authors.
- `edition`: Specifies the Rust edition. The "2021" edition makes certain language features stable and changes some defaults to improve ergonomics and safety.

## Build Dependencies Section

```plaintext
[build-dependencies]
tauri-build = { version = "1", features = [] }
```

This section lists the build dependencies for your package. These are libraries that your package depends on during its build process. 

- `tauri-build`: A build dependency for Tauri, a framework for building lightweight, secure desktop apps with web technologies.

## Dependencies Section

```plaintext
[dependencies]
tauri = { version = "1", features = [ "dialog-message", "dialog-save", "dialog-open", "shell-open"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rusqlite = "0.31.0"
lazy_static = "1.4.0"
aws-config = "1.2.1"
aws-sdk-s3 = "1.24.0"
tokio = { version = "1.37.0", features = ["full"] }
tracing = "0.1.37"
anyhow = "1.0"
uuid = { version = "0.8", features = ["v4"] }
tracing-subscriber = "0.2.25"
bytes = "1.0"
chrono = "0.4.38"
tantivy = "0.22.0"
dirs = "5.0.1"
notify-rust = "4.11.0"
ring = "0.17.8"
base64 = "0.22.1"
```

This section lists the dependencies for your package. These are libraries that your package depends on. Each dependency is listed with its version number. Some dependencies also have features specified.

- `tauri`: The Tauri framework for building desktop apps. The specified features enable various dialog and shell capabilities.
- `serde` and `serde_json`: Libraries for serializing and deserializing data. The "derive" feature allows automatic generation of serialization code for data structures.
- `rusqlite`: Bindings for SQLite, a lightweight disk-based database.
- `lazy_static`: A macro for declaring lazily evaluated statics in Rust.
- `aws-config` and `aws-sdk-s3`: Libraries for interacting with AWS services, specifically S3 for object storage.
- `tokio`: A runtime for asynchronous programming in Rust. The "full" feature enables all optional components.
- `tracing` and `tracing-subscriber`: Libraries for application-level tracing, useful for diagnostics and debugging.
- `anyhow`: A library for flexible error handling.
- `uuid`: A library for creating and parsing UUIDs. The "v4" feature enables generation of random UUIDs.
- `bytes`: A utility library for working with byte sequences.
- `chrono`: A date and time library.
- `tantivy`: A full-text search engine library.
- `dirs`: A library for finding various standard directories.
- `notify-rust`: A library for displaying desktop notifications.
- `ring`: A library for cryptography.
- `base64`: A library for encoding and decoding base64 as bytes or utf8.

## Features Section

```plaintext
[features]
custom-protocol = ["tauri/custom-protocol"]
```

This section defines features for your package. Features allow conditional compilation of parts of your code or your dependencies. 

- `custom-protocol`: This feature enables the `custom-protocol` feature of the `tauri` dependency. This is used for production builds or when a dev server is not specified.

---

# `tauri.conf.json` Documentation

This file is the configuration file for the Tauri framework, which is used to build desktop applications with web technologies.

```json
{
  "build": {
    "devPath": "../src",
    "distDir": "../src",
    "withGlobalTauri": true
  },
  "package": {
    "productName": "custom_notes",
    "version": "0.0.1"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "dialog": {
        "open": true,
        "save": true,
        "message": true
      },
      "shell": {
        "all": false,
        "open": true
      }
    },
    "windows": [
      {
        "title": "custom_notes",
        "width": 1920,
        "height": 1080
      }
    ],
    "security": {
      "csp": null
    },
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.customnotes.rust",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    }
  }
}
```

## Build Section

- `devPath`: The path to the development server. Here it's set to "../src".
- `distDir`: The path to the production-ready files. Here it's also set to "../src".
- `withGlobalTauri`: A boolean indicating whether to use the globally installed Tauri CLI or the one specified in your `package.json`.

## Package Section

- `productName`: The name of your product. Here it's "custom_notes".
- `version`: The current version of your product. It follows the semantic versioning rules.

## Tauri Section

### Allowlist Subsection

This subsection specifies which Tauri APIs are allowed in your application.

- `dialog`: Allows the use of dialog boxes for opening files, saving files, and displaying messages.
- `shell`: Allows the use of shell commands. Here, only the "open" command is allowed.

### Windows Subsection

This subsection specifies the default window configuration for your application.

- `title`: The title of the window.
- `width` and `height`: The default width and height of the window.

### Security Subsection

- `csp`: The Content Security Policy for your application. Here it's set to `null`, which means the default CSP is used.

### Bundle Subsection

This subsection specifies the configuration for bundling your application.

- `active`: A boolean indicating whether to bundle the application.
- `targets`: The target platforms for bundling. Here it's set to "all", which means all supported platforms.
- `identifier`: The unique identifier for your application.
- `icon`: An array of paths to the icon files for your application.

---

# `main.rs` Documentation

`main.rs` is the entry point of a Rust application. It is where the `main` function resides, which is the starting point of the program.

## Module Level Attributes

```rust
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```
This line is a conditional compilation attribute that sets the subsystem to "windows" when the code is not being compiled with debug assertions. This prevents an additional console window from appearing when the application is run on Windows in release mode.

## Module Declarations

```rust
mod models;
mod s3_operations;
mod local_operations;
```
These lines declare three modules (`models`, `s3_operations`, and `local_operations`) that are defined in other files in the same directory as `main.rs`. The `models` module likely contains data structures used in the application, `s3_operations` contains functions for interacting with Amazon S3, and `local_operations` contains functions for performing operations on the local file system.

## Imports

```rust
use std::str;
use models::Note;
```
These lines import the `str` module from the Rust standard library and the `Note` struct from the `models` module. The `str` module provides functions for manipulating Rust strings, and `Note` is likely a data structure that represents a note in the application.

```rust
use tantivy::schema::{Schema, TEXT, STORED};
use tantivy::Index;
use tantivy::query::QueryParser;
use tantivy::TantivyDocument;
use tantivy::DocAddress;
use tantivy::Score;
use tantivy::collector::TopDocs;
```
These lines import various items from the `tantivy` crate, which is a full-text search engine library in Rust. `Schema`, `TEXT`, and `STORED` are used to define the schema of the search index. `Index` is used to create and manipulate a search index. `QueryParser` is used to parse query strings into a `Query` object that can be used to search the index. `TantivyDocument` represents a document that can be added to the search index. `DocAddress` and `Score` are used in the search results, with `DocAddress` representing the address of a document in the search index and `Score` representing the relevance score of a document for a particular query. `TopDocs` is a collector that collects the top search results.

### `route_command`

This asynchronous function routes a command to the appropriate operation based on the command string and arguments.

#### Parameters

- `command`: A string representing the command to be executed.
- `args`: A string representing the arguments for the command.

#### Returns

A `Result` containing either the result of the operation as a string or an error message as a string.

This function uses a match statement to determine which command to execute based on the `command` string. It then parses the `args` string into a JSON object and extracts the necessary arguments for the command. The function then calls the appropriate function based on the command and returns the result.

### `execute_command`

This asynchronous function is a Tauri command that routes a command and its arguments to the appropriate function and returns the result.

#### Parameters

- `command`: A string representing the command to be executed.
- `args`: A `serde_json::Value` representing the arguments for the command.

#### Returns

A `Result` containing either the success message as a `String` or an error message as a `String`.

This function simply calls the `route_command` function with the `command` string and the `args` serialized into a string. It then awaits the result and returns it.

### `search_in_notes`

This method retrieves notes based on the specified parameters. It takes in a query string, a boolean indicating whether to fetch local notes or not, and an optional string representing the name of the bucket.

The method first defines the schema for the index, creates a new index, and gets the index writer. It then fetches the notes either locally or from the specified bucket. The notes are then indexed and committed to the index.

A reader and a searcher are created, and a query parser is created for the content field. The query is parsed and the search is performed. The actual content of the documents is then retrieved and returned as a vector of `Note` objects.

#### Parameters

- `query_str`: A string representing the query to search for in the notes.
- `local`: A boolean indicating whether to fetch local notes or not.
- `bucket_name`: An optional string representing the name of the bucket.

#### Returns

A vector of `Note` objects containing the retrieved notes.

#### Errors

Returns an error if any of the following conditions are met:

- `local` is `true` and there was an error retrieving local notes.
- `local` is `false` and `bucket_name` is not provided.
- `local` is `false` and there was an error fetching bucket notes.

### `main`

This is the main entry point of the application. It initializes the Tauri application and sets up the necessary database connection. It registers the command handlers for creating, reading, updating, and deleting notes.

The function then executes the Tauri application and runs the event loop.

#### Errors

Returns an error if there was an issue while running the Tauri application.

---

# `model.rs` Documentation

This file contains the data structures and error handling for the application. It uses the `aws_sdk_s3` crate for AWS S3 operations.

## Structs

### Note

This struct represents a note. It has the following fields:

- `id`: An optional 64-bit integer representing the ID of the note.
- `uuid`: An optional string representing the UUID of the note.
- `title`: A string representing the title of the note.
- `content`: A string representing the content of the note.
- `created_at`: A 64-bit integer representing the creation time of the note.
- `updated_at`: An optional 64-bit integer representing the update time of the note.
- `timestamp`: An optional string representing the timestamp of the note.

## Enums

### BucketError

This enum represents the possible errors that can occur when interacting with S3 buckets. It has the following variants:

- `BucketAlreadyExists`: This variant is used when a bucket already exists.
- `S3Error`: This variant is used when an error occurs in the S3 operation. It contains a boxed standard error.
- `TaggingError`: This variant is used when an error occurs while tagging a bucket.

## Implementations

### From<SdkError<CreateBucketError>> for BucketError

This implementation allows for conversion from `SdkError<CreateBucketError>` to `BucketError`. It wraps the error in the `S3Error` variant of `BucketError`.

### From<SdkError<PutBucketTaggingError>> for BucketError

This implementation allows for conversion from `SdkError<PutBucketTaggingError>` to `BucketError`. It wraps the error in the `S3Error` variant of `BucketError`.

### fmt::Display for BucketError

This implementation allows for `BucketError` to be displayed. It matches on the variant of `BucketError` and writes the corresponding message to the formatter.

### From<aws_sdk_s3::Error> for BucketError

This implementation allows for conversion from `aws_sdk_s3::Error` to `BucketError`. It wraps the error in the `S3Error` variant of `BucketError`.

---

# `local_operations.rs` Documentation

This file contains functions for interacting with a local SQLite database using the `rusqlite` crate in Rust.

## Initialization

The file establishes a connection to a SQLite database named "notes.db" located in the user's home directory. If the file does not exist, it will be created. A SQL statement is executed to create a new table named "notes" in the database if it does not already exist. The table has the following columns:

- "id" (INTEGER): The primary key of the table.
- "uuid" (TEXT): The UUID of the note.
- "title" (TEXT): The title of the note. It cannot be null.
- "content" (TEXT): The content of the note. It cannot be null.
- "created_at" (INTEGER): The timestamp when the note was created.
- "updated_at" (INTEGER): The timestamp when the note was last updated. It can be null.
- "timestamp" (TEXT): The timestamp of the note in RFC 3339 format. It can be null.

This static reference to the database connection is used throughout the application to interact with the database. It is wrapped in a Mutex for thread safety, allowing it to be shared across multiple threads.

## Functions

### `create_local_note`

Creates a new note with the given title and content in the local database.

#### Parameters

- `note`: The note to create. It should contain the title and content of the note.

#### Returns

- `Ok(Note)` if the note is created successfully.
- `Err(String)` if an error occurs.

### `get_local_note`

Retrieves a note from the local database based on its ID.

#### Parameters

- `id`: The ID of the note to retrieve.

#### Returns

- `Ok(Note)` if the note is found.
- `Err(String)` if the note is not found or an error occurs.

### `update_local_note`

Updates the note with the given ID, title, and content in the local database.

#### Parameters

- `note`: The note to update. It should contain the ID, title, and content of the note.

#### Returns

- `Ok(())` if the note is updated successfully.
- `Err(String)` if an error occurs.

### `delete_local_note`

Deletes the note with the given ID from the local database.

#### Parameters

- `id`: The ID of the note to delete.

#### Returns

- `Ok(())` if the note is deleted successfully.
- `Err(String)` if an error occurs.

### `get_local_notes`

Retrieves all notes from the local database.

#### Returns

- `Ok(Vec<(i64, String, String, String, i64, Option<i64>, Option<String>)>)` containing the ID, UUID, title, content, created_at, updated_at, and timestamp of each note.
- `Err(String)` if an error occurs.

### `delete_all_local_notes`

Deletes all notes from the local database.

#### Returns

- `Ok(())` if all notes are deleted successfully.
- `Err(String)` if an error occurs.

### `validate_params`

Validates the title and content of a note.

#### Parameters

- `note`: The note to validate. It should contain the title and content of the note.

#### Returns

- `Ok(())` if the parameters are valid.
- `Err(String)` if an error occurs.

### `derive_nonce_from_id`

Derives the nonce from the note ID in the local database.

#### Parameters

- `id`: The ID of the note to derive the nonce from.

#### Returns

- Returns a `Result` containing the derived nonce as a `String` if it exists, or an `Err` if the nonce is not found or an error occurs.

#### Errors

This function will return an error if there is an issue with the database connection or if the note with the specified ID does not exist.

---

# `s3_operations.rs` Documentation

This file contains functions for interacting with Amazon S3 using the AWS SDK for Rust.

## Functions

### `create_bucket`

Creates a new Amazon S3 bucket.

#### Parameters

- `bucket_name`: The name of the bucket to create.

#### Returns

- `Ok(())` if the operation is successful.
- `Err(BucketError)` if the operation fails.

### `fetch_buckets`

Fetches the list of buckets that have the "App" tag set to "RustCustomNotes".

#### Returns

- `Ok(Vec<String>)` containing the names of the buckets if the operation is successful.
- `Err(s3::Error)` describing the error if the operation fails.

### `bucket_exists`

Checks if an Amazon S3 bucket exists.

#### Parameters

- `bucket_name`: The name of the bucket to check.

#### Returns

- `Ok(true)` if the bucket exists.
- `Ok(false)` if the bucket does not exist.
- `Err(s3::Error)` if an error occurs while checking the bucket existence.

### `delete_bucket`

Deletes an Amazon S3 bucket.

#### Parameters

- `bucket_name`: The name of the bucket to delete.

#### Returns

- `Ok(())` if the operation is successful.
- `Err(s3::Error)` if the operation fails.

### `upload_note_to_bucket`

Uploads a note to an Amazon S3 bucket.

#### Parameters

- `bucket_name`: The name of the bucket to upload the note to.
- `note`: The note to upload. It should contain the title and content of the note.

#### Returns

- `Ok(String)` containing the message "Object uploaded successfully" if the operation is successful.
- `Err(String)` describing the error if the operation fails.

### `fetch_bucket_note`

Fetches a note from an Amazon S3 bucket based on its UUID.

#### Parameters

- `bucket`: The name of the bucket to fetch the note from.
- `uuid`: The UUID of the note to fetch.

#### Returns

- `Ok(Note)` containing the fetched note if the note is found in the bucket.
- `Err(Box<dyn std::error::Error>)` if the note is not found or if the AWS SDK encounters an error when fetching the note.

### `update_bucket_note`

Updates a note in an Amazon S3 bucket based on its UUID.

#### Parameters

- `bucket`: The name of the bucket where the note is stored.
- `note`: The updated note to be stored in the bucket. It should contain the UUID, title, and content of the note.

#### Returns

- `Ok(())` if the operation is successful.
- `Err(Box<dyn std::error::Error>)` if the note is not found or if the AWS SDK encounters an error when updating the note.


### `delete_bucket_note`

Deletes a note from an Amazon S3 bucket based on its UUID.

#### Parameters

- `bucket`: The name of the bucket where the note is stored.
- `uuid`: The UUID of the note to delete.

#### Returns

- `Ok(())` if the operation is successful.
- `Err(Box<dyn std::error::Error>)` if the operation fails or if the note is not found.

### `fetch_bucket_notes`

Fetches the notes from an Amazon S3 bucket.

#### Parameters

- `bucket_name`: The name of the bucket to fetch the notes from.

#### Returns

- `Ok(Vec<(String, Option<String>, Option<HashMap<String, String>>, String)>)` containing a vector of tuples with the note's key, last modified timestamp, metadata, and content if the operation is successful.
- `Err(Box<dyn std::error::Error>)` describing the error if the operation fails.

### `delete_bucket_notes`

Deletes all notes from an Amazon S3 bucket.

#### Parameters

- `bucket_name`: The name of the bucket from which to delete the notes.

#### Returns

- `Ok(())` if the operation is successful.
- `Err(Box<dyn std::error::Error>)` if the operation fails or if there is an error in the response.

### `decrypt_note_content`

Decrypts the content of a note using the provided encrypted content and note ID.

#### Parameters

- `encrypted_content`: The encrypted content of the note.
- `note_id`: The ID of the note.

#### Returns

- Returns a `Result` containing the decrypted content as a `String` if the decryption is successful.
- If the decryption fails, an `Err` with a `String` describing the error is returned.

#### Errors

This function will return an error if any of the following conditions are met:
- Failed to derive the nonce from the note ID.
- Failed to decode the encrypted content.
- Failed to decode the nonce.
- Failed to convert the nonce to an array.
- Failed to generate the encryption key.
- Failed to decrypt the content.

---

# `main.js` Documentation

This file contains functions and event listeners for interacting with local notes and notes stored in a bucket.

## Functions

### `formatTimestamp`

Formats a timestamp into a readable date and time string.

#### Parameters

- `timestampText`: The timestamp to format.

#### Returns

- A string representing the formatted date and time.

### `formatDateTime`

Formats a Date object into a readable date and time string.

#### Parameters

- `timestampText`: The Date object to format.

#### Returns

- A string representing the formatted date and time.

### `searchInNote`

Searches for notes based on a query and location.

#### Parameters

- None

#### Returns

- None (A promise that resolves when the search is complete)

## Event Listeners

### `DOMContentLoaded`

Loads local notes and updates the bucket list when the DOM is fully loaded.

### `click`

Handles click events on the body of the document and performs actions based on the target of the click.

### `change`

Reloads the page or loads bucket notes when the value of the bucket list changes.

## Variables

### `invoke`

A method from the Tauri API for invoking backend commands.

### `noteForm`, `noteId`, `noteTitle`, `noteContent`, `selectedNoteId`

DOM elements for the note form, note ID, note title, note content, and selected note ID.

### `quill`

An instance of the Quill rich text editor.

---

# `localData.js` Documentation

This file contains functions for creating, retrieving, updating, and deleting notes locally using the Tauri API.

## Functions

### `createLocalNote`

Creates a new note locally by invoking the "create_local_note" function.

#### Returns

- `Promise<void>`: A promise that resolves when the note is created successfully.
- Throws an `Error` if an error occurs while creating the note.

### `showLocalNote`

Retrieves and displays a note with the specified ID.

#### Parameters

- `id`: The ID of the note to be shown.

#### Returns

- `Promise<void>`: A promise that resolves when the note is shown successfully.
- Throws an `Error` if an error occurs while retrieving or displaying the note.

### `updateLocalNote`

Updates a note with the specified ID.

#### Parameters

- `id`: The ID of the note to update.

#### Returns

- `Promise<void>`: A promise that resolves when the note is successfully updated.
- Throws an `Error` if an error occurs while updating the note.

### `loadLocalNotes`

Loads the notes from the server and populates the notes table.

#### Returns

- `Promise<void>`: A promise that resolves when the notes are loaded and the table is populated.
- Throws an `Error` if an error occurs while loading the notes.

### `deleteLocalNote`

Deletes a note with the specified ID.

#### Parameters

- `id`: The ID of the note to delete.

#### Returns

- `Promise<void>`: A promise that resolves when the note is deleted.
- Throws an `Error` if an error occurs while deleting the note.

### `deleteAllLocalNotes`

Deletes all local notes by invoking the "delete_all_local_notes" function.

#### Returns

- `Promise<void>`: A promise that resolves when all notes are deleted successfully.
- Throws an `Error` if an error occurs while deleting the notes.

## Functions

### `exportNotesAsJSON`

Exports the notes as JSON and downloads the file.

#### Parameters

- `notes`: The array of notes to export.

#### Returns

- `void`

### `exportNotesAsMarkdown`

Exports the notes as Markdown and downloads the file.

#### Parameters

- `notes`: The array of notes to export.

#### Returns

- `void`

### `exportNotesAsPDF`

Exports the notes as PDF and downloads the file.

#### Parameters

- `notes`: The array of notes to export.

#### Returns

- `void`

### `exportNotes`

Exports the notes in the specified format and downloads the file.

#### Parameters

- `format`: The format in which to export the notes. Supported formats are "pdf", "json", and "markdown".

#### Returns

- `void`

### `importNotes`

Imports notes into the application by creating local notes from the provided data.

#### Parameters

- `notes`: The array of notes to import.

#### Returns

- `Promise<void>`: A promise that resolves when all notes are imported successfully.

#### Throws

- `Error`: If an error occurs while importing the notes.

### `handleFile`

Handles the selected file and imports notes into the application.

#### Parameters

- `event`: The file input change event.

#### Returns

- `void`

---

# `bucketData.js` Documentation

This file contains functions for interacting with the bucket data in the application.

## Functions

### `createBucket`

Creates a new bucket with the specified name.

#### Parameters

- None. The bucket name is fetched from the `#bucket-name` input field.

#### Returns

- A promise that resolves when the bucket is created successfully.
- Throws an error if an error occurs while creating the bucket.

### `isValidBucketName`

Checks if the provided bucket name is valid.

#### Parameters

- `bucketName`: The name of the bucket to validate.

#### Returns

- Returns true if the bucket name is valid, otherwise returns false.

### `updateBucketList`

Updates the bucket list by fetching the latest buckets from the server and populating the dropdown menu.

#### Parameters

- None.

#### Returns

- A promise that resolves when the bucket list is updated successfully.
- Throws an error if an error occurs while updating the bucket list.

### `deleteBucket`

Deletes the selected bucket.

#### Parameters

- None. The bucket name is fetched from the selected value in the bucket list.

#### Returns

- A promise that resolves when the bucket is deleted successfully.
- Throws an error if an error occurs while deleting the bucket.

### `uploadNoteToBucket`

Uploads the displayed note to the specified S3 bucket.

#### Parameters

- None. The bucket name is fetched from the selected value in the bucket list.

#### Returns

- A promise that resolves when the note is uploaded successfully.
- Throws an error if an error occurs while uploading the note.

### `showBucketNote`

Shows the details of a specific note from the selected bucket.

#### Parameters

- `noteUuid`: The UUID of the note to show.

#### Returns

- A promise that resolves when the note is shown successfully.
- Throws an error if an error occurs while showing the note.

### `loadBucketNotes`

Loads the notes from the specified S3 bucket and populates the bucket notes table.

#### Parameters

- None. The bucket name is fetched from the selected value in the bucket list.

#### Returns

- A promise that resolves when the notes are loaded and the table is populated.
- Throws an error if an error occurs while loading the notes.

### `updateBucketNote`

Updates the specified note in the selected bucket with the provided information.

#### Parameters

- `noteUuid`: The UUID of the note to update.

#### Returns

- A promise that resolves when the note is updated successfully.
- Throws an error if an error occurs while updating the note.

### `deleteBucketNote`

Deletes a specific note from the selected bucket.

#### Parameters

- `noteUuid`: The UUID of the note to delete.

#### Returns

- A promise that resolves when the note is deleted successfully.
- Throws an error if an error occurs while deleting the note.

### `deleteBucketNotes`

Deletes all notes from the selected bucket.

#### Parameters

- None. The bucket name is fetched from the selected value in the bucket list.

#### Returns

- A promise that resolves when all notes are deleted successfully.
- Throws an error if an error occurs while deleting the notes.

---

# `test_example.rs` Documentation

This file contains an example of test implementation if eventually you wish to implement some changes to the app