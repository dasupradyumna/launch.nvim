/*------------------------------------------ LAUNCH-NVIM -----------------------------------------*/

use nvim_oxi::lua::print;
use nvim_oxi::{Dictionary, Function};

mod settings;

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
        let Some(ref active) = settings::ACTIVE_SETTINGS else {
            panic!("")
        };
        print!("Active settings: {:?}", active);
    }
}

fn task() {
    print!("Task launched!");
}

fn debugger() {
    print!("Debugger launched!");
}
