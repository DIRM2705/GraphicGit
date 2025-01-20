
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RecentFiles {
    files: Vec<String>
}

impl RecentFiles {
    pub fn new() -> Self {
        Self {
            files: Vec::new()
        }
    }

    pub fn add(&mut self, file: String) {
        if self.files.len() >= 5 {
            self.files[4] = file;
        }
        else if self.files.contains(&file) {
            let index = self.files.iter().position(|x| *x == file).unwrap();
            self.files.remove(index);
            self.files.push(file);
        }
        else {
            self.files.push(file);
        }

        self.write_to_file();
    }

    pub fn get_recent_files(&self) -> Vec<String> {
        return self.files.clone();
    }

    pub fn read_from_file() -> Self {
        if let Ok(file) = std::fs::read_to_string("recents.json")
        {
            return serde_json::from_str(&file).unwrap()
        }
        else {
            return Self::new()
        }
    }

    pub fn write_to_file(&self) {
        let json = serde_json::to_string(self).unwrap();
        std::fs::write("recents.json", json).unwrap();
    }
}