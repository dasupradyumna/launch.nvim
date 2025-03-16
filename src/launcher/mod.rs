/*------------------------------------ CONFIGURATION LAUNCHER ------------------------------------*/

mod action;
mod edit;
mod select;
mod view;

use self::edit::Edit;
use self::select::Select;
use self::view::View;
use crate::{config, utils};
use ::nvim_oxi::api::opts::OptionOpts;
use ::nvim_oxi::api::{self as nvim, Buffer, Window};
use ::nvim_oxi::Array;

utils::setup_module_state!(launcher, [pub(self)] Launcher);

pub(crate) fn open() {
    if let Err(e) = config::setup_buffer_and_configs() {
        utils::notify::send!(Warn: {format!("config::load failed - {e}")});
        return;
    }

    action::handle_result(self::state!().on(action::Event::Open))
}

#[derive(Debug, Clone)]
enum Launcher {
    Closed,
    Select(Select),
    View(View),
    Edit(Edit),
}

impl std::fmt::Display for Launcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self {
            Self::Closed => "Closed",
            Self::Select(_) => "Select",
            Self::View(_) => "View",
            Self::Edit(_) => "Edit",
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

        let current = self.clone();
        let next = match (current, &event) {
            (Self::Closed, Event::Open) => {
                let buffer = nvim::create_buf(false, true)?;
                let opts = OptionOpts::builder().buffer(buffer.clone()).build();
                nvim::set_option_value("filetype", "launch_nvim_launcher", &opts)?;
                let window = utils::open_float("Task Launcher", &buffer, 1, 1)?;

                let mut select = Select { buffer, window };
                select.setup()?;
                let opts = OptionOpts::builder().win(select.window.clone()).build();
                nvim::set_option_value("cursorline", !config::state!().list.is_empty(), &opts)?;
                Self::Select(select)
            },

            (Self::Select(Select { buffer, .. }), Event::Close)
            | (Self::View(View { buffer, .. }), Event::Close)
            | (Self::Edit(Edit { buffer, .. }), Event::Close) => {
                action::close(buffer)?;
                Self::Closed
            },

            /*-------------------------------- SELECT MODE -------------------------------*/
            (Self::Select(mut select), Event::Delete) => {
                action::delete(action::get_config_index(&select.window)?)?;
                select.setup()?;
                let opts = OptionOpts::builder().win(select.window.clone()).build();
                nvim::set_option_value("cursorline", !config::state!().list.is_empty(), &opts)?;
                Self::Select(select)
            },

            (Self::Select(select), Event::Edit) => Self::Edit(select.try_into()?),

            (Self::Select(Select { buffer, window }), Event::Launch) => {
                action::launch(buffer, action::get_config_index(&window)?)?;
                Self::Closed
            },

            (Self::Select(select), Event::View) => Self::View(select.try_into()?),

            /*--------------------------------- VIEW MODE --------------------------------*/
            (Self::View(view), Event::Back) => Self::Select(view.try_into()?),

            (Self::View(view), Event::Delete) => {
                action::delete(view.index)?;
                Self::Select(view.try_into()?)
            },

            (Self::View(view), Event::Edit) => Self::Edit(view.try_into()?),

            (Self::View(View { buffer, index, .. }), Event::Launch) => {
                action::launch(buffer, index)?;
                Self::Closed
            },

            /*--------------------------------- EDIT MODE --------------------------------*/
            (Self::Edit(edit), Event::Back) => {
                config::write_buffer()?;
                if edit.from_select {
                    Self::Select(edit.try_into()?)
                } else {
                    Self::View(edit.try_into()?)
                }
            },

            (Self::Edit(mut edit), Event::Delete) => {
                action::delete_field(&mut edit)?;
                Self::Edit(edit)
            },

            (Self::Edit(edit), Event::Edit) => {
                action::edit_field(edit.clone())?;
                Self::Edit(edit)
            },

            (Self::Edit(edit), Event::InsertArg) => {
                action::insert_arg(edit.clone())?;
                Self::Edit(edit)
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
    fn buffer(&mut self) -> &mut Buffer;
    fn window(&mut self) -> &mut Window;
    fn create_contents(&self) -> utils::Result<Vec<String>>;
    fn update_callbacks(&mut self) -> utils::Result<()>;

    /*---------------- BELOW METHODS SHOULD NOT BE RE-IMPLEMENTED ----------------*/

    fn setup(&mut self) -> utils::Result<()> {
        self.update_ui()?;
        self.window().set_cursor(2, 0)?;
        self.update_callbacks()
    }

    fn update_ui(&mut self) -> utils::Result<()> {
        let lines = self.create_contents()?;

        // Display buffer content
        let range = 1..self.buffer().line_count()?;
        let opts = OptionOpts::builder().buffer(self.buffer().clone()).build();
        nvim::set_option_value("modifiable", true, &opts)?;
        self.buffer()
            .set_lines(range, true, lines.iter().map(|s| format!("    {s}    ")))?;
        nvim::set_option_value("modifiable", false, &opts)?;

        // Set navigation bounds
        let n = lines.len() as u32;
        // FIX: handle other navigation keymaps like wW, eE, bB etc.
        self.buffer().set_var("bounds", Array::from((2, n + 1)))?;

        // Modify window size to match current config list
        use ::nvim_oxi::api::types::*;
        let height = n + 2;
        let width = lines.iter().map(|l| l.len() + 8).max().unwrap() as u32;
        let (row, col) = utils::get_float_position(width, height)?;
        let win_config = WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .row(row)
            .col(col)
            .width(width)
            .height(height)
            .build();
        self.window().set_config(&win_config)?;

        Ok(())
    }
}

macro_rules! setup_getters {
    () => {
        fn buffer(&mut self) -> &mut Buffer {
            &mut self.buffer
        }

        fn window(&mut self) -> &mut Window {
            &mut self.window
        }
    };
}
use setup_getters;

macro_rules! setup_callbacks {
    { $( ($key:expr, $event: ident) ,)* } => {

        fn update_callbacks(&mut self) -> crate::utils::Result<()> {
            use crate::launcher::{action::{wrap_cb, Event}, state};
            use ::nvim_oxi::Array;

            nvim::command("call b:remove_callbacks()")?;
            let action_list = Array::from((
                $( Array::from(($key, wrap_cb(|()| state!().on(Event::$event)))) ),+
            ));
            self.buffer.set_var("callbacks", action_list)?;
            nvim::command("call b:setup_callbacks()")?;

            Ok(())
        }
    };
}
use setup_callbacks;
