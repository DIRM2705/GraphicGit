use std::path::PathBuf;
use crate::utils::{log::log_error, validation::problem_path_is_valid};
use tauri::AppHandle;
use tauri::{api::dialog::FileDialogBuilder, Manager};

#[tauri::command]
pub fn choose_directory(app_handle: AppHandle) {
    FileDialogBuilder::new().pick_folder(move |sel_path| {
        //User selected a directory
        if sel_path.is_some() {
            let selected_path = &sel_path.unwrap();

            //Verify permissions for the selected directory
            if !problem_path_is_valid(selected_path) {
                log_error("Invalid problem path");
                return;
            }

            //Send a message to indicate the directory was selected
            if let Some(window) = app_handle.get_window("main") {
                let _ = window
                    .emit("directory_selected", String::from(selected_path.to_str().unwrap_or_default()))
                    .map_err(|e| log_error(&e.to_string()));
            } else {return;}
        }
    });
}

#[tauri::command]
pub fn validate_git_repo(repo_path: String) -> bool {
    let full_path = PathBuf::from(format!("{}/.git", repo_path));
    //Validate git repo
    return problem_path_is_valid(&full_path);
}