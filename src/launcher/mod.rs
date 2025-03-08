/*------------------------------------ CONFIGURATION LAUNCHER ------------------------------------*/

mod select;

use crate::core::task;
use crate::{config, utils};
use ::nvim_oxi::api::opts::{BufDeleteOpts, OptionOpts};
use ::nvim_oxi::api::{self as nvim, Window};
use ::nvim_oxi::{Array, Function};
use std::fmt::Display;

utils::setup_module_state!(launcher, [pub(self)] Launcher);

pub(crate) fn open() -> utils::Result<()> {
    // TODO: call the close behavior in case of error
    self::state!().on(Event::Open)
}

#[derive(Debug)]
enum Launcher {
    Closed,
    Select(select::Select),
}

impl Display for Launcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_str = match self {
            Self::Closed => "Closed",
            Self::Select(_) => "Select",
        };
        write!(f, "Launcher::{}", fmt_str)
    }
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

fn wrap_callback<F>(key: &str, func: F) -> Array
where
    F: Fn() -> utils::Result<()> + 'static,
{
    let wrapped = Function::from_fn(move |()| {
        if let Err(e) = func() {
            utils::notify::send!(Warn: {format!("{e}")});
        }
    });

    Array::from((key, wrapped))
}

fn get_config_index(window: &Window) -> utils::Result<usize> {
    if config::state!().list.is_empty() {
        // FIX: this should not be an error, since it is a valid state for the launcher
        return utils::Error::new("No active configurations found.");
    }

    Ok(window.get_cursor()?.0 - 2)
}

impl Launcher {
    fn on(&mut self, event: Event) -> utils::Result<()> {
        let new_state = match (&self, &event) {
            (Launcher::Closed, Event::Open) => {
                let buffer = nvim::create_buf(false, true)?;
                let opts = OptionOpts::builder().buffer(buffer.clone()).build();
                nvim::set_option_value("filetype", "launch_nvim_launcher", &opts)?;
                let window = utils::open_float("Task Launcher", &buffer, 1, 1)?;

                let mut inner = select::Select { buffer, window };
                inner.setup()?;

                Launcher::Select(inner)
            },
            (Launcher::Select(inner), Event::Close) => {
                let buffer = inner.buffer.clone();
                buffer.delete(&BufDeleteOpts::builder().force(true).build())?;
                config::save()?;

                Launcher::Closed
            },
            (Launcher::Select(inner), Event::Delete) => {
                let index = self::get_config_index(&inner.window)?;
                {
                    config::state!().list.remove(index);
                }
                config::save()?;

                let mut inner = inner.clone();
                inner.setup()?;

                Launcher::Select(inner)
            },
            (Launcher::Select(inner), Event::Launch) => {
                let index = self::get_config_index(&inner.window)?;

                // XXX: refactor into behaviors submodule
                let buffer = inner.buffer.clone();
                buffer.delete(&BufDeleteOpts::builder().force(true).build())?;
                config::save()?;

                let config = config::state!().list[index].clone().into();
                ::nvim_oxi::dbg!(&config);
                task::run(config)?;

                Launcher::Closed
            },
            _ => {
                return utils::Error::new(format!(
                    "Unsupported transition requested: Event::{event:?} on {self}"
                ))
            },
        };

        *self = new_state;
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
