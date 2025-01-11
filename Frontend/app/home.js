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
                    console.log(event.payload);
                    invoke("create_repo", { "url": "", "path": event.payload }).
                        then(() => invoke("show_info", { "title": "Nuevo proyecto", "message": "Proyecto creado exitosamente" }))
                        .catch((error) => {
                            invoke("show_error", { "errorMessage": error });
                        });
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
                }
                else {
                    invoke("show_error", { "errorMessage": "No hay un proyecto en esta carpeta" });
                }
            });

        eventlistener.then(f => f());
    });
}