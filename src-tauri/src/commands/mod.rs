use std::sync::Mutex;

use crate::utils::runner::Runner;

pub mod home_cmds;
pub mod dialog_cmds;
pub mod git_cmds;

pub struct RunnerWrapper(pub Mutex<Runner>);