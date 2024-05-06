// main.js

import {
  createLocalNote,
  showLocalNote,
  updateLocalNote,
  deleteLocalNote,
  loadLocalNotes,
  deleteAllLocalNotes,
} from "./localData.js";
import {
  createBucket,
  updateBucketList,
  loadBucketNotes,
  uploadNoteToBucket,
  showBucketNote,
  updateBucketNote,
  deleteBucket,
  deleteBucketNote,
  deleteBucketNotes,
} from "./bucketData.js";

export const { invoke } = window.__TAURI__.tauri;
export const noteForm = document.querySelector("#note-form");
export const noteId = document.querySelector("#note-id");
export const noteTitle = document.querySelector("#note-title");
export const noteContent = document.querySelector("#note-content");
export const selectedNoteId = document.querySelector("#selected-note-id");

// Initialize the Quill editor
export const quill = new Quill("#editor", {
  modules: {
    toolbar: [
      ["bold", "italic", "underline", "strike"],
      ["blockquote", "code-block"],
      [{ header: 1 }, { header: 2 }],
      [{ list: "ordered" }, { list: "bullet" }],
      [{ script: "sub" }, { script: "super" }],
      [{ indent: "-1" }, { indent: "+1" }],
      [{ direction: "rtl" }],
      [{ size: ["small", false, "large", "huge"] }],
      [{ header: [1, 2, 3, 4, 5, 6, false] }],
      [{ color: [] }, { background: [] }],
      [{ font: [] }],
      [{ align: [] }],
      ["clean"],
    ],
  },
  theme: "snow",
});

// Reload the page when the reload button is clicked
document.querySelector("#reload-button").addEventListener("click", function () {
  location.reload();
});

// Format the timestamp to a human-readable format
export function formatTimestamp(timestampText) {
  if (timestampText === null) {
    return "-";
  }
  let timestamp = Number(timestampText);
  let date = new Date(timestamp * 1000);
  let seconds = String(date.getSeconds()).padStart(2, "0");
  let minutes = String(date.getMinutes()).padStart(2, "0");
  let hours = String(date.getHours()).padStart(2, "0");
  let day = String(date.getDate()).padStart(2, "0");
  let month = String(date.getMonth() + 1).padStart(2, "0");
  let year = date.getFullYear();
  return `${day}/${month}/${year} - ${hours}:${minutes}:${seconds}`;
}

// Format the timestamp to a human-readable format
export function formatDateTime(timestampText) {
  const timestamp = new Date(timestampText);
  const day = String(timestamp.getDate()).padStart(2, "0");
  const month = String(timestamp.getMonth() + 1).padStart(2, "0");
  const year = String(timestamp.getFullYear()).slice(-2);
  const hours = String(timestamp.getHours()).padStart(2, "0");
  const minutes = String(timestamp.getMinutes()).padStart(2, "0");
  const seconds = String(timestamp.getSeconds()).padStart(2, "0");
  return `${day}/${month}/${year} - ${hours}:${minutes}:${seconds}`;
}


window.addEventListener("DOMContentLoaded", () => {
  const bucket = "olivier-rust-custom-notes";

  const cancelButton = document.querySelector("#cancel-button");

  document.body.addEventListener("click", (event) => {
    event.preventDefault();

    const targetId = event.target.id;

    switch (targetId) {
      case "cancel-button":
        noteId.value = "";
        noteTitle.value = "";
        quill.setContents("");
        selectedNoteId.textContent = "";
        break;
      case "create-local-note":
        createLocalNote();
        break;
      case "show-local-note":
        showLocalNote();
        break;
      case "update-local-note":
        updateLocalNote();
        break;
      case "delete-local-note":
        deleteLocalNote();
        break;
      case "create-bucket":
        createBucket();
        break;
      case "delete-all-local-notes":
        deleteAllLocalNotes();
        break;
      case "delete-bucket":
        deleteBucket();
        break;
      case "upload-note-to-bucket":
        uploadNoteToBucket();
        break;
      case "empty-bucket":
        deleteBucketNotes();
        break;
      case "search-button":
        searchInNote();
        break;
    }
  });

  const bucketList = document.querySelector("#bucket-list");

  // Refresh the notes list when a bucket is selected
  bucketList.addEventListener("change", async () => {
    if (bucketList.value === "default") {
      location.reload();
    } else {
      await loadBucketNotes();
    }
  });

  /**
   * Performs a search in the notes based on the provided search query and location.
   * @returns {Promise<void>} A promise that resolves when the search is complete.
   */
  async function searchInNote() {
    // Get the search query, search location, and bucket name from the input fields
    const searchQuery = document.querySelector("#search-query").value;
    const searchLocation = document.querySelector("#search-location").value;
    const bucketName = document.querySelector("#bucket-list").value;

    // Validate the search query
    if (!searchQuery) {
      alert("Please enter a search query");
      return;
    }

    // Prepare the arguments for the search command
    let args = {
      query: searchQuery,
      local: searchLocation === "local",
    };

    // If the search location is "bucket", add the bucket name to the arguments
    if (searchLocation === "bucket") {
      args.bucket_name = bucketName;
    }

    try {
      // Invoke the "execute_command" function with the search command and arguments
      const searchResultsJson = await invoke("execute_command", {
        command: "search_in_notes",
        args: args,
      });

      // Parse the search results from the JSON response
      const searchResults = JSON.parse(searchResultsJson);

      // Handle the case when no results are found
      if (searchResults.length === 0) {
        alert("No result found");
        return;
      }

      // Update the UI based on the search location
      if (searchLocation === "local") {
        const notesTableBody = document.querySelector("#notes-table-body");
        notesTableBody.innerHTML = "";
        searchResults.forEach((note, index) => {
          const row = notesTableBody.insertRow();
          row.className = index % 2 === 0 ? "even-row" : "odd-row";
          row.innerHTML = `
            <td>${note.title}</td>
            <td>${formatTimestamp(note.created_at)}</td>
            <td>${formatTimestamp(note.updated_at)}</td>
            <td>
              <button class="btn btn-primary" onclick="showBucketNote('${note.id}')">Show</button>
              <button class="btn btn-secondary" onclick="updateBucketNote('${note.id}')">Update</button>
              <button class="btn btn-danger" onclick="deleteBucketNote('${note.id}')">Delete</button>
            </td>
          `;
        });
      } else {
        const notesTableBody2 = document.querySelector("#notes-table-2-body");
        notesTableBody2.innerHTML = "";
        searchResults.forEach((note, index) => {
          const row = notesTableBody2.insertRow();
          row.className = index % 2 === 0 ? "even-row" : "odd-row";
          row.innerHTML = `
              <td>${note.title}</td>
              <td>${formatDateTime(note.timestamp)}</td>
              <td>
                  <button class="btn btn-primary" onclick="showBucketNote('${
                    note.uuid
                  }')">Show</button>
                  <button class="btn btn-secondary" onclick="updateBucketNote('${
                    note.uuid
                  }')">Update</button>
                  <button class="btn btn-danger" onclick="deleteBucketNote('${
                    note.uuid
                  }')">Delete</button>
              </td>
          `;
        });
      }
    } catch (error) {
      console.error("Error searching in notes:", error);
      alert("An error occurred while trying to search in the notes.");
    }
  }

  // Makes the methods globally available in the window element.
  window.showLocalNote = showLocalNote;
  window.updateLocalNote = updateLocalNote;
  window.deleteLocalNote = deleteLocalNote;

  window.showBucketNote = showBucketNote;
  window.updateBucketNote = updateBucketNote;
  window.deleteBucketNote = deleteBucketNote;

  loadLocalNotes();
  updateBucketList();
});
