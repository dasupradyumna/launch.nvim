/*------------------------------------------ LAUNCH-NVIM -----------------------------------------*/

mod config;
mod core;
mod ui {
    pub(super) mod active_tasks;
    pub(super) mod launcher;
    mod utils;
}
mod settings;
mod utils;

use ::nvim_oxi::{Dictionary, Function, Object};

/// Returns the neovim namespace used by the plugin
///
/// Initialized on first call, and reused on subsequent calls
fn nvim_namespace() -> u32 {
    use ::nvim_oxi::api as nvim;
    use std::sync::LazyLock;
    static INSTANCE: LazyLock<u32> = LazyLock::new(|| nvim::create_namespace("launch.nvim"));
    *INSTANCE
}

/// Main plugin entry point
///
/// Exposes public API as a Lua dictionary, usable in neovim
/// Exposes private autocommand callbacks as `_impl_` dictionary (not for public use)
///
/// ## Examples
///
/// ```lua
/// require("launch").setup()  -- Run plugin setup
/// require("launch").open()  -- Open launcher UI
/// ```
#[nvim_oxi::plugin]
fn launch() -> Dictionary {
    let task_event_callbacks = Dictionary::from_iter::<[(_, Object); 2]>([
        ("on_bufwipeout", Function::from_fn(core::task::on_bufwipeout).into()),
        ("on_winclosed", Function::from_fn(core::task::on_winclosed).into()),
    ]);

    let dirchanged_callbacks = Dictionary::from_iter([
        ("post", Function::from_fn(|()| config::on_dirchanged())),
        (
            "pre",
            Function::from_fn(|()| {
                ui::launcher::on_dirchanged();
                core::task::on_dirchanged();
            }),
        ),
    ]);

    let _impl_ = Dictionary::from_iter([
        ("task", task_event_callbacks),
        ("dirchanged", dirchanged_callbacks),
    ]);

    Dictionary::from_iter::<[(_, Object); 4]>([
        ("setup", Function::from_fn(core::setup).into()),
        ("open", Function::from_fn(|()| ui::launcher::open()).into()),
        ("list_active_tasks", Function::from_fn(|()| ui::active_tasks::open()).into()),
        ("_impl_", _impl_.into()),
    ])
}
