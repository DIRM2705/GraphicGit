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
pub fn show_url_dialog(handler : AppHandle, repo_dir : String)
{
    if let Some(window) = handler.get_window("git-url")
    {
        let _ = window.show();
        let _ = window.emit("get_repo_dir", repo_dir);
    }
}

#[tauri::command]
pub fn show_info(handler : AppHandle, title :String, message : String)
{
    dialog::message(handler.get_focused_window().as_ref(), title, message);
}