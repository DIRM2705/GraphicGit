const { invoke } = window.__TAURI__.tauri;
const {listen} = window.__TAURI__.event;

function chooseDir()
{
    invoke("choose_directory");
}