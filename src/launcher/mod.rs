/*------------------------------------ CONFIGURATION LAUNCHER ------------------------------------*/

mod select;
mod view;

use self::select::Select;
use self::view::View;
use crate::core::task;
use crate::{config, utils};
use ::nvim_oxi::api::opts::{BufDeleteOpts, OptionOpts};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};
use ::nvim_oxi::{Array, Function};

utils::setup_module_state!(launcher, [pub(self)] Launcher);

pub(crate) fn open() {
    self::handle_result(self::state!().on(Event::Open))
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

#[derive(Debug)]
enum Event {
    Close,
    Open,
    Delete,
    Launch,
    View,
    Back,
}

fn handle_result(result: utils::Result<()>) {
    if let Err(e) = result {
        utils::notify::send!(Warn: {format!("{e}")});

        let mut state = self::state!();
        match &mut *state {
            Launcher::Select(Select { buffer, .. }) | Launcher::View(View { buffer, .. }) => {
                let _ = buffer.clone().delete(&BufDeleteOpts::builder().force(true).build());
                *state = Launcher::Closed;
            },
            _ => {},
        }
    }
}

fn wrap_callback<F>(key: &str, func: F) -> Array
where
    F: Fn() -> utils::Result<()> + 'static,
{
    Array::from((key, Function::from_fn(move |()| self::handle_result(func()))))
}

fn get_config_index(window: &Window) -> utils::Result<usize> {
    if config::state!().list.is_empty() {
        // FIX: this should not be an error, since it is a valid state for the launcher
        return utils::Error::new("No active configurations found.");
    }

    Ok(window.get_cursor()?.0 - 2)
}

fn close(buffer: Buffer) -> utils::Result<()> {
    buffer.delete(&BufDeleteOpts::builder().force(true).build())?;
    config::save()
}

impl Launcher {
    fn on(&mut self, event: Event) -> utils::Result<()> {
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
                self::close(buffer)?;
                Self::Closed
            },

            (Self::Select(mut select), Event::Delete) => {
                let index = self::get_config_index(&select.window)?;
                {
                    config::state!().list.remove(index);
                }
                config::save()?;

                select.setup()?;
                Self::Select(select)
            },

            (Self::Select(Select { buffer, window }), Event::Launch) => {
                let index = self::get_config_index(&window)?;
                self::close(buffer)?;

                let config = config::state!().list[index].clone().into();
                ::nvim_oxi::dbg!(&config);
                task::run(config)?;

                Self::Closed
            },

            (Self::Select(Select { buffer, window }), Event::View) => {
                let index = self::get_config_index(&window)?;

                let mut view = View { buffer, window, index };
                view.setup()?;
                Self::View(view)
            },

            (Self::View(View { buffer, window, .. }), Event::Back) => {
                let mut select = Select { buffer, window };
                select.setup()?;
                Self::Select(select)
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
