/*--------------------------------------- PLUGIN STATE-API ---------------------------------------*/

use super::task::{self, ActiveTask};
use crate::config::{self, TaskConfigJson};
use crate::settings::Settings;
use crate::utils::notify;
use ::nvim_oxi::api::Window;
use ::nvim_oxi::Object;
use std::fs::File;
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
    pub(crate) runtime_file: File,
    pub(crate) settings: Settings,
    pub(crate) task: StateTask,
    pub(crate) configs: Vec<TaskConfigJson>,
}

#[derive(Debug)]
pub(crate) struct StateTask {
    pub(crate) active_list: Vec<ActiveTask>,
    pub(crate) windows: [Option<Window>; 3],
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            runtime_file: config::open_file(),
            settings: Settings::new(),
            task: StateTask {
                active_list: Vec::new(),
                windows: [const { None }; 3],
            },
            configs: Vec::new(),
        }
    }
}

pub(crate) fn setup(user_settings: Object) {
    let mut state = state!();

    state.settings.apply(user_settings);

    let data_dir = config::data_dir();
    ::nvim_oxi::dbg!(data_dir);
    std::fs::create_dir_all(data_dir)
        .unwrap_or_else(|err| notify::send!(Error: {format!("creating data dir - {err}")}));

    if let Err(e) = config::load(&mut state) {
        notify::send!(Warn: {format!("parsing config file - {e}")});
    }
}

pub(crate) fn task() {
    let config;

    // HACK: this is due mutex locking issues, design revision is required
    {
        let state = state!();
        let task_settings = &state.settings.task;
        config = state.configs[0].build_config(task_settings);
    }

    match task::run(config) {
        Ok(active_task) => state!().task.active_list.push(active_task),
        Err(e) => notify::send!(Warn: {format!("{e}")}),
    };
}

pub(crate) fn debugger() {
    notify::send!(Info: "Debugger launched!");
}
