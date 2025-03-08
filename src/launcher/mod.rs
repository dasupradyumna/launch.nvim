/*------------------------------------ CONFIGURATION LAUNCHER ------------------------------------*/

mod action;
mod select;
mod view;

use self::select::Select;
use self::view::View;
use crate::utils;
use ::nvim_oxi::api as nvim;
use ::nvim_oxi::api::opts::OptionOpts;

utils::setup_module_state!(launcher, [pub(self)] Launcher);

pub(crate) fn open() {
    action::handle_result(self::state!().on(action::Event::Open))
}

#[derive(Debug)]
enum Launcher {
    Closed,
    Select(Select),
    View(View),
}

impl std::fmt::Display for Launcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self {
            Self::Closed => "Closed",
            Self::Select(_) => "Select",
            Self::View(_) => "View",
        };
        write!(f, "Launcher::{}", fmt)
    }
}

impl Default for Launcher {
    fn default() -> Self {
        Self::Closed
    }
}

impl Launcher {
    fn on(&mut self, event: action::Event) -> utils::Result<()> {
        use action::Event;

        let next = match (std::mem::replace(self, Self::Closed), &event) {
            (Self::Closed, Event::Open) => {
                let buffer = nvim::create_buf(false, true)?;
                let opts = OptionOpts::builder().buffer(buffer.clone()).build();
                nvim::set_option_value("filetype", "launch_nvim_launcher", &opts)?;
                let window = utils::open_float("Task Launcher", &buffer, 1, 1)?;

                let mut select = Select { buffer, window };
                select.setup()?;
                Self::Select(select)
            },

            (Self::Select(Select { buffer, .. }), Event::Close)
            | (Self::View(View { buffer, .. }), Event::Close) => {
                action::close(buffer)?;
                Self::Closed
            },

            (Self::Select(mut select), Event::Delete) => {
                action::delete(action::get_config_index(&select.window)?)?;
                select.setup()?;
                Self::Select(select)
            },

            (Self::Select(Select { buffer, window }), Event::Launch) => {
                action::launch(buffer, action::get_config_index(&window)?)?;
                Self::Closed
            },

            (Self::Select(select), Event::View) => Self::View(select.try_into()?),

            (Self::View(view), Event::Back) => Self::Select(view.try_into()?),

            (Self::View(view), Event::Delete) => {
                action::delete(view.index)?;
                Self::Select(view.try_into()?)
            },

            (Self::View(View { buffer, index, .. }), Event::Launch) => {
                action::launch(buffer, index)?;
                Self::Closed
            },

            _ => {
                return utils::Error::new(format!(
                    "Unsupported transition requested: Event::{event:?} on {self}"
                ))
            },
        };

        *self = next;
        Ok(())
    }
}

trait LauncherState {
    // This method should not be re-implemented by structs
    fn setup(&mut self) -> utils::Result<()> {
        self.update_ui()?;
        self.update_callbacks()?;

        Ok(())
    }

    fn update_ui(&mut self) -> utils::Result<()>;
    fn update_callbacks(&mut self) -> utils::Result<()>;
}
