/*---------------------------------------- PLUGIN SETTINGS ---------------------------------------*/

use crate::utils::StructVisitor;
use ::nvim_oxi::serde::Deserializer as NvimOxiDeserializer;
use ::nvim_oxi::{print as nvim_print, Object};
use ::serde::de::{Error, MapAccess, Visitor};
use ::serde::{Deserialize, Deserializer};

const WIKI_URL: &str = "https://github.com/dasupradyumna/launch.nvim/wiki/Plugin-Settings";

crate::setup_deserializable_structs! {
    pub(crate) Settings {
        confirm_choice: bool = false;
        ---
        task: SettingsTask;
    },
    SettingsTask {
        insert_mode_on_launch: bool = false;
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
                nvim_print!("Deserialization failed! {err}");
                nvim_print!("Refer to [{WIKI_URL}] for documentation.\nUsing default settings...");
            },
        }
    }
}
