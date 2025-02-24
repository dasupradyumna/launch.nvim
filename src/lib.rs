/*------------------------------------------ LAUNCH-NVIM -----------------------------------------*/

mod config;
mod core;
mod launcher;
mod settings;
mod utils;

use crate::core::task;
use ::nvim_oxi::{Dictionary, Function, Object};

#[nvim_oxi::plugin]
fn launch() -> Dictionary {
    let _impl_ = Dictionary::from_iter([
        ("on_task_bufwipeout", Function::from_fn(task::on_bufwipeout)),
        ("on_task_winclosed", Function::from_fn(task::on_winclosed)),
    ]);

    Dictionary::from_iter::<[(_, Object); 3]>([
        ("setup", Function::from_fn(core::setup).into()),
        ("launch", Function::from_fn(|()| core::launch()).into()),
        ("_impl_", _impl_.into()),
    ])
}
