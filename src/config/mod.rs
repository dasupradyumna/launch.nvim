/*------------------------------------ RUNTIME CONFIGURATIONS ------------------------------------*/
//!
//! This module contains functions related to managing the runtime configurations file for a working
//! directory and the standard data directory.
//!
//! ## Submodules
//! - [`buffer`] - contains items related to the runtime configurations buffer management
//! - [`task`] - contains items related to specifying a task configuration

pub(crate) mod buffer;
mod task;

pub(crate) use task::*;

use crate::utils::{notify, setup_module_state};
use ::nvim_oxi::api::{self as nvim, Buffer};
use std::path::PathBuf;

setup_module_state!(config, [pub(crate)]
{
    filepath: PathBuf = self::get_runtime_filepath(),
    buffer: Buffer = Buffer::from(0),
    version: u8 = 1,
    pub(crate) tasks: Vec<TaskConfigJson> = Vec::new(),
});

/// Callback logic for the `DirChanged` event
pub(crate) fn on_dirchanged() {
    self::state!().filepath = self::get_runtime_filepath();
}

/// Returns the path to the plugin standard data directory
///
/// Will return a dummy path if `stdpath('data')` throws an error, along with a notification
pub(crate) fn standard_data_directory() -> &'static PathBuf {
    use std::sync::LazyLock;
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

/// Returns the runtime configurations file path for the current working directory
///
/// Will return a dummy path if `std::env::current_dir()` throws an error, along with a notification
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
    self::standard_data_directory().join(json_filename)
}
