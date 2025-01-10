const { invoke } = window.__TAURI__.tauri;
const {listen} = window.__TAURI__.event;

function newProject()
{
    invoke("choose_directory");
    const eventlistener = listen('directory_selected', (event) => {
        console.log(event.payload);
        eventlistener.then(f => f());
    });
}

function openProject()
{
    invoke("choose_directory");
    const eventlistener = listen('directory_selected', (event) => {
        console.log(event.payload);
        eventlistener.then(f => f());
    });
}