use std::path::{Path, PathBuf};

pub fn problem_path_is_valid(selected_path: &PathBuf) -> bool {
    //Check if the selected path is a valid directory
    let path = Path::new(selected_path);
    if !path.is_dir() {
        return false;
    }

    return true;
}