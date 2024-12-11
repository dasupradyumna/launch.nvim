/*--------------------------------------- PLUGIN STATE-API ---------------------------------------*/

use crate::settings::Settings;
use crate::utils::notify;
use ::nvim_oxi::Object;
use std::sync::{LazyLock, Mutex};

pub(crate) static STATE: LazyLock<Mutex<Plugin>> = LazyLock::new(|| Mutex::new(Plugin::new()));

macro_rules! api {
    () => {
        crate::core::plugin::STATE.lock().unwrap()
    };
}

pub(crate) use api;

pub(crate) struct Plugin {
    settings: Settings,
}

impl Plugin {
    const fn new() -> Self {
        Self { settings: Settings::new() }
    }

    pub(crate) fn setup(&mut self, user_settings: Object) {
        self.settings.apply(user_settings);
    }

    pub(crate) fn task(&self) {
        notify::send!(Info: "Task launched!");
    }

    pub(crate) fn debugger(&self) {
        notify::send!(Info: "Debugger launched!");
    }
}
