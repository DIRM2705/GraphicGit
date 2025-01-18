use crate::utils::runner::Runner;
use std::{path::Path, vec};
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn create_repo(handler: AppHandle, url: String, path: String) -> Result<(), String> {
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
        } else {
            if let Some(window) = handler.get_window("git-url") {
                runner.exec_cmd("git pull origin main")?;
                let _ = window.hide();
            }
        }
    }

    return Ok(());
}

#[tauri::command]
pub fn get_changes(project_name: String) -> Result<Vec<String>, String> {
    let mut runner = Runner::load_from_app_data(&project_name)?;
    let output = runner.exec_with_output("git status -s -uall")?;
    let mut changes: Vec<String> = Vec::new();
    output.lines().for_each(|line| {
        if line.starts_with("??") || line.starts_with(" M") || line.starts_with(" A") {
            changes.push(line[2..].to_string());
        }
    });
    return Ok(changes);
}

#[tauri::command]
pub fn get_branches(project_name: String) -> Result<Vec<String>, String> {
    let mut runner = Runner::load_from_app_data(&project_name)?;
    let mut branches: Vec<String> = vec![runner.exec_with_output("git branch --show-current")?];
    let output = runner.exec_with_output("git branch -l -a")?;
    output.lines().for_each(|line| {
        if  !(line.starts_with("  remotes") || line.starts_with("*")) {
            branches.push(line[2..].to_string());
        }
    });
    return Ok(branches);
}

#[tauri::command]
pub fn pull_repo(project_name: String) -> Result<(), String> {
    let mut runner = Runner::load_from_app_data(&project_name)?;
    let branch = runner.exec_with_output("git branch --show-current")?;
    if runner.exec_with_output("git fetch --all")? != ""
    {
        runner.exec_cmd(format!("git pull origin {}", branch).as_str())?;
    }
    return Ok(());
}

#[tauri::command]
pub fn commit(project_name : String, changes : Vec<String>, message : String) -> Result<(), String> {
    let mut runner = Runner::load_from_app_data(&project_name)?;

    if changes.len() == 0 {
        return Err("No hay cambios para hacer commit".to_string());
    }
    if message.trim() == "" {
        return Err("El mensaje del commit no puede estar vacío".to_string());
    }

    for mut change in changes.clone() {

        change = change.replace("\"", "");

        let status = runner.exec_with_args("git", vec!["add", change.as_str()])?;
        if status == false {
            for change in changes {
                runner.exec_with_args("git", vec!["rm", "--cached", change.as_str()])?;
            }
            return Err(format!("No se pudo agregar el archivo {}", change));
        }
    }
    
    let status = runner.exec_with_args("git", vec!["commit", "-m", message.as_str()])?;
    if status == false {
        return Err("No se pudo hacer commit".to_string());
    }
    return Ok(());
}

#[tauri::command]
pub fn push(project_name : String) -> Result<(), String> {
    let mut runner = Runner::load_from_app_data(&project_name)?;
    let branch = runner.exec_with_output("git branch --show-current")?;
    let status = runner.exec_cmd(format!("git push origin {}", branch).as_str())?;
    if status == false {
        return Err("No se pudo hacer push".to_string());
    }
    return Ok(());
}
