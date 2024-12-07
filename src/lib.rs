/*------------------------------------------ LAUNCH-NVIM -----------------------------------------*/

mod settings;

use std::cell::{LazyCell, RefCell};

use ::nvim_oxi::lua::print;
use ::nvim_oxi::{Dictionary, Function};

use crate::settings::Settings;

#[derive(Debug)]
struct PluginState {
    settings: Settings,
}

impl PluginState {
    fn new() -> PluginState {
        PluginState { settings: Settings::new() }
    }
}

static mut STATE: LazyCell<RefCell<PluginState>> =
    LazyCell::new(|| RefCell::new(PluginState::new()));

#[nvim_oxi::plugin]
fn launch() -> Dictionary {
    let mut api = Dictionary::new();

    api.insert("setup", Function::from_fn(setup));
    api.insert("task", Function::from_fn(|()| task()));
    api.insert("debugger", Function::from_fn(|()| debugger()));

    api
}

fn setup(user_settings: Dictionary) {
    settings::apply(&user_settings);

    unsafe {
        print!("State settings: {:?}", STATE.borrow().settings);
    }
}

fn task() {
    print!("Task launched!");
}

fn debugger() {
    print!("Debugger launched!");
}
