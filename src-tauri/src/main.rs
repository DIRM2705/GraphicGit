// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod utils;

use commands::*;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      home_cmds::choose_directory,
      home_cmds::validate_git_repo,
      dialog_cmds::show_error,
      dialog_cmds::show_info,
      git_cmds::create_repo,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
