/*------------------------------------------ LAUNCH-NVIM -----------------------------------------*/

mod settings;
mod utils;

use crate::plugin::Plugin;
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
    use crate::settings::Settings;
    use crate::utils::notify;
    use ::nvim_oxi::Object;

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

        pub(super) fn setup(&mut self, user_settings: Object) {
            self.settings.apply(user_settings);
        }

        pub(super) fn task(&self) {
            notify::send!(Info: "Task launched!");
        }

        pub(super) fn debugger(&self) {
            notify::send!(Info: "Debugger launched!");
        }
    }
}
