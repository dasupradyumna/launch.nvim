/*------------------------------------ CONFIGURATION LAUNCHER ------------------------------------*/

mod action;
mod edit;
mod select;
mod view;

use self::edit::Edit;
use self::select::Select;
use self::view::View;
use crate::config;
use crate::utils::{float, setup_module_state, Error, Result};
use ::nvim_oxi::api::opts::OptionOpts;
use ::nvim_oxi::api::{self as nvim, Buffer, Window};

setup_module_state!(launcher, Launcher);

pub(crate) fn open() {
    let result = { self::state!().on(action::Event::Open) };
    action::handle_result(result);
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
    fn on(&mut self, event: action::Event) -> Result<()> {
        use action::Event;

        let current = self.clone();
        let next = match (current, &event) {
            (Self::Closed, Event::Open) => {
                config::create_buffer()?;
                config::load_configs_from_json()?;

                let buffer = nvim::create_buf(false, true)?;
                let opts = OptionOpts::builder().buffer(buffer.clone()).build();
                nvim::set_option_value("filetype", "launch_nvim_launcher", &opts)?;
                let window = float::centered("Task Launcher", &buffer, 1, 1)?;

                let mut select = Select { buffer, window };
                select.setup()?;
                let opts = OptionOpts::builder().win(select.window.clone()).build();
                nvim::set_option_value("cursorline", !config::state!().list.is_empty(), &opts)?;
                Self::Select(select)
            },

            (Self::Select(Select { buffer, .. }), Event::Close)
            | (Self::View(View { buffer, .. }), Event::Close)
            | (Self::Edit(Edit { buffer, .. }), Event::Close) => {
                action::close_launcher(buffer)?;
                Self::Closed
            },

            /*-------------------------------- SELECT MODE -------------------------------*/
            (Self::Select(select), Event::Add) => {
                let index = { config::state!().list.len() };
                action::add_config()?;
                Self::Edit(select.into_edit(index)?)
            },

            (Self::Select(select), Event::Copy) => {
                let index = { config::state!().list.len() };
                action::copy_config(action::get_config_index(&select.window)?)?;
                Self::Edit(select.into_edit(index)?)
            },

            (Self::Select(mut select), Event::Delete) => {
                action::delete_config(action::get_config_index(&select.window)?)?;
                select.setup()?;
                let opts = OptionOpts::builder().win(select.window.clone()).build();
                nvim::set_option_value("cursorline", !config::state!().list.is_empty(), &opts)?;
                Self::Select(select)
            },

            (Self::Select(select), Event::Edit) => {
                let index = action::get_config_index(&select.window)?;
                Self::Edit(select.into_edit(index)?)
            },

            (Self::Select(Select { buffer, window }), Event::Launch) => {
                action::launch_config(buffer, action::get_config_index(&window)?)?;
                Self::Closed
            },

            (Self::Select(select), Event::View) => Self::View(select.into_view()?),

            /*--------------------------------- VIEW MODE --------------------------------*/
            (Self::View(view), Event::Back) => Self::Select(view.into_select()?),

            (Self::View(view), Event::Copy) => {
                let index = { config::state!().list.len() };
                action::copy_config(action::get_config_index(&view.window)?)?;
                Self::Edit(view.into_edit(index)?)
            },

            (Self::View(view), Event::Delete) => {
                action::delete_config(view.index)?;
                Self::Select(view.into_select()?)
            },

            (Self::View(view), Event::Edit) => {
                let index = action::get_config_index(&view.window)?;
                Self::Edit(view.into_edit(index)?)
            },

            (Self::View(View { buffer, index, .. }), Event::Launch) => {
                action::launch_config(buffer, index)?;
                Self::Closed
            },

            /*--------------------------------- EDIT MODE --------------------------------*/
            (Self::Edit(edit), event @ (Event::Back | Event::Save)) => {
                match event {
                    Event::Save => config::write_buffer()?,
                    Event::Back => config::load_configs_from_json()?,
                    _ => (),
                }

                if edit.from_select {
                    Self::Select(edit.into_select()?)
                } else {
                    Self::View(edit.into_view()?)
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

            (Self::Edit(mut edit), Event::Undo) => {
                action::undo_action(&mut edit)?;
                Self::Edit(edit)
            },

            _ => {
                return Error::new(format!(
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
    fn create_contents(&self) -> Result<Vec<String>>;
    fn update_callbacks(&mut self) -> Result<()>;

    /*---------------- BELOW METHODS SHOULD NOT BE RE-IMPLEMENTED ----------------*/

    fn setup(&mut self) -> Result<()> {
        self.update_ui()?;
        self.window().set_cursor(2, 0)?;
        self.update_callbacks()
    }

    fn update_ui(&mut self) -> Result<()> {
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
        self.buffer().set_var("bounds", ::nvim_oxi::Array::from((2, n + 1)))?;

        // Modify window size to match current config list
        use ::nvim_oxi::api::types::*;
        let height = n + 2;
        let width = unsafe { lines.iter().map(|l| l.len() + 8).max().unwrap_unchecked() as u32 };
        let (row, col) = float::get_position(width, height)?;
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
