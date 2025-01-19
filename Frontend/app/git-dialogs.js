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
        })
        .catch((error) => {
            invoke("show_error", { "errorMessage": error });
        });
}

function sendBranchName()
{
    //Get name parameter
    const urlParams = new URLSearchParams(window.location.search);
    const repo_name = urlParams.get('name');
    invoke("new_branch", {"projectName" : repo_name, "branchName" : document.getElementById("branch-name").value})
    .then(() => {
    })
    .catch((error) => {
        invoke("show_error", { "errorMessage": error });
    });
}