use tauri::{api::dialog::FileDialogBuilder, Manager};
use tauri::AppHandle;
use crate::utils::{log::log_error, validation::problem_path_is_valid};

#[tauri::command]
pub fn choose_directory(app_handle : AppHandle){
    FileDialogBuilder::new().pick_folder(move |sel_path|{
        //User selected a directory
        if sel_path.is_some() 
        {
            let selected_path = &sel_path.unwrap();
            
            //Verify permissions for the selected directory
            if !problem_path_is_valid(selected_path)
            {
                log_error("Invalid problem path");
                return;
            }
        }

        //Send a message to indicate the directory was selected
        if let Some(window) = app_handle.get_window("main")
        {
            let _ = window.emit("directory_selected", "test")
            .map_err(|e| log_error(&e.to_string()));
        }
    });
}