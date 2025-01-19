const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

function sendURL() {
    //Validate the input git-url has a github url
    const url = document.getElementById("github-url").value;
    if (!url || !url.includes("github.com")) {
        invoke("show_error", { "errorMessage": "La URL no es válida" });
        return;
    }

    //Get name parameter
    const urlParams = new URLSearchParams(window.location.search);
    const repo_name = urlParams.get('name');

    invoke("connect_remote", { "url": url, "projectName": repo_name }).
        then(() => 
            {
                invoke("show_info", { "title": "Nuevo proyecto", "message": "Proyecto creado exitosamente" });
        })
        .catch((error) => {
            invoke("show_error", { "errorMessage": error });
        });
}