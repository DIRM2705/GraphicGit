use bincode;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct Runner {
    //This is a struct that will be used to run commands
    project_name: String,
    pub execution_path: String,
}

impl Runner {
    pub fn new(name: &str, directory: &str) -> Runner {
        return Runner {
            project_name: String::from(name),
            execution_path: String::from(directory),
        };
    }

    pub fn exec_cmd(&mut self, cmd: &str) -> Result<(), String> {
        let cmd = Command::new("cmd")
            .args(["/c", cmd])
            .current_dir(&self.execution_path)
            .spawn()
            .map_err(|e| e.to_string())?
            .wait()
            .map_err(|e| e.to_string())?;
        if cmd.success() {
            return Ok(());
        } else {
            return Err("Ha ocurrido un error".to_string());
        }
    }

    pub fn exec_with_output(&mut self, cmd: &str) -> Result<String, String> {
        let cmd = Command::new("cmd")
            .args(["/C", cmd])
            .current_dir(&self.execution_path)
            .output()
            .map_err(|e| e.to_string())?;

        return Ok(String::from_utf8_lossy(&cmd.stdout).to_string());
    }

    pub fn exec_with_args(&mut self, cmd: &str, arg: Vec<&str>) -> Result<bool, String> {
        let cmd = Command::new("cmd")
            .args(["/C", cmd])
            .args(arg)
            .current_dir(&self.execution_path)
            .spawn()
            .map_err(|e| e.to_string())?
            .wait()
            .map_err(|e| e.to_string())?;

        return Ok(cmd.success());
    }

    pub fn save_to_app_data(&self) -> Result<(), String> {
        let path = format!("{}.ggit", self.project_name);
        let data = bincode::serialize(&self).map_err(|e| e.to_string())?;
        std::fs::write(path, data).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn load_from_app_data(name: &str) -> Result<Runner, String> {
        let path = format!("{}.ggit", name);
        let data = std::fs::read(path).map_err(|e| e.to_string())?;
        let runner: Runner = bincode::deserialize(&data).map_err(|e| e.to_string())?;
        Ok(runner)
    }
}
