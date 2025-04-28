/*------------------------------------ RUNTIME CONFIGURATIONS ------------------------------------*/
//! This module contains all the functions related to managing the runtime configurations file for a
//! working directory.
//!
//! ### Submodules
//! - [`buffer`] - contains items related to the runtime configurations buffer management
//! - [`task`] - contains items related to specifying a task configuration

pub(crate) mod buffer;
mod task;

pub(crate) use task::*;

use crate::utils::{notify, setup_module_state};
use ::nvim_oxi::api::{self as nvim, Buffer};
use std::path::PathBuf;
use std::sync::LazyLock;

setup_module_state!(config, [pub(crate)]
{
    filepath: PathBuf = self::get_runtime_filepath(),
    buffer: Buffer = Buffer::from(0),
    version: u8 = 1,
    pub(crate) tasks: Vec<TaskConfigJson> = Vec::new(),
});

pub(crate) fn on_dirchanged() {
    self::state!().filepath = self::get_runtime_filepath();
}

pub(crate) fn get_data_dir() -> &'static PathBuf {
    static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        let mut ret = match nvim::call_function::<_, String>("stdpath", ("data",)) {
            Ok(path) => PathBuf::from(path),
            Err(err) => {
                notify!(Error: format!("stdpath('data') failed - {err}"));
                return PathBuf::from("/nvim-stdpath-data-error");
            },
        };
        // FIX: change this to default path before release
        ret.push("launch_nvim_rust");

        ret
    });

    &DATA_DIR
}

fn get_runtime_filepath() -> PathBuf {
    let json_filename = match std::env::current_dir() {
        Ok(cwd) => cwd.to_string_lossy().replace("@", "@@"),
        Err(err) => {
            notify!(Error: format!("std::env::current_dir() failed - {err}"));
            return PathBuf::from("/rust-env-cwd-error");
        },
    };
    let re = unsafe { ::regex::Regex::new(r"[\\/:]").unwrap_unchecked() };
    let json_filename = format!("{}.json", re.replace_all(&json_filename, "@"));
    self::get_data_dir().join(json_filename)
}
