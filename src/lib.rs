/*------------------------------------------ LAUNCH-NVIM -----------------------------------------*/

mod config;
mod core;
mod settings;
mod utils;

use crate::core::plugin::plugin as api;
use ::nvim_oxi::{Dictionary, Function};

#[nvim_oxi::plugin]
fn launch() -> Dictionary {
    Dictionary::from_iter([
        ("setup", Function::from_fn(|settings| api!().setup(settings))),
        ("task", Function::from_fn(|_| api!().task())),
        ("debugger", Function::from_fn(|_| api!().debugger())),
    ])
}
