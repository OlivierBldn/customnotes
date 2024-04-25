const { invoke } = window.__TAURI__.tauri;

async function saveNote(note) {
  await invoke('save_note', { note: note });
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#noteForm").addEventListener("submit", (e) => {
    e.preventDefault();
    const note = document.querySelector("#note").value;
    saveNote(note);
  });
});