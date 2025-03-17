/*-------------------------------------- CORE FUNCTIONALITY --------------------------------------*/

pub(crate) mod task;

use crate::settings::state as settings;
use crate::{config, utils};
use ::nvim_oxi::Object;

pub(crate) fn setup(user_settings: Object) {
    settings!().apply(user_settings);

    let data_dir = config::get_data_dir();
    ::nvim_oxi::dbg!(data_dir);
    std::fs::create_dir_all(data_dir)
        .unwrap_or_else(|err| utils::notify!(Error: format!("creating data dir - {err}")));
}
