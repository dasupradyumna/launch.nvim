/*------------------------------------------ LAUNCH-NVIM -----------------------------------------*/

mod config;
mod core;
mod launcher;
mod settings;
mod utils;

use ::nvim_oxi::{Dictionary, Function, Object};

#[nvim_oxi::plugin]
fn launch() -> Dictionary {
    let task_event_callbacks = Dictionary::from_iter([
        ("on_bufwipeout", Function::from_fn(core::task::on_bufwipeout)),
        ("on_winclosed", Function::from_fn(core::task::on_winclosed)),
    ]);
    let _impl_ = Dictionary::from_iter([("task", task_event_callbacks)]);

    Dictionary::from_iter::<[(_, Object); 4]>([
        ("setup", Function::from_fn(core::setup).into()),
        ("launch", Function::from_fn(|()| launcher::open()).into()),
        ("list_active_tasks", Function::from_fn(|()| core::task::list_active_tasks()).into()),
        ("_impl_", _impl_.into()),
    ])
}
