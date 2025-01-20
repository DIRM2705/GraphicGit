use std::vec;
use tauri::{AppHandle, Manager, State};

use crate::RunnerWrapper;

#[tauri::command]
pub async fn create_repo(state: State<'_, RunnerWrapper>) -> Result<(), String> {
    let mut runner = state.0.lock().unwrap();

    runner.exec_cmd("git init")?;
    runner.exec_cmd("git branch -M main")?;

    return Ok(());
}

#[tauri::command]
pub async fn connect_remote(handler: AppHandle, state: State<'_, RunnerWrapper>, url: String) -> Result<(), String> {
    let mut runner = state.0.lock().unwrap();
    runner.exec_cmd(&format!("git remote add origin {}", url))?;

    let fetch = runner.exec_cmd("git fetch --all");

    if fetch.is_err() {
        runner.exec_cmd("git remote remove origin")?;
        return Err("La URL no es válida".to_string());
    } else {
        runner.exec_cmd("git pull origin main")?;

        if let Some(window) = handler.get_window("git-url") {
            window.close().map_err(|e| e.to_string())?;
        }
        handler.emit_all("connection-success", "").map_err(|e| e.to_string())?;
    }

    return Ok(());
}

#[tauri::command]
pub async fn get_changes(state: State<'_, RunnerWrapper>) -> Result<Vec<String>, String> {
    let mut runner = state.0.lock().unwrap();
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
pub async fn get_branches(state: State<'_, RunnerWrapper>) -> Result<Vec<String>, String> {
    let mut runner = state.0.lock().unwrap();
    let mut branches: Vec<String> = vec![runner.exec_with_output("git branch --show-current")?];
    let output = runner.exec_with_output("git branch -l -a")?;
    output.lines().for_each(|line| {
        if !(line.starts_with("  remotes") || line.starts_with("*")) {
            branches.push(line[2..].to_string());
        }
    });
    return Ok(branches);
}

#[tauri::command]
pub async fn new_branch(handler : AppHandle, state : State<'_, RunnerWrapper>, branch_name : String) -> Result<(), String>
{
    let mut runner = state.0.lock().unwrap();

    let branch_name = branch_name.replace(" ", "-");
    runner.exec_with_args("git", vec!["checkout", "-b", &branch_name])?;

    if let Some(window) = handler.get_window("new-branch") {
        window.close().map_err(|e| e.to_string())?;
    }

    handler.emit_all("add-branch","").map_err(|e| e.to_string())?;

    return Ok(());
}

#[tauri::command]
pub async fn checkout_branch(state : State<'_, RunnerWrapper>, branch_name: String) -> Result<(), String> {
    let mut runner = state.0.lock().unwrap();
    runner.exec_with_args("git", vec!["checkout", &branch_name])?;
    return Ok(());
}

#[tauri::command]
pub async fn pull_repo(state : State<'_, RunnerWrapper>) -> Result<(), String> {
    let mut runner = state.0.lock().unwrap();
    let branch = runner.exec_with_output("git branch --show-current")?;
    if runner.exec_with_output("git fetch --all")? != "" {
        runner.exec_cmd(format!("git pull origin {}", branch).as_str())?;
    }
    return Ok(());
}

#[tauri::command]
pub async fn commit(state : State<'_, RunnerWrapper>, changes: Vec<String>, message: String) -> Result<(), String> {
    let mut runner = state.0.lock().unwrap();

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
pub async fn push(state : State<'_, RunnerWrapper>) -> Result<(), String> {
    let mut runner = state.0.lock().unwrap();
    let branch = runner.exec_with_output("git branch --show-current")?;
    runner.exec_cmd(format!("git push origin {}", branch).as_str())?;
    return Ok(());
}
