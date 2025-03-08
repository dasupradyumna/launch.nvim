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

    config::load()
        .unwrap_or_else(|err| notify::send!(Warn: {format!("parsing config file - {err}")}));
}

pub(crate) fn launch() {
    // if let Err(e) = crate::launcher::open() {
    //     notify::send!(Warn: {format!("{e}")});
    // }

    if let Err(e) = crate::launcher_s::open() {
        notify::send!(Warn: {format!("{e}")});
    }
}
