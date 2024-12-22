/*------------------------------------------ LAUNCH-NVIM -----------------------------------------*/

mod config;
mod core;
mod settings;
mod utils;

use crate::core::plugin::Plugin;
use ::nvim_oxi::{Dictionary, Function};

#[nvim_oxi::plugin]
fn launch() -> Dictionary {
    // CHECK: is wrapping with a mutex required?
    static mut PLUGIN: Plugin = Plugin::new();

    Dictionary::from_iter([
        ("setup", Function::from_fn(|settings| unsafe { PLUGIN.setup(settings) })),
        ("task", Function::from_fn(|_| unsafe { PLUGIN.task() })),
        ("debugger", Function::from_fn(|_| unsafe { PLUGIN.debugger() })),
    ])
}
