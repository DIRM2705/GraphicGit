use crate::utils::runner::Runner;
use crate::utils::{log::log_error, validation::problem_path_is_valid};
use crate::RunnerWrapper;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{api::dialog::FileDialogBuilder, Manager};
use tauri::{AppHandle, State};

#[tauri::command]
pub fn choose_directory(app_handle: AppHandle) {
    FileDialogBuilder::new().pick_folder(move |sel_path| {
        //Main window
        let window = app_handle
            .get_window("main")
            .expect("Main window not found");

        //User selected a directory
        if sel_path.is_some() {
            let selected_path = &sel_path.unwrap();

            //Verify permissions for the selected directory
            if !problem_path_is_valid(selected_path) {
                log_error("Invalid problem path");
                return;
            }

            println!("Selected path: {:?}", selected_path);
            
            if let Some(state) = app_handle.try_state::<RunnerWrapper>()
            {
                state.0.lock().unwrap().set_execution_path(selected_path.to_str().unwrap_or_default());
            }
            else {
                let runner = Runner::new(selected_path.to_str().unwrap_or_default());
                app_handle.manage(RunnerWrapper(Mutex::new(runner)));
            }


            let _ = window
                .emit(
                    "directory_selected",
                    String::from(selected_path.to_str().unwrap_or_default()),
                )
                .map_err(|e| log_error(&e.to_string()));
            return;
        }

        let _ = window
            .emit("directory_selected", None::<String>)
            .map_err(|e| log_error(&e.to_string()));
    });
}

#[tauri::command]
pub fn validate_git_repo(state: State<RunnerWrapper>) -> bool {
    let repo_path = state.0.lock().unwrap().get_execution_path().clone();
    let full_path = PathBuf::from(format!("{}/.git", repo_path));
    //Validate git repo
    return problem_path_is_valid(&full_path);
}

#[tauri::command]
pub fn get_recents() -> Vec<String> {
    //Read recents file
    let recents_file = PathBuf::from("recents.txt");
    let recents = std::fs::read_to_string(&recents_file).unwrap_or_default();
    //Make each line a separate string
    let lines = recents.lines().collect::<Vec<&str>>();
    return lines.iter().map(|s| s.to_string()).collect();
}

#[tauri::command]
pub fn add_to_recents_file(state: State<RunnerWrapper>) {
    let repo_path = state.0.lock().unwrap().get_execution_path().clone();
    //Add path to recents file
    let recents_file = PathBuf::from("recents.txt");
    let mut recents = std::fs::read_to_string(&recents_file).unwrap_or_default();
    recents = format!("{}\n{}", repo_path, recents);
    std::fs::write(&recents_file, recents).unwrap();
}
