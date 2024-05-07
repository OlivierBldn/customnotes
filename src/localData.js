// localData.js

const { invoke } = window.__TAURI__.tauri;
import {
  quill,
  noteForm,
  noteId,
  noteTitle,
  noteContent,
  selectedNoteId,
  formatTimestamp,
} from "./main.js";

const notesTableBody = document.querySelector("#notes-table-body");

/**
 * Creates a new note locally by invoking the "create_local_note" function.
 *
 * @async
 * @function createLocalNote
 * @returns {Promise<void>} A promise that resolves when the note is created successfully.
 * @throws {Error} If an error occurs while creating the note.
 */
export async function createLocalNote() {
  let content = JSON.stringify(quill.getContents());
  try {
    await invoke("execute_command", {
      command: "create_local_note",
      args: {
        note: {
          title: noteTitle.value,
          content: content,
          created_at: Math.floor(Date.now() / 1000),
          updated_at: Math.floor(Date.now() / 1000),
        },
      },
    });
    noteTitle.value = "";
    quill.setContents([]);
    await loadLocalNotes();
  } catch (error) {
    console.error("Error creating note:", error);
    alert("An error occurred while trying to create the note.");
  }
}

/**
 * Retrieves and displays a note with the specified ID.
 *
 * @async
 * @param {number} id - The ID of the note to be shown.
 * @returns {Promise<void>} - A promise that resolves when the note is shown successfully.
 * @throws {Error} - If an error occurs while retrieving or displaying the note.
 */
export async function showLocalNote(id) {
  try {
    // Retrieve the note from the server
    let noteJson = await invoke("execute_command", {
      command: "get_local_note",
      args: { id: id },
    });
    let note = JSON.parse(noteJson);

    if (note) {
      // Update the form fields with the note data
      noteId.value = note.id;
      noteTitle.value = note.title;
      let content;
      if (typeof note.content === "string") {
        try {
          content = JSON.parse(note.content);
        } catch (error) {
          console.error("Error parsing content as JSON:", error);
          content = note.content;
        }
      } else {
        content = note.content;
      }
      quill.setContents(content);
      selectedNoteId.textContent = "Selected Note ID: " + note.id;
      noteForm.dataset.noteId = note.id;
    } else {
      alert("Note not found.");
    }
  } catch (error) {
    console.error("Error showing note:", error);
    alert("An error occurred while trying to show the note.");
  }
}

/**
 * Updates a note with the specified ID.
 *
 * @async
 * @param {number} id - The ID of the note to update.
 * @returns {Promise<void>} - A promise that resolves when the note is successfully updated.
 * @throws {Error} - If an error occurs while updating the note.
 */
export async function updateLocalNote(id) {
  let content = JSON.stringify(quill.getContents());

  if (!id) {
    console.error("No id provided to updateLocalNote");
    alert("No id provided. Please select a note to update.");
    return;
  }

  if (!noteTitle.value) {
    alert("Please enter a title.");
    return;
  }

  try {
    let noteJson = await invoke("execute_command", {
      command: "get_local_note",
      args: { id },
    });
    let note = JSON.parse(noteJson);
    if (!note) {
      console.error("No note found with id:", id);
      alert("Note not found. Please select a note to update.");
      return;
    }
    await invoke("execute_command", {
      command: "update_local_note",
      args: {
        note: {
          id: Number(note.id),
          title: noteTitle.value,
          content: content,
          created_at: note.created_at,
          updated_at: Math.floor(Date.now() / 1000),
        },
      },
    });

    quill.setContents([]);
    noteTitle.value = "";
    selectedNoteId.textContent = "";
    noteForm.removeAttribute("data-noteId");
    id = null;
    await loadLocalNotes();
    alert("Note updated successfully");
  } catch (error) {
    console.error("Error updating note:", error);
    alert("An error occurred while trying to update the note.");
  }
}

/**
 * Loads the notes from the server and populates the notes table.
 *
 * @async
 * @function loadLocalNotes
 * @returns {Promise<void>} A promise that resolves when the notes are loaded and the table is populated.
 * @throws {Error} If an error occurs while loading the notes.
 */
export async function loadLocalNotes() {
  try {
    // Retrieve the notes from the server
    let notes = JSON.parse(
      await invoke("execute_command", { command: "get_local_notes", args: "" })
    );

    // Clear the existing table rows
    notesTableBody.innerHTML = "";

    // Populate the table with the retrieved notes
    notes.forEach((note, index) => {
      const row = notesTableBody.insertRow();
      row.className = index % 2 === 0 ? "even-row" : "odd-row";
      row.innerHTML = `
          <td>${note.title}</td>
          <td>${formatTimestamp(note.created_at)}</td>
          <td>${formatTimestamp(note.updated_at)}</td>
          <td>
            <button class="btn btn-primary" onclick="showLocalNote(${
              note.id
            })">Show</button>
            <button class="btn btn-secondary" onclick="updateLocalNote(${
              note.id
            })">Update</button>
            <button class="btn btn-danger" onclick="deleteLocalNote(${
              note.id
            })">Delete</button>
          </td>
        `;
    });
  } catch (error) {
    console.error("Error loading local notes:", error);
    alert("An error occurred while trying to load the local notes.");
  }
}

/**
 * Deletes a note with the specified ID.
 *
 * @async
 * @param {number} id - The ID of the note to delete.
 * @returns {Promise<void>} - A promise that resolves when the note is deleted.
 * @throws {Error} - If an error occurs while deleting the note.
 */
export async function deleteLocalNote(id) {
  try {
    // Invoke the "delete_local_note" command to delete the note
    await invoke("execute_command", {
      command: "delete_local_note",
      args: {
        id: Number(id),
      },
    });

    // Clear the selected note ID and form fields
    selectedNoteId.textContent = "";
    noteId.value = "";
    noteTitle.value = "";
    quill.setText("");
    quill.removeFormat(0, quill.getLength());

    // Reload the local notes and update the table
    await loadLocalNotes();

    // Display a success message
    alert("Note deleted successfully");
  } catch (error) {
    console.error("Error deleting note:", error);
    alert("An error occurred while trying to delete the note.");
  }
}

/**
 * Deletes all local notes by invoking the "delete_all_local_notes" function.
 *
 * @async
 * @function deleteAllLocalNotes
 * @returns {Promise<void>} A promise that resolves when all notes are deleted successfully.
 * @throws {Error} If an error occurs while deleting the notes.
 */
export async function deleteAllLocalNotes() {
  try {
    // Invoke the "delete_all_local_notes" command to delete all notes
    await invoke("execute_command", {
      command: "delete_all_local_notes",
      args: "",
    });
    // Reload the local notes and update the table
    await loadLocalNotes();
    // Clear the form fields and editor content
    noteId.value = "";
    noteTitle.value = "";
    quill.setText("");
    quill.removeFormat(0, quill.getLength());
  } catch (error) {
    console.error("Error deleting all notes:", error);
    alert("An error occurred while trying to delete all notes.");
  }
}


/**
 * Exports the notes as JSON and downloads the file.
 *
 * @param {Array} notes - The array of notes to export.
 * @returns {void}
 */
async function exportNotesAsJSON(notes) {
  // Create a new Quill instance
  const quill = new Quill('#editor');

  notes.forEach(note => {
    // Decrypt the content using the encryption algorithm
    const decryptedContent = decrypt(note.content);

    // Parse the decrypted content as JSON to get the Delta
    const delta = JSON.parse(decryptedContent);

    // Set the Quill contents to the Delta
    quill.setContents(delta);

    // Get the plain text from the Quill editor
    note.content = quill.getText();
  });

  // Convert the notes array to a formatted JSON string
  const dataStr = JSON.stringify(notes, null, 2);

  // Create a Blob from the JSON string
  const dataBlob = new Blob([dataStr], {type: 'application/json'});

  // Create a URL for the Blob
  const url = URL.createObjectURL(dataBlob);

  // Create a link element to download the JSON file
  const link = document.createElement('a');
  link.href = url;
  link.download = 'notes.json';
  link.click();

  // Revoke the URL to free up memory
  URL.revokeObjectURL(url);
}


/**
 * Exports the notes as Markdown and downloads the file.
 *
 * @param {Array} notes - The array of notes to export.
 * @returns {void}
 */
function exportNotesAsMarkdown(notes) {
  let markdownContent = "";
  notes.forEach(note => {
    markdownContent += `# ${note.title}\n\n${note.content}\n\n`;
  });

  const blob = new Blob([markdownContent], { type: "text/plain" });
  const url = URL.createObjectURL(blob);

  const link = document.createElement('a');
  link.href = url;
  link.download = "notes.md";
  link.click();

  URL.revokeObjectURL(url);
}


/**
 * Exports the notes as PDF and downloads the file.
 *
 * @param {Array} notes - The array of notes to export.
 * @returns {void}
 */
function exportNotesAsPDF(notes) {
  const doc = new jsPDF();
  let y = 10;

  notes.forEach((note, index) => {
    const lines = doc.splitTextToSize(note.content, 180);
    doc.setFontSize(12); 
    doc.text(note.title, 10, y);
    y += 10;
    doc.setFontSize(16); 
    doc.text(lines, 10, y);
    y += lines.length * 10;
  });

  doc.save("notes.pdf");
}


/**
 * Exports the notes in the specified format and downloads the file.
 *
 * @param {string} format - The format in which to export the notes. Supported formats are "pdf", "json", and "markdown".
 * @returns {void}
 */
export async function exportNotes(format) {
  const response = await invoke("execute_command", { command: "get_local_notes", args: "" });
  const notes = JSON.parse(response);

  switch (format) {
    case 'pdf':
      exportNotesAsPDF(notes);
      break;
    case 'json':
      exportNotesAsJSON(notes);
      break;
    case 'markdown':
      exportNotesAsMarkdown(notes);
      break;
    default:
      console.error("Unsupported format:", format);
  }
}


/**
 * Imports notes into the application by creating local notes from the provided data.
 *
 * @async
 * @param {Array} notes - The array of notes to import.
 * @returns {Promise<void>} - A promise that resolves when all notes are imported successfully.
 * @throws {Error} - If an error occurs while importing the notes.
 */
async function importNotes(notes) {
  // Create a new Quill instance
  const quill = new Quill(document.createElement('div'));

  for (let note of notes) {
    // Insert the HTML content into the Quill editor
    quill.clipboard.dangerouslyPasteHTML(note.content);

    // Get the Delta from the Quill editor
    note.content = quill.getContents();

    try {
      await invoke("execute_command", {
        command: "create_local_note",
        args: {
          note: {
            title: note.title,
            content: JSON.stringify(note.content),
            created_at: Math.floor(Date.now() / 1000),
            updated_at: Math.floor(Date.now() / 1000),
          },
        },
      });

      await loadLocalNotes();
    } catch (error) {
      console.error("Error creating note:", error);
      alert("An error occurred while trying to create the note.");
    }
  }
}


/**
 * Handles the selected file and imports notes into the application.
 *
 * @param {Event} event - The file input change event.
 * @returns {void}
 */
export async function handleFile(event) {
  const file = event.target.files[0];
  if (file) {
    const reader = new FileReader();
    reader.onload = async function(event) {
      let notes;
      switch (file.name.split('.').pop()) {
        case 'json':
          notes = JSON.parse(event.target.result);
          break;
        case 'csv':
          const csvData = Papa.parse(event.target.result, {header: true});
          notes = csvData.data.map(row => ({
            title: row.Title,
            content: row.Content
          }));
          break;
        case 'md':
          const mdData = event.target.result;
          notes = mdData.split('# ').map(section => {
            const [title, ...content] = section.split('\n');
            return {
              title: title.trim(),
              content: content.join('\n').trim()
            };
          });
          break;
        default:
          console.error("Unsupported file type:", file.type);
          return;
      }
      notes = notes.filter(note => note.title.trim() !== "" && note.content.trim() !== "");
      await importNotes(notes);
    };
    reader.readAsText(file);
  }
}