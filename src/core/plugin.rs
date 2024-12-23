/*--------------------------------------- PLUGIN STATE-API ---------------------------------------*/

use super::task;
use crate::settings::Settings;
use crate::utils::notify;
use ::nvim_oxi::Object;

pub(crate) struct Plugin {
    pub(crate) settings: Settings,
}

impl Plugin {
    pub(crate) const fn new() -> Self {
        Self { settings: Settings::new() }
    }

    pub(crate) fn setup(&mut self, user_settings: Object) {
        self.settings.apply(user_settings);
        // ::nvim_oxi::dbg!(&self.settings);
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

        match task::run(&self.settings.task, config) {
            Ok(_) => notify::send!(Info: "Task launched!"),
            Err(e) => notify::send!(Warn: {format!("{e}")}),
        };
    }

    pub(crate) fn debugger(&self) {
        notify::send!(Info: "Debugger launched!");
    }
}
