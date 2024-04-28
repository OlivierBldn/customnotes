const { invoke } = window.__TAURI__.tauri;

window.addEventListener("DOMContentLoaded", () => {
  const createNoteButton = document.querySelector("#create-note");
  const noteId = document.querySelector("#note-id");
  const noteTitle = document.querySelector("#note-title");
  const noteContent = document.querySelector("#note-content");
  const notesTableBody = document.querySelector("#notes-table-body");
  const cancelButton = document.querySelector("#cancel-button");
  const selectedNoteId = document.querySelector("#selected-note-id");
  // const saveButton = document.querySelector("#save-note");
  const sendNotesToCloudButton = document.querySelector("#send-notes-to-cloud");

  createNoteButton.addEventListener("click", createNote);
  // saveButton.addEventListener("click", saveNote);
  sendNotesToCloudButton.addEventListener("click", sendNotesToCloud);

  // Clear the input fields when the cancel button is clicked
  cancelButton.addEventListener("click", () => {
    noteId.value = "";
    noteTitle.value = "";
    noteContent.value = "";
    selectedNoteId.textContent = '';
  });

  /**
   * Creates a new note by invoking the "create_note" function.
   *
   * @async
   * @function createNote
   * @returns {Promise<void>} A promise that resolves when the note is created successfully.
   * @throws {Error} If an error occurs while creating the note.
   */
  async function createNote() {
    try {
      await invoke("create_note", {
        note: {
          title: noteTitle.value,
          content: noteContent.value,
        },
      });
      noteTitle.value = "";
      noteContent.value = "";
      await loadNotes();
    } catch (error) {
      console.error("Error creating note:", error);
      alert("An error occurred while trying to create the note.");
    }
  }

  /**
   * Updates a note with the specified ID.
   *
   * @async
   * @param {number} id - The ID of the note to update.
   * @returns {Promise<void>} - A promise that resolves when the note is successfully updated.
   */
  async function updateNote(id) {
    if (!id) {
      console.error("No id provided to updateNote");
      alert("No id provided. Please select a note to update.");
      return;
    }
    try {
      let notes = await invoke("read_notes");
      const note = notes.find((note) => note[0] === id);
      if (!note) {
        console.error("No note found with id:", id);
        alert("Note not found. Please select a note to update.");
        return;
      }
      await invoke("update_note", {
        note: {
          id: Number(note[0]),
          title: noteTitle.value,
          content: noteContent.value,
        },
      });
      await loadNotes();
    } catch (error) {
      console.error("Error updating note:", error);
      alert("An error occurred while trying to update the note.");
    }
  }

  /**
   * Loads the notes from the server and populates the notes table.
   *
   * @async
   * @function loadNotes
   * @returns {Promise<void>} A promise that resolves when the notes are loaded and the table is populated.
   * @throws {Error} If an error occurs while loading the notes.
   */
  async function loadNotes() {
    try {
      let notes = await invoke("read_notes");
      notesTableBody.innerHTML = "";
      notes.forEach((note, index) => {
        const row = notesTableBody.insertRow();
        row.className = index % 2 === 0 ? "even-row" : "odd-row";
        row.innerHTML = `
          <td>${note[0]}</td>
          <td>${note[1]}</td>
          <td>${note[2]}</td>
          <td>
            <button class="btn btn-primary" onclick="showNote(${note[0]})">Show</button>
            <button class="btn btn-secondary" onclick="updateNote(${note[0]})">Update</button>
            <button class="btn btn-danger" onclick="deleteNote(${note[0]})">Delete</button>
          </td>
        `;
      });
    } catch (error) {
      console.error("Error loading notes:", error);
      alert("An error occurred while trying to load the notes.");
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
  async function showNote(id) {
    try {
      let notes = await invoke("read_notes");
      const note = notes.find((note) => note[0] === id);

      if (note) {
        noteId.value = note[0];
        noteTitle.value = note[1];
        noteContent.value = note[2];
        selectedNoteId.textContent = 'Selected Note ID: ' + note[0];
      } else {
        alert("Note not found.");
      }
    } catch (error) {
      console.error("Error showing note:", error);
      alert("An error occurred while trying to show the note.");
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
  async function deleteNote(id) {
    try {
      await invoke("delete_note", { id: Number(id) });
      selectedNoteId.textContent = '';
      await loadNotes();
      noteId.value = '';
      noteTitle.value = '';
      noteContent.value = '';
    } catch (error) {
      console.error("Error deleting note:", error);
      alert("An error occurred while trying to delete the note.");
    }
  }

  /**
   * Saves the displayed note in a s3 bucket
   *
   * @async
   * @param {number} id - The ID of the note to save.
   * @function saveNote
   * @returns {Promise<void>} A promise that resolves when the note is saved successfully.
   * @throws {Error} If an error occurs while saving the note.
   */
  async function saveNote(id) {
    let notes = await invoke("read_notes");
    const note = notes.find((note) => note[0] === id);
    try {
      if (id && note) {
        const result = await invoke('save_note', {
          note: {
            id: Number(note[0]),
            title: noteTitle.value,
            content: noteContent.value,
          },
        });
        console.log(result);
      } else {
        await createNote();
      }
    } catch (error) {
      console.error("Error saving note:", error);
      alert("An error occurred while trying to save the note.");
    }
  }

  /**
   * Sends the notes to the cloud.
   * @async
   * @function sendNotesToCloud
   * @throws {Error} If an error occurs while sending the notes to the cloud.
   * @returns {Promise<void>} A promise that resolves when the notes are successfully sent to the cloud.
   */
  async function sendNotesToCloud() {
    try {
      await invoke("send_notes_to_cloud");
    } catch (error) {
      console.error("Error sending notes to cloud:", error);
      alert("An error occurred while trying to send the notes to the cloud.");
    }
  }

  // Makes the methods globally available in the window element.
  window.showNote = showNote;
  window.updateNote = updateNote;
  window.deleteNote = deleteNote;

  loadNotes();
});
