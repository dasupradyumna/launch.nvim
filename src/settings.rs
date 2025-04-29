/*---------------------------------------- PLUGIN SETTINGS ---------------------------------------*/
//!
//! This module contains structs and functions related to the user settings for the plugin

use crate::config::{TaskDisplay, TaskDisplayFloatSize};
use crate::utils::{notify, serde, setup_module_state};
use ::nvim_oxi::serde::Deserializer as NvimOxiDeserializer;

/// URL to the GitHub wiki for plugin settings
const WIKI_URL: &str = "https://github.com/dasupradyumna/launch.nvim/wiki/Plugin-Settings";

setup_module_state!(settings, [pub(crate)] Settings);

serde::setup_deserializable_structs! {

    pub(crate) Settings {
        ---
        pub(crate) task: SettingsTask;
    },

    pub(crate) SettingsTask {
        pub(crate) insert_mode_on_launch: bool = false;
        ---
        pub(crate) ui: SettingsTaskUI;
    },

    pub(crate) SettingsTaskUI {
        pub(crate) display: TaskDisplay = TaskDisplay::Float;
        pub(crate) hsplit_height: u8 = 30;
        pub(crate) vsplit_width: u8 = 50;
        ---
        pub(crate) float: SettingsTaskUIFloat;
    },

    pub(crate) SettingsTaskUIFloat {
        pub(crate) size: TaskDisplayFloatSize = TaskDisplayFloatSize::Medium;
        ---
    },

}

impl Settings {
    /// Apply user specified settings to the plugin
    ///
    /// Uses the default settings if deserialization fails
    pub(crate) fn apply(&mut self, settings: ::nvim_oxi::Object) {
        // TODO: improve error messages when deserialization fails
        // - this can probably be done by implementing visit_* methods for a base visitor that all
        //   our custom visitors will inherit
        // - need to display which field has incorrect value, and what type is actually expected

        let des = NvimOxiDeserializer::new(settings);
        match Settings::deserialize(des) {
            Ok(value) => *self = value,
            Err(err) => {
                notify!(Error: format!("\
                    Deserialization failed! {err}\n\
                    Refer to [{WIKI_URL}] for documentation.
                "));
                notify!(Warn: format!("\
                    Using default settings...\n\
                    {self:#?}
                "));
            },
        };
    }
}
