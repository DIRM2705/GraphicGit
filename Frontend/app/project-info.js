const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

var projectName = "";

window.onload = function () {
    //Get parameters from URL
    const urlParams = new URLSearchParams(window.location.search);
    projectName = urlParams.get('name');
    refresh()

    listen("add-branch", (changes) => {
        invoke("close_loading");
        invoke("show_info", { "title": "Nueva rama", "message": "¡Rama creada con éxito!" });
        refresh()
    });
}

function update_changes() {
    changePanel = document.getElementById("changes")
    changePanel.innerHTML = "<p class=\"changes-header\">Agregar cambios</p>";
    invoke("get_changes")
        .then((changes) => {
            for (change of changes) {
                changePanel.innerHTML += make_change_checkbox(change);
            }
        })
        .catch((error) => {
            invoke("show_error", { "errorMessage": error });
        });
}

function refresh() {
    invoke("show_loading");
    update_changes()
    update_branches()
    invoke("pull_repo")
        .then(() => {
            invoke("close_loading");
        });
}

function add_branch() {
    invoke("show_new_branch_dialog")
}

function checkout_branch() {
    const branch = document.getElementById("branch-select").value;
    invoke("checkout_branch", {"branchName": branch })
        .then(() => {
            refresh()
            invoke("show_info", { "title": "Cambio de rama", "message": "¡Rama cambiada con éxito!" });
        })
        .catch((error) => {
            invoke("show_error", { "errorMessage": error });
        });
}

function update_branches() {
    select = document.getElementById("branch-select");
    select.length = 0;
    invoke("get_branches")
        .then((branches) => {
            for (branch of branches) {
                const newOption = new Option(branch, branch);
                select.add(newOption, undefined);
            }
        })
        .catch((error) => {
            invoke("show_error", { "errorMessage": error });
        });
}

function push_to_repo() {
    invoke("show_loading");
    const changes = document.querySelectorAll('.checkbox');
    const selectedChanges = [];
    changes.forEach((change) => {
        if (change.checked) {
            selectedChanges.push(change.nextElementSibling.textContent.trim());
        }
    });
    const message = document.getElementById("commit-message").value;
    // Clear the commit message textarea
    document.getElementById("commit-message").value = "";

    //Commit changes
    console.log("commit")
    invoke("commit", {"changes": selectedChanges, "message": message })
        .then(() => {
            //Pull changes before push
            invoke("pull_repo")
                .then(() => {
                    //Push changes
                    invoke("push")
                        .then(() => {
                            invoke("close_loading");
                            invoke("show_info", { "title": "Git push", "message": "¡Cambios subidos con éxito!" });
                            refresh()
                        })
                        .catch((error) => {
                            invoke("close_loading");
                            invoke("show_error", { "errorMessage": error });
                        });
                })
                .catch((error) => {
                    invoke("close_loading");
                    invoke("show_error", { "errorMessage": error });
                });
        })
        .catch((error) => {
            invoke("close_loading");
            invoke("show_error", { "errorMessage": error });
        });
}

function make_change_checkbox(change) {
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