use crate::utils::runner::Runner;
use tauri::{AppHandle, Manager};
use std::path::Path;

#[tauri::command]
pub fn create_repo(handler : AppHandle, url : String, path : String) -> Result<(), String> {

    let path = Path::new(&path);
    
    //Get directory name from path
    let dir_name = path
        .file_name()
        .ok_or_else(|| "Invalid path".to_string())?
        .to_str()
        .ok_or_else(|| "Invalid UTF-8 sequence".to_string())?;


    let mut runner = Runner::new(dir_name, path.to_str().unwrap());
    runner.save_to_app_data()?;

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

#[tauri::command]
pub fn get_changes(project_name : String) -> Result<Vec<String>, String>
{
    let mut runner = Runner::load_from_app_data(&project_name)?;
    let output = runner.exec_with_output("git status -s -uall")?;
    let mut changes : Vec<String> = Vec::new();
    output.lines().for_each(|line| {
        if line.starts_with("??") || line.starts_with(" M") || line.starts_with(" A")
        {
            changes.push(line[2..].to_string());
        }
    });
    return Ok(changes);
}