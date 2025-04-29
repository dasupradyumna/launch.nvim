/*-------------------------------------- CORE FUNCTIONALITY --------------------------------------*/
//!
//! This module contains items related to the core functionality of the plugin
//!
//! ## Submodules
//! - [`task`] - contains items related to launching and managing active tasks

pub(crate) mod task;

use crate::settings::state as settings;
use crate::{config, utils};
use ::nvim_oxi::Object;

/// Sets up the plugin based on user settings
///
/// The plugin is configured by user settings, and the standard data directory is created if needed
/// Sends an error notification if the data directory cannot be created
pub(crate) fn setup(user_settings: Object) {
    settings!().apply(user_settings);

    let data_dir = config::standard_data_directory();
    std::fs::create_dir_all(data_dir).unwrap_or_else(
        |err| utils::notify!(Error: format!("Creating data dir @ {data_dir:?} - {err}")),
    );
}
