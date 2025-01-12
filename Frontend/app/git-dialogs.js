const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

var repo_dir = "";

window.onload = function () {
    listen('get_repo_dir', (event) => {
        repo_dir = event.payload;
    });
}


function sendURL() {
    //Validate the inpnut git-url has a github url
    const url = document.getElementById("github-url").value;
    if (!url || !url.includes("github.com")) {
        invoke("show_error", { "errorMessage": "La URL no es válida" });
        return;
    }

    invoke("create_repo", { "url": url, "path": repo_dir }).
        then(() => invoke("show_info", { "title": "Nuevo proyecto", "message": "Proyecto creado exitosamente" }))
        .catch((error) => {
            invoke("show_error", { "errorMessage": error });
        });
}