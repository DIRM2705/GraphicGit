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
pub fn show_info(handler : AppHandle, title :String, message : String)
{
    dialog::message(handler.get_focused_window().as_ref(), title, message);
}