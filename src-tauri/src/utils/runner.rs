use std::process::Command;

pub struct Runner {
    //This is a struct that will be used to run commands
    execution_path: String,
}

impl Runner {
    pub fn new(directory: &str) -> Runner {
        return Runner {
            execution_path: String::from(directory),
        };
    }

    pub fn exec_cmd(&mut self, cmd: &str) -> Result<(), String> {

        Command::new("cmd")
            .args(["/C", cmd])
            .current_dir(&self.execution_path)
            .spawn().map_err(|e| e.to_string())?
            .wait().map_err(|e| e.to_string())?;

        return Ok(());
    }
}
