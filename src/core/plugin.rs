/*--------------------------------------- PLUGIN STATE-API ---------------------------------------*/

use super::task;
use crate::settings::Settings;
use crate::utils::notify;
use ::nvim_oxi::Object;
use std::sync::{LazyLock, Mutex};

pub(crate) static STATE: LazyLock<Mutex<Plugin>> = LazyLock::new(|| Mutex::new(Plugin::new()));
macro_rules! state {
    () => {{
        crate::core::plugin::STATE.lock().unwrap()
    }};
}
pub(crate) use state;

pub(crate) struct Plugin {
    pub(crate) settings: Settings,
    pub(crate) active_tasks: Vec<task::ActiveTask>,
}

impl Plugin {
    pub(crate) const fn new() -> Self {
        Self {
            settings: Settings::new(),
            active_tasks: Vec::new(),
        }
    }
}

pub(crate) fn setup(user_settings: Object) {
    let settings = &mut state!().settings;

    settings.apply(user_settings);
    ::nvim_oxi::dbg!(settings);
}

pub(crate) fn task() {
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
        Ok(active_task) => {
            notify::send!(Info: "Task launched!");
            state!().active_tasks.push(active_task);
        },
        Err(e) => notify::send!(Warn: {format!("{e}")}),
    };
}

pub(crate) fn debugger() {
    notify::send!(Info: "Debugger launched!");
}
