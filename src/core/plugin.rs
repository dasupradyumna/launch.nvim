/*--------------------------------------- PLUGIN STATE-API ---------------------------------------*/

use super::task::{self, ActiveTask};
use crate::settings::Settings;
use crate::utils::notify;
use ::nvim_oxi::api::Window;
use ::nvim_oxi::Object;
use std::sync::{LazyLock, Mutex};

pub(crate) static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| Mutex::new(State::new()));
macro_rules! state {
    () => {{
        crate::core::plugin::STATE.lock().unwrap()
    }};
}
pub(crate) use state;

#[derive(Debug)]
pub(crate) struct State {
    pub(crate) settings: Settings,
    pub(crate) task: StateTask,
}

#[derive(Debug)]
pub(crate) struct StateTask {
    pub(crate) active_list: Vec<ActiveTask>,
    pub(crate) windows: [Option<Window>; 3],
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            settings: Settings::new(),
            task: StateTask {
                active_list: Vec::new(),
                windows: [const { None }; 3],
            },
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
            state!().task.active_list.push(active_task);
        },
        Err(e) => notify::send!(Warn: {format!("{e}")}),
    };
}

pub(crate) fn debugger() {
    notify::send!(Info: "Debugger launched!");
}
