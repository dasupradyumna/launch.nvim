/*------------------------------------------ LAUNCH-NVIM -----------------------------------------*/

mod config;
mod core;
mod settings;
mod utils;

use crate::core::plugin;
use ::nvim_oxi::{Dictionary, Function};

#[nvim_oxi::plugin]
fn launch() -> Dictionary {
    Dictionary::from_iter([
        ("setup", Function::from_fn(|settings| plugin::setup(settings))),
        ("task", Function::from_fn(|_| plugin::task())),
        ("debugger", Function::from_fn(|_| plugin::debugger())),
        (
            "show_active",
            Function::from_fn(|_| {
                ::nvim_oxi::dbg!(&plugin::state!().active_tasks);
            }),
        ),
    ])
}
