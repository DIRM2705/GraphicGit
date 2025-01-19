const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

window.onload = function () {
    listen("connection-success", (event) => {
        invoke("show_info", { "title": "Nuevo proyecto", "message": "Proyecto creado exitosamente" });
    });
}

function newProject() {
    invoke("choose_directory");
    const eventlistener = listen('directory_selected', (event) => {
        if (event.payload == null) {
            eventlistener.then(f => f());
            return;
        };

        invoke("validate_git_repo", { "repoPath": event.payload })
            .then((repo_already_exists) => {
                if (repo_already_exists) {
                    invoke("show_error", { "errorMessage": "Este proyecto ya fue inicializado" });
                }
                else {
                    invoke("create_repo", {"path": event.payload})
                    var dirName = event.payload.split('\\').slice(-1).toString();
                    invoke("show_url_dialog", {"repoName": dirName});
                }
            });



        eventlistener.then(f => f());
    });
}

function openProject() {
    invoke("choose_directory");
    const eventlistener = listen('directory_selected', (event) => {
        if (event.payload == null) {
            eventlistener.then(f => f());
            return;
        };

        invoke("validate_git_repo", { "repoPath": event.payload })
            .then((repo_already_exists) => {
                if (repo_already_exists) {
                    //Go to project page
                    var dirName = event.payload.split('\\').slice(-1);
                    window.location.href = "project-info.html?name=" + dirName;
                }
                else {
                    invoke("show_error", { "errorMessage": "No hay un proyecto en esta carpeta" });
                }
            });

        eventlistener.then(f => f());
    });
}