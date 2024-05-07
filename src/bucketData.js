// bucketData.js

const { invoke } = window.__TAURI__.tauri;
import {
  quill,
  noteForm,
  noteId,
  noteTitle,
  noteContent,
  selectedNoteId,
  formatTimestamp,
  formatDateTime,
} from "./main.js";

const bucketList = document.querySelector("#bucket-list");
const notesTableBody2 = document.querySelector("#notes-table-2-body");

/**
 * Creates a new bucket with the specified name.
 *
 * @async
 * @function createBucket
 * @returns {Promise<void>} A promise that resolves when the bucket is created successfully.
 * @throws {Error} If an error occurs while creating the bucket.
 */
export async function createBucket() {
  const bucketName = document.querySelector("#bucket-name").value;
  if (!bucketName || !isValidBucketName(bucketName)) {
    alert("Please enter a valid bucket name");
    return;
  }
  try {
    await invoke("execute_command", {
      command: "create_bucket",
      args: {
        bucket_name: bucketName,
      },
    });
    await updateBucketList();
    alert("Bucket created successfully");
  } catch (error) {
    console.error("Error creating bucket:", error);
    if (error.toString().includes("Bucket already exists")) {
      alert("The bucket already exists. Please choose a different name.");
    } else {
      alert("An error occurred while trying to create the bucket.");
    }
  }
}

/**
 * Checks if the provided bucket name is valid.
 *
 * @param {string} bucketName - The name of the bucket to validate.
 * @returns {boolean} Returns true if the bucket name is valid, otherwise returns false.
 */
function isValidBucketName(bucketName) {
  if (bucketName.length < 3 || bucketName.length > 63) {
    alert("Bucket name must be between 3 and 63 characters long");
    return false;
  }
  if (!/^[a-z0-9][a-z0-9.-]*[a-z0-9]$/.test(bucketName)) {
    alert("Bucket name must start and end with a lowercase letter or number");
    return false;
  }
  if (/(\d+\.){3}\d+/.test(bucketName)) {
    alert("Bucket name must not be an IP address");
    return false;
  }
  if (
    bucketName.includes("..") ||
    bucketName.includes(".-") ||
    bucketName.includes("-.")
  ) {
    alert(
      "Bucket name must not contain consecutive periods or periods adjacent to hyphens"
    );
    return false;
  }
  return true;
}

/**
 * Updates the bucket list by fetching the latest buckets from the server and populating the dropdown menu.
 *
 * @async
 * @function updateBucketList
 * @returns {Promise<void>} A promise that resolves when the bucket list is updated successfully.
 * @throws {Error} If an error occurs while updating the bucket list.
 */
export async function updateBucketList() {
  const bucketsJson = await invoke("execute_command", {
    command: "fetch_buckets",
    args: {},
  });

  const buckets = JSON.parse(bucketsJson);

  // Clear existing options in the bucket list
  while (bucketList.firstChild) {
    bucketList.removeChild(bucketList.firstChild);
  }

  // Add default option
  const defaultOption = document.createElement("option");
  defaultOption.value = "default";
  defaultOption.text = "Select a bucket";
  bucketList.appendChild(defaultOption);

  // Add options for each bucket
  for (const bucket of buckets) {
    const option = document.createElement("option");
    option.value = bucket;
    option.text = bucket;
    bucketList.appendChild(option);
  }
}

/**
 * Deletes the selected bucket.
 *
 * @async
 * @function deleteBucket
 * @returns {Promise<void>} A promise that resolves when the bucket is deleted successfully.
 * @throws {Error} If an error occurs while deleting the bucket.
 */
export async function deleteBucket() {
  const bucketName = bucketList.value;
  if (bucketName === "default") {
    alert("Please select a bucket to delete");
    return;
  }
  try {
    await invoke("execute_command", {
      command: "delete_bucket",
      args: {
        bucket_name: bucketName,
      },
    });
    await updateBucketList();
    alert("Bucket deleted successfully");
  } catch (error) {
    console.error("Error deleting bucket:", error);
    alert("An error occurred while trying to delete the bucket.");
  }
}

/**
 * Uploads the displayed note to the specified S3 bucket.
 *
 * @async
 * @function uploadNoteToBucket
 * @returns {Promise<void>} A promise that resolves when the note is uploaded successfully.
 * @throws {Error} If an error occurs while uploading the note.
 */
export async function uploadNoteToBucket() {
  if (bucketList.value === "default") {
    alert("Please select a bucket to upload the note to");
    return;
  }

  let notes = JSON.parse(
    await invoke("execute_command", { command: "get_local_notes", args: "" })
  );
  const id = noteForm.dataset.noteId;
  const note = notes.find((note) => note.id === Number(id));

  try {
    if (note) {
      const result = await invoke("execute_command", {
        command: "upload_note_to_bucket",
        args: {
          bucket_name: bucketList.value,
          note: {
            id: Number(note.id),
            uuid: note.uuid,
            title: note.title,
            content: note.content,
            nonce: null,
            created_at: Number(note.created_at),
            updated_at: Number(note.updated_at),
            timestamp: note.timestamp,
          },
        },
      });
      await loadBucketNotes();
      alert("Note uploaded successfully to bucket");
    } else {
      alert("Please save your note locally first.");
    }
  } catch (error) {
    console.error("Error uploading note:", error);
    alert("An error occurred while trying to upload the note.");
  }
}

/**
 * Shows the details of a specific note from the selected bucket.
 *
 * @async
 * @function showBucketNote
 * @param {string} noteUuid - The UUID of the note to show.
 * @returns {Promise<void>} A promise that resolves when the note is shown successfully.
 * @throws {Error} If an error occurs while showing the note.
 */
export async function showBucketNote(noteUuid) {
  document.getElementById("selected-note-id").innerText = "";
  noteForm.removeAttribute("data-noteId");
  let content = JSON.stringify(quill.getContents());

  if (!noteUuid) {
    console.error("No UUID provided to showBucketNote");
    alert("No UUID provided. Please select a note to show.");
    return;
  }
  try {
    let noteJson = await invoke("execute_command", {
      command: "fetch_bucket_note",
      args: {
        bucket_name: bucketList.value,
        uuid: noteUuid,
      },
    });
    let note = JSON.parse(noteJson);

    if (!note) {
      console.error("No note found with UUID:", noteUuid);
      alert("Note not found. Please select a note to show.");
      return;
    }

    noteId.value = note.id;
    noteTitle.value = note.title;
    let noteContent;
    if (typeof note.content === "string") {
      let jsonEndIndex = note.content.lastIndexOf('}') + 1;
      let jsonPart = note.content.substring(0, jsonEndIndex);
      let nonJsonPart = note.content.substring(jsonEndIndex);
    
      if (jsonPart.startsWith('{') || jsonPart.startsWith('[')) {
        try {
          noteContent = JSON.parse(jsonPart);
        } catch (error) {
          console.error("Error parsing note content:", error);
          alert("An error occurred while trying to parse the note content.");
          return;
        }
      } else {
        noteContent = note.content;
      }
    } else {
      noteContent = note.content;
    }
    quill.setContents(noteContent);
  } catch (error) {
    console.error("Error showing note:", error);
    alert("An error occurred while trying to show the note.");
  }
}

/**
 * Loads the notes from the specified S3 bucket and populates the bucket notes table.
 *
 * @async
 * @function loadBucketNotes
 * @returns {Promise<void>} A promise that resolves when the notes are loaded and the table is populated.
 * @throws {Error} If an error occurs while loading the notes.
 */
export async function loadBucketNotes() {
  try {
    // Fetch the notes from the server
    let notes = JSON.parse(
      await invoke("execute_command", {
        command: "fetch_bucket_notes",
        args: {
          bucket_name: bucketList.value,
        },
      })
    );

    if (!notes) {
      alert("No notes found in the bucket.");
      return;
    }

    // Clear the existing table rows
    notesTableBody2.innerHTML = "";

    // Populate the table with the fetched notes
    notes.forEach((note, index) => {
      const row = notesTableBody2.insertRow();
      row.className = index % 2 === 0 ? "even-row" : "odd-row";
      const metadata = note[2];
      row.innerHTML = `
          <td>${note[0]}</td>
          <td>${formatDateTime(note[1])}</td>
          <td>
            <button class="btn btn-primary" onclick="showBucketNote('${
              note[2].uuid
            }')">Show</button>
            <button class="btn btn-secondary" onclick="updateBucketNote('${
              note[2].uuid
            }')">Update</button>
            <button class="btn btn-danger" onclick="deleteBucketNote('${
              note[2].uuid
            }')">Delete</button>
          </td>
        `;
    });
  } catch (error) {
    console.error("Error loading bucket notes:", error);
    alert("An error occurred while trying to load the bucket notes.");
  }
}

/**
 * Updates the specified note in the selected bucket with the provided informations.
 *
 * @async
 * @function updateBucketNote
 * @param {string} noteUuid - The UUID of the note to update.
 * @returns {Promise<void>} A promise that resolves when the note is updated successfully.
 * @throws {Error} If an error occurs while updating the note.
 */
export async function updateBucketNote(noteUuid) {
  let content = JSON.stringify(quill.getContents());

  if (!noteUuid) {
    console.error("No UUID provided to updateBucketNote");
    alert("No UUID provided. Please select a note to update.");
    return;
  }

  if (!noteTitle.value) {
    alert("Please enter a title.");
    return;
  }

  try {
    let noteJson = await invoke("execute_command", {
      command: "fetch_bucket_note",
      args: {
        bucket_name: bucketList.value,
        uuid: noteUuid,
      },
    });
    let note = JSON.parse(noteJson);
    if (!note) {
      console.error("No note found with UUID:", noteUuid);
      alert("Note not found. Please select a note to update.");
      return;
    }

    await invoke("execute_command", {
      command: "update_bucket_note",
      args: {
        bucket_name: bucketList.value,
        note: {
          uuid: noteUuid,
          title: noteTitle.value,
          content: content,
          created_at: note.created_at,
          updated_at: Math.floor(Date.now() / 1000),
        },
      },
    });

    quill.setContents([]);
    noteTitle.value = "";
    document.getElementById("selected-note-id").innerText = "";
    noteForm.removeAttribute("data-noteId");
    noteUuid = null;
    await loadBucketNotes();
    alert("Note updated successfully");
  } catch (error) {
    console.error("Error updating note:", error);
    alert("An error occurred while trying to update the note.");
  }
}

/**
 * Deletes a specific note from the selected bucket.
 *
 * @async
 * @function deleteBucketNote
 * @param {string} noteUuid - The UUID of the note to delete.
 * @returns {Promise<void>} A promise that resolves when the note is deleted successfully.
 * @throws {Error} If an error occurs while deleting the note.
 */
export async function deleteBucketNote(noteUuid) {
  try {
    await invoke("execute_command", {
      command: "delete_bucket_note",
      args: {
        bucket_name: bucketList.value,
        uuid: noteUuid,
      },
    });
    await loadBucketNotes();
    alert("Note removed successfully from the bucket");
  } catch (error) {
    console.error("Error deleting note:", error);
    alert("An error occurred while trying to delete the note.");
  }
}

/**
 * Deletes all notes from the selected bucket.
 *
 * @async
 * @function deleteBucketNotes
 * @returns {Promise<void>} A promise that resolves when all notes are deleted successfully.
 * @throws {Error} If an error occurs while deleting the notes.
 */
export async function deleteBucketNotes() {
  if (bucketList.value === "default") {
    alert("Please select a bucket to delete notes from");
    return;
  }

  try {
    await invoke("execute_command", {
      command: "delete_bucket_notes",
      args: {
        bucket_name: bucketList.value,
      },
    });
    await loadBucketNotes();
    alert("All notes removed successfully from the bucket");
  } catch (error) {
    console.error("Error deleting notes:", error);
    alert("An error occurred while trying to delete the notes.");
  }
}
