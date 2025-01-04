/*------------------------------------------ LAUNCH-NVIM -----------------------------------------*/

mod config;
mod core;
mod settings;
mod utils;

use crate::core::{plugin, task};
use ::nvim_oxi::{Dictionary, Function, Object};

#[nvim_oxi::plugin]
fn launch() -> Dictionary {
    let _impl_ = Dictionary::from_iter([
        ("on_task_bufwipeout", Function::from_fn(task::on_bufwipeout)),
        ("on_task_winclosed", Function::from_fn(task::on_winclosed)),
    ]);

    Dictionary::from_iter::<[(_, Object); 5]>([
        ("setup", Function::from_fn(plugin::setup).into()),
        ("task", Function::from_fn(|()| plugin::task()).into()),
        ("debugger", Function::from_fn(|()| plugin::debugger()).into()),
        (
            "task_state",
            Function::from_fn(|()| {
                ::nvim_oxi::dbg!(&plugin::state!().task);
            })
            .into(),
        ),
        ("_impl_", _impl_.into()),
    ])
}
