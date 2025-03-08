/*------------------------------------ CONFIGURATION LAUNCHER ------------------------------------*/

mod select;

use crate::utils;
use ::nvim_oxi::api as nvim;
use ::nvim_oxi::api::opts::OptionOpts;

utils::setup_module_state!(launcher_s, [pub(self)] Launcher);

pub(crate) fn open() -> utils::Result<()> {
    self::state!().on(Event::Open)
}

#[derive(Debug)]
enum Launcher {
    Closed,
    Select(select::Select),
}

impl Default for Launcher {
    fn default() -> Self {
        Self::Closed
    }
}

#[derive(Debug)]
enum Event {
    Close,
    Open,
    Delete,
    Launch,
}

impl Launcher {
    fn on(&mut self, event: Event) -> utils::Result<()> {
        let new_state = match (&self, &event) {
            (Launcher::Closed, Event::Open) => {
                let buffer = nvim::create_buf(false, true)?;
                let opts = OptionOpts::builder().buffer(buffer.clone()).build();
                nvim::set_option_value("filetype", "launch_nvim_launcher", &opts)?;
                let window = utils::open_float("Task Launcher", &buffer, 1, 1)?;

                let mut inner_state = select::Select { buffer, window };
                setup_state(&mut inner_state)?;
                Launcher::Select(inner_state)
            },
            _ => {
                // TODO: error handling and exit logic
                Launcher::Closed
            },
        };

        *self = new_state;
        Ok(())
    }
}

fn setup_state<S: LauncherState>(state: &mut S) -> utils::Result<()> {
    state.update_ui()?;
    state.update_callbacks()?;

    Ok(())
}

trait LauncherState {
    fn update_ui(&mut self) -> utils::Result<()>;
    fn update_callbacks(&mut self) -> utils::Result<()>;
}
