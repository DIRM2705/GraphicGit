use crate::utils::log::log_error;
use tauri::api::dialog;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn show_error(handler : AppHandle, error_message : String)
{
    log_error(&error_message);
    dialog::message(handler.get_focused_window().as_ref(), "Error", error_message);
}

#[tauri::command]
pub async fn show_url_dialog(handler : AppHandle, repo_name : String) -> Result<(), String>
{
    let url_window = tauri::WindowBuilder::new(
        &handler,
        "git-url",
        tauri::WindowUrl::App(format!("git-url.html?name={}", repo_name).into()),
    )
    .build().unwrap();

    url_window.set_title("Git URL").map_err(|e| e.to_string())?;

    url_window
        .set_minimizable(false)
        .map_err(|e| e.to_string())?;
    url_window
        .set_maximizable(false)
        .map_err(|e| e.to_string())?;
    url_window.set_resizable(false).map_err(|e| e.to_string())?;
    url_window.center().map_err(|e| e.to_string())?;
    url_window
        .set_size(tauri::Size::Logical(tauri::LogicalSize {
            width: 600.0,
            height: 150.0,
        }))
        .map_err(|e| e.to_string())?;
    url_window
        .set_always_on_top(true)
        .map_err(|e| e.to_string())?;

    url_window.show().map_err(|e| e.to_string())?;
    

    return Ok(());
}

#[tauri::command]
pub async fn show_new_branch_dialog(handler : AppHandle, repo_name : String) -> Result<(), String>
{
    let url_window = tauri::WindowBuilder::new(
        &handler,
        "new-branch",
        tauri::WindowUrl::App(format!("new-branch.html?name={}", repo_name).into()),
    )
    .build().unwrap();

    url_window.set_title("Nueva rama").map_err(|e| e.to_string())?;

    url_window
        .set_minimizable(false)
        .map_err(|e| e.to_string())?;
    url_window
        .set_maximizable(false)
        .map_err(|e| e.to_string())?;
    url_window.set_resizable(false).map_err(|e| e.to_string())?;
    url_window.center().map_err(|e| e.to_string())?;
    url_window
        .set_size(tauri::Size::Logical(tauri::LogicalSize {
            width: 600.0,
            height: 150.0,
        }))
        .map_err(|e| e.to_string())?;
    url_window
        .set_always_on_top(true)
        .map_err(|e| e.to_string())?;

    url_window.show().map_err(|e| e.to_string())?;
    

    return Ok(());
}

#[tauri::command]
pub fn show_info(handler : AppHandle, title :String, message : String)
{
    dialog::message(handler.get_focused_window().as_ref(), title, message);
}

#[tauri::command]
pub fn show_loading(handler : AppHandle) -> Result<(), String>
{
    let loading_window = tauri::WindowBuilder::new(
        &handler,
        "loading",
        tauri::WindowUrl::App("loading.html".into()),
    ).build().map_err(|e| e.to_string())?;

    loading_window.set_title("Cargando").map_err(|e| e.to_string())?;
    loading_window.set_minimizable(false).map_err(|e| e.to_string())?;
    loading_window.set_maximizable(false).map_err(|e| e.to_string())?;
    loading_window.set_resizable(false).map_err(|e| e.to_string())?;
    loading_window.center().map_err(|e| e.to_string())?;
    loading_window.set_size(tauri::Size::Logical(tauri::LogicalSize {
        width: 0.150,
        height: 150.0,
    })).map_err(|e| e.to_string())?;
    loading_window.set_always_on_top(true).map_err(|e| e.to_string())?;
    loading_window.set_focus().map_err(|e| e.to_string())?;

    loading_window.show().map_err(|e| e.to_string())?;
    Ok(())
}
