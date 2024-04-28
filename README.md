# Custom notes app written in Rust
## Tauri & Vanilla
This template is developped using Tauri in vanilla HTML, CSS and Javascript.
## Recommended IDE Setup
- [VS Code](https://code.visualstudio.com/)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
### Current features
-   Write a note in a text input
-   Create automatically a note.txt file at the root of the project
-   Saves the note when the form is submitted
-   Modify the actual note when form is re-submitted
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
- Place yourself in the custom_notes folder and run :
```bash
    cargo tauri build
```
- Then place yourself in the src-tauri/target/release folder and run :
```bash
    ./custom-notes
```