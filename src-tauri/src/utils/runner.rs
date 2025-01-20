
use std::process::Command;
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Clone)]
pub struct Runner {
    execution_path: String,
}

impl Runner {
    pub fn new(directory: &str) -> Runner {
        return Runner {
            execution_path: String::from(directory),
        };
    }

    pub fn get_execution_path(&self) -> &String {
        return &self.execution_path;
    }

    pub fn set_execution_path(&mut self, path: &str) {
        self.execution_path = String::from(path);
    }

    pub fn exec_cmd(&mut self, cmd: &str) -> Result<(), String> {
        let cmd = Command::new("cmd")
            .args(["/c", cmd])
            .creation_flags(CREATE_NO_WINDOW)
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
            .creation_flags(CREATE_NO_WINDOW)
            .current_dir(&self.execution_path)
            .output()
            .map_err(|e| e.to_string())?;

        return Ok(String::from_utf8_lossy(&cmd.stdout).to_string());
    }

    pub fn exec_with_args(&mut self, cmd: &str, arg: Vec<&str>) -> Result<bool, String> {
        let cmd = Command::new("cmd")
            .args(["/C", cmd])
            .creation_flags(CREATE_NO_WINDOW)
            .args(arg)
            .current_dir(&self.execution_path)
            .spawn()
            .map_err(|e| e.to_string())?
            .wait()
            .map_err(|e| e.to_string())?;

        return Ok(cmd.success());
    }
}
