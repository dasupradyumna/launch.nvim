/*--------------------------------------- PLUGIN STATE-API ---------------------------------------*/

use super::task::{self, ActiveTask};
use crate::config::{self, TaskConfigJson};
use crate::settings::Settings;
use crate::utils::notify;
use ::nvim_oxi::api::Window;
use ::nvim_oxi::Object;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};

pub(crate) static STATE: LazyLock<Mutex<State>> = LazyLock::new(Mutex::default);
macro_rules! state {
    () => {{
        crate::core::plugin::STATE.lock().unwrap()
    }};
}
pub(crate) use state;

#[derive(Debug)]
pub(crate) struct State {
    pub(crate) runtime_file: PathBuf,
    pub(crate) settings: Settings,
    pub(crate) task: StateTask,
    pub(crate) configs: Vec<TaskConfigJson>,
}

#[derive(Debug)]
pub(crate) struct StateTask {
    pub(crate) active_list: Vec<ActiveTask>,
    pub(crate) windows: [Option<Window>; 3],
}

impl Default for State {
    fn default() -> Self {
        Self {
            runtime_file: config::get_runtime_filepath(),
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
    {
        let settings = &mut state!().settings;
        settings.apply(user_settings);
    }

    let data_dir = config::get_data_dir();
    ::nvim_oxi::dbg!(data_dir);
    std::fs::create_dir_all(data_dir)
        .unwrap_or_else(|err| notify::send!(Error: {format!("creating data dir - {err}")}));

    if let Err(e) = config::load() {
        notify::send!(Warn: {format!("parsing config file - {e}")});
    }
}

pub(crate) fn task() {
    let config_json = { state!().configs[0].clone() };

    match task::run(config_json.into()) {
        Ok(active_task) => {
            state!().task.active_list.push(active_task);
        },
        Err(e) => notify::send!(Warn: {format!("{e}")}),
    };
}

pub(crate) fn debugger() {
    notify::send!(Info: "Debugger launched!");
}
