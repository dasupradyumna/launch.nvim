/*--------------------------------------- PLUGIN STATE-API ---------------------------------------*/

use super::task::{self};
use crate::config::{self, TaskConfigJson};
use crate::settings::Settings;
use crate::utils::notify;
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
    pub(crate) configs: Vec<TaskConfigJson>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            runtime_file: config::get_runtime_filepath(),
            settings: Settings::new(),
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

pub(crate) fn launch() {
    let _ = crate::launcher::open();
}

pub(crate) fn task() {
    let config_json = { state!().configs[0].clone() };

    if let Err(e) = task::run(config_json.into()) {
        notify::send!(Warn: {format!("{e}")});
    }
}

pub(crate) fn debugger() {
    notify::send!(Info: "Debugger launched!");
}
