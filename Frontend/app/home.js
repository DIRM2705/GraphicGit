const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;


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
                    invoke("show_url_dialog", {"repoDir": event.payload});
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
                    console.log(dirName);
                    window.location.href = "project-info.html?name=" + dirName;
                }
                else {
                    invoke("show_error", { "errorMessage": "No hay un proyecto en esta carpeta" });
                }
            });

        eventlistener.then(f => f());
    });
}