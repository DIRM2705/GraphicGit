use crate::utils::runner::Runner;

#[tauri::command]
pub fn create_repo(url : String, path : String) -> Result<(), String> {
    let mut runner = Runner::new(&path);

    runner.exec_cmd("git init")?;
    //runer.exec_cmd("git remote add origin", vec![url]))?;
    runner.exec_cmd("git branch -M main")?;

    return Ok(());
}