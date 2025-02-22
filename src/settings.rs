/*---------------------------------------- PLUGIN SETTINGS ---------------------------------------*/

use crate::config::{TaskDisplay, TaskDisplayFloatSize};
use crate::utils::notify;
use ::nvim_oxi::serde::Deserializer as NvimOxiDeserializer;
use ::nvim_oxi::Object;

const WIKI_URL: &str = "https://github.com/dasupradyumna/launch.nvim/wiki/Plugin-Settings";

crate::utils::setup_module_state!(settings, Settings);

crate::utils::serde::setup_deserializable_structs! {

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
    pub(crate) fn apply(&mut self, settings: Object) {
        // TODO: improve error messages when deserialization fails
        // - this can probably be done by implementing visit_* methods for a base visitor that all
        //   our custom visitors will inherit
        // - need to display which field has incorrect value, and what type is actually expected

        let des = NvimOxiDeserializer::new(settings);
        match Settings::deserialize(des) {
            Ok(value) => *self = value,
            Err(err) => {
                notify::send!(Error: [
                    format!("Deserialization failed! {err}"),
                    format!("Refer to [{WIKI_URL}] for documentation."),
                ]);
                notify::send!(Warn: [
                    "Using default settings...",
                    format!("{:#?}", self).as_str(),
                ]);
            },
        };
    }
}
