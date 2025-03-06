/*------------------------------------ CONFIGURATION LAUNCHER ------------------------------------*/

mod select;

use crate::utils;
use ::nvim_oxi::api as nvim;

utils::setup_module_state!(launcher_enum, Status);

#[derive(Debug)]
enum Status {
    Closed,
    Select,
}

impl Default for Status {
    fn default() -> Self {
        Self::Closed
    }
}

pub(crate) fn open() -> utils::Result<()> {
    self::state!().update(Status::Select)
}

impl Status {
    fn update(&mut self, to: Status) -> utils::Result<()> {
        use ::nvim_oxi::api::opts::OptionOpts;
        use Status::*;

        match (&self, &to) {
            (Closed, Select) => {
                let buffer = nvim::create_buf(false, true)?;
                let opts = OptionOpts::builder().buffer(buffer.clone()).build();
                nvim::set_option_value("filetype", "launch_nvim_launcher", &opts)?;
                let window = utils::open_float("Task Launcher", &buffer, 1, 1)?;
                select::enter(buffer, window)?;
            },
            (Select, Closed) => {
                select::exit();
            },
            _ => {},
        }

        *self = to;
        Ok(())
    }
}
