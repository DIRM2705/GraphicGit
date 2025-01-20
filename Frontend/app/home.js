const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

var newProjectButton = null;
var openProjectButton = null;

window.onload = function () {
    listen("connection-success", (event) => {
        invoke("close_loading");
        invoke("show_info", { "title": "Nuevo proyecto", "message": "Proyecto creado exitosamente" });
    });

    newProjectButton = document.getElementById("newProject");
    openProjectButton = document.getElementById("openProject");
    recentsPanel = document.getElementById("recent-projects");
    recents = invoke("get_recents");
    recents.then((recents) => {
        recents.forEach((element) => {
            recentsPanel.innerHTML += `<p>${element}</p>`;
        });
    });
}

function newProject() {
    toggle_buttons();
    invoke("choose_directory");
    const eventlistener = listen('directory_selected', (event) => {
        if (event.payload == null) {
            eventlistener.then(f => f());
            return;
        };

        invoke("validate_git_repo")
            .then((repo_already_exists) => {
                if (repo_already_exists) {
                    invoke("show_error", { "errorMessage": "Este proyecto ya fue inicializado" });
                }
                else {
                    invoke("create_repo")
                    invoke("show_url_dialog");
                }
            });

        toggle_buttons();
        eventlistener.then(f => f());
    });
}

function openProject() {
    toggle_buttons();
    invoke("choose_directory");
    const eventlistener = listen('directory_selected', (event) => {
        if (event.payload == null) {
            eventlistener.then(f => f());
            return;
        };

        invoke("validate_git_repo")
            .then((repo_already_exists) => {
                if (repo_already_exists) {
                    //Go to project page
                    //invoke("add_to_recents_file");
                    window.location.href = "project-info.html";
                }
                else {
                    invoke("show_error", { "errorMessage": "No hay un proyecto en esta carpeta" });
                    toggle_buttons();
                }
            });

        eventlistener.then(f => f());
    });
}

function toggle_buttons()
{
    newProjectButton.disabled = !newProjectButton.disabled;
    openProjectButton.disabled = !openProjectButton.disabled;
}