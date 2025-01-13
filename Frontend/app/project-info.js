const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

window.onload = function() {
    //Get parameters from URL
    const urlParams = new URLSearchParams(window.location.search);
    const name = urlParams.get('name');
    update_changes(name)
    update_branches(name)
    invoke("pull_repo", {"projectName": name})
}

function update_changes(name) {
    invoke("get_changes", {"projectName": name})
    .then((changes) => {
        for(change of changes) {
            document.getElementById("changes").innerHTML += make_change_checkbox(change);
        }
    })
    .catch((error) => {
        invoke("show_error", { "errorMessage": error });
    });
}

function update_branches(name) {
    invoke("get_branches", {"projectName": name})
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