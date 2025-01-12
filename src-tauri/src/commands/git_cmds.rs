use crate::utils::runner::Runner;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn create_repo(handler : AppHandle, url : String, path : String) -> Result<(), String> {
    let mut runner = Runner::new(&path);

    runner.exec_cmd("git init")?;
    runner.exec_cmd("git branch -M main")?;
    runner.exec_cmd(&format!("git remote add origin {}", url))?;

    let fetch = runner.exec_cmd("git fetch --all");

    if let Ok(res) = fetch {

        if !res {
            runner.exec_cmd("git remote remove origin")?;
            return Err("La URL no es válida".to_string());
        }
        else {
            if let Some(window) = handler.get_window("git-url")
            {
                runner.exec_cmd("git pull origin main")?;
                let _ = window.hide();
            }
        }
    }


    return Ok(());
}