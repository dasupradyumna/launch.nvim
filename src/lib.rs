/*------------------------------------------ LAUNCH-NVIM -----------------------------------------*/

mod settings;

use crate::plugin::Plugin;
use crate::settings::Settings;
use ::nvim_oxi::{Dictionary, Function};
use std::sync::{LazyLock, Mutex};

static PLUGIN: LazyLock<Mutex<Plugin>> = plugin::new();
macro_rules! plugin {
    () => {
        PLUGIN.lock().unwrap()
    };
}

#[nvim_oxi::plugin]
fn launch() -> Dictionary {
    let mut api = Dictionary::new();

    api.insert("setup", Function::from_fn(|settings| plugin!().setup(settings)));
    api.insert("task", Function::from_fn(|()| plugin!().task()));
    api.insert("debugger", Function::from_fn(|()| plugin!().debugger()));

    api
}

mod plugin {

    use super::*;
    use ::nvim_oxi::lua::print;

    pub(super) struct Plugin {
        settings: Settings,
    }

    pub(super) const fn new() -> LazyLock<Mutex<Plugin>> {
        LazyLock::new(|| Mutex::new(Plugin::new()))
    }

    impl Plugin {
        const fn new() -> Self {
            Self { settings: Settings::new() }
        }

        pub(super) fn setup(&mut self, user_settings: Dictionary) {
            self.settings.apply(&user_settings);

            print!("{:?}", self.settings);
        }

        pub(super) fn task(&self) {
            print!("Task launched!");
        }

        pub(super) fn debugger(&self) {
            print!("Debugger launched!");
        }
    }
}
