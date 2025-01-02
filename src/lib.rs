/*------------------------------------------ LAUNCH-NVIM -----------------------------------------*/

mod config;
mod core;
mod settings;
mod utils;

use crate::core::{plugin, task};
use ::nvim_oxi::{Dictionary, Function, Object};

#[nvim_oxi::plugin]
fn launch() -> Dictionary {
    let internal =
        Dictionary::from_iter([("on_task_bufwipeout", Function::from_fn(task::on_bufwipeout))]);

    Dictionary::from_iter::<[(&str, Object); 5]>([
        ("setup", Function::from_fn(plugin::setup).into()),
        ("task", Function::from_fn(|()| plugin::task()).into()),
        ("debugger", Function::from_fn(|()| plugin::debugger()).into()),
        (
            "show_active",
            Function::from_fn(|()| {
                ::nvim_oxi::dbg!(&plugin::state!().active_tasks);
            })
            .into(),
        ),
        ("__internal__", internal.into()),
    ])
}
