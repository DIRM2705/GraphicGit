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
      home_cmds::get_recents,
      home_cmds::add_to_recents_file,
      home_cmds::set_current_project,
      dialog_cmds::show_error,
      dialog_cmds::show_info,
      dialog_cmds::show_url_dialog,
      dialog_cmds::show_new_branch_dialog,
      dialog_cmds::show_loading,
      dialog_cmds::close_loading,
      git_cmds::create_repo,
      git_cmds::get_changes,
      git_cmds::get_branches,
      git_cmds::pull_repo,
      git_cmds::commit,
      git_cmds::new_branch,
      git_cmds::checkout_branch,
      git_cmds::connect_remote,
      git_cmds::push
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
