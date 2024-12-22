/*---------------------------------------- PLUGIN SETTINGS ---------------------------------------*/

use crate::utils::notify;
use ::nvim_oxi::serde::Deserializer as NvimOxiDeserializer;
use ::nvim_oxi::Object;

const WIKI_URL: &str = "https://github.com/dasupradyumna/launch.nvim/wiki/Plugin-Settings";

crate::utils::serde::setup_deserializable_structs! {
    pub(crate) Settings {
        pub(crate) confirm_choice: bool = false;
        ---
        pub(crate) task: SettingsTask;
    },
    pub(crate) SettingsTask {
        pub(crate) insert_mode_on_launch: bool = false;
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
