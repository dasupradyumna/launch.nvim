/*------------------------------------------ LAUNCH-NVIM -----------------------------------------*/

use nvim_oxi::lua::print;
use nvim_oxi::{Dictionary, Function};

#[nvim_oxi::plugin]
fn launch() -> Dictionary {
    let mut api = Dictionary::new();

    api.insert("setup", Function::from_fn(|settings| setup(settings)));
    api.insert("task", Function::from_fn(|()| task()));
    api.insert("debugger", Function::from_fn(|()| debugger()));

    api
}

fn setup(settings: Dictionary) {
    print!("{settings:?}");
}

fn task() {
    print!("Task launched!");
}

fn debugger() {
    print!("Debugger launched!");
}
