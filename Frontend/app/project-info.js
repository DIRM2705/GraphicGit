const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

window.onload = function() {
    //Get parameters from URL
    const urlParams = new URLSearchParams(window.location.search);
    const name = urlParams.get('name');
    update_changes(name)
}

function update_changes(name) {
    invoke("get_changes", {"projectName": name})
    .then((changes) => {
        for(change of changes) {
            document.getElementById("changes").innerHTML += make_change_checkbox(change);
        }
    })
    .catch((except) => {
        console.log(name)
        invoke("show_error", { "errorMessage": "No se encontró el proyecto" });
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