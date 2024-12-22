/*--------------------------------------- PLUGIN STATE-API ---------------------------------------*/

use super::task;
use crate::settings::Settings;
use crate::utils::notify;
use ::nvim_oxi::Object;
use std::sync::{LazyLock, Mutex};

pub(crate) static STATE: LazyLock<Mutex<Plugin>> = LazyLock::new(|| Mutex::new(Plugin::new()));

// FIX: unwrap() can panic! and crash neovim instance ; handle this gracefully
macro_rules! plugin {
    () => {
        crate::core::plugin::STATE.lock().unwrap()
    };
}

pub(crate) use plugin;

pub(crate) struct Plugin {
    pub(crate) settings: Settings,
}

// CHECK: if API can be separated as module-level functions due to crate-level state! macro
impl Plugin {
    const fn new() -> Self {
        Self { settings: Settings::new() }
    }

    pub(crate) fn setup(&mut self, user_settings: Object) {
        self.settings.apply(user_settings);
    }

    pub(crate) fn task(&self) {
        ///////////////// testing config ///////////////////////////
        use crate::config::{TaskConfig, TaskDisplay};
        use std::collections::HashMap;
        let config = TaskConfig::new(
            "Launch Test",
            "echo",
            &["\"Hey ${USR:-default_user}", "from India", "at '$PWD'!\""],
            TaskDisplay::Float,
            "/home/pradyumna/data/jira",
            HashMap::from_iter([("USR".to_string(), "Pradyu".to_string())]),
        );

        match task::run(config) {
            Ok(_) => notify::send!(Info: "Task launched!"),
            Err(e) => notify::send!(Warn: {format!("{e}")}),
        }
    }

    pub(crate) fn debugger(&self) {
        notify::send!(Info: "Debugger launched!");
    }
}
