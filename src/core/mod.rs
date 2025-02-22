/*-------------------------------------- CORE FUNCTIONALITY --------------------------------------*/

pub(crate) mod task;

use crate::config;
use crate::settings::state as settings;
use crate::utils::notify;
use ::nvim_oxi::Object;

pub(crate) fn setup(user_settings: Object) {
    settings!().apply(user_settings);

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

pub(crate) fn task_() {
    let config_json = { config::state!().list[0].clone() };

    if let Err(e) = task::run(config_json.into()) {
        notify::send!(Warn: {format!("{e}")});
    }
}

pub(crate) fn debugger() {
    notify::send!(Info: "Debugger launched!");
}
