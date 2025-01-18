const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

var projectName = "";

window.onload = function() {
    //Get parameters from URL
    const urlParams = new URLSearchParams(window.location.search);
    projectName = urlParams.get('name');
    update_changes()
    update_branches()
    invoke("pull_repo", {"projectName": projectName})
}

function update_changes() {
    invoke("get_changes", {"projectName": projectName})
    .then((changes) => {
        for(change of changes) {
            document.getElementById("changes").innerHTML += make_change_checkbox(change);
        }
    })
    .catch((error) => {
        invoke("show_error", { "errorMessage": error });
    });
}

function update_branches() {
    invoke("get_branches", {"projectName": projectName})
    .then((branches) => {
        for(branch of branches) {
            select = document.getElementById("branch-select");
            const newOption = new Option(branch, branch);
            select.add(newOption, undefined);
        }
    })
    .catch((error) => {
        invoke("show_error", { "errorMessage": error });
    });
}

function push_to_repo()
{
    const changes = document.querySelectorAll('.checkbox');
    const selectedChanges = [];
    changes.forEach((change) => {
        if(change.checked) {
            selectedChanges.push(change.nextElementSibling.textContent.trim());
        }
    });
    const message = document.getElementById("commit-message").value;
    
    //Commit changes
    console.log("commit")
    invoke("commit", {"projectName": projectName, "changes": selectedChanges, "message": message})
    .then(() => {
        //Pull changes before push
        console.log("pull")
        invoke("pull_repo", {"projectName": projectName})
        .then(() => {
            //Push changes
            console.log("push")
            invoke("push", {"projectName": projectName})
            .then(() => {
                invoke("show_info", { "message": "¡Cambios subidos con éxito!" });
            })
            .catch((error) => {
                invoke("show_error", { "errorMessage": error });
            });
        })
    })
    .catch((error) => {
        invoke("show_error", { "errorMessage": error });
    });
}

function make_change_checkbox(change)
{
    const html = `<div class="custom-check">
                    <label class="checkbox__label" for="checkbox">
                        <span class="checkbox__container">
                            <input class="checkbox" id="checkbox" type="checkbox" />

                            <span class="checkbox__label--text"> ${change} </span>
                        </span>
                    </label>
                </div>`;

    return html;
}