/*------------------------------------ CONFIGURATION LAUNCHER ------------------------------------*/

mod action;
mod edit;
mod select;
mod view;

use self::edit::Edit;
use self::select::Select;
use self::view::View;
use super::utils::index_from_cursor;
use crate::config;
use crate::utils::{buffer, float, notify, nvim_set_local, setup_module_state, Error, Result};
use ::nvim_oxi::api::{Buffer, Window};

setup_module_state!(ui::launcher, Launcher);

pub(crate) fn open() {
    let result = { self::state!().on(action::Event::Open) };
    action::result_handler(result);
}

pub(crate) fn on_dirchanged() {
    let result = { self::state!().on(action::Event::Close) };
    action::result_handler(result);
}

#[derive(Debug, Clone)]
enum Launcher {
    Closed,
    Select(Select),
    View(View),
    Edit(Edit),
}

impl Default for Launcher {
    fn default() -> Self {
        Self::Closed
    }
}

impl Launcher {
    fn on(&mut self, event: action::Event) -> Result<()> {
        use action::Event;

        let num_configs = { config::state!().tasks.len() };

        let current = self.clone();
        let next = match (current, &event) {
            /*-------------------------------- OPEN-CLOSE --------------------------------*/
            (Self::Closed, Event::Open) => {
                config::create_buffer()?;
                config::load_configs_from_json()?;

                let buffer = buffer::create_scratch("launcher")?;
                let window = float::open_centered("Task Launcher", &buffer, 1, 1)?;

                let mut select = Select { buffer, window };
                select.setup()?;
                Self::Select(select)
            },

            (Self::Select(Select { buffer, .. }), Event::Close)
            | (Self::View(View { buffer, .. }), Event::Close)
            | (Self::Edit(Edit { buffer, .. }), Event::Close) => {
                action::close_launcher(buffer)?;
                Self::Closed
            },

            (state, Event::Open | Event::Close) => state,

            /*-------------------------------- SELECT MODE -------------------------------*/
            (
                Self::Select(select),
                Event::Copy | Event::Delete | Event::Edit | Event::Launch | Event::View,
            ) if num_configs == 0 => {
                notify!(Warn: "No task configurations found.");
                Self::Select(select)
            },

            (Self::Select(select), Event::Add) => {
                action::add_config()?;
                Self::Edit(select.into_edit(num_configs)?)
            },

            (Self::Select(select), Event::Copy) => {
                action::copy_config(index_from_cursor(&select.window)?)?;
                Self::Edit(select.into_edit(num_configs)?)
            },

            (Self::Select(mut select), Event::Delete) => {
                action::delete_config(index_from_cursor(&select.window)?)?;
                select.setup()?;
                Self::Select(select)
            },

            (Self::Select(select), Event::Edit) => {
                let index = index_from_cursor(&select.window)?;
                Self::Edit(select.into_edit(index)?)
            },

            (Self::Select(Select { buffer, window }), Event::Launch) => {
                action::launch_config(buffer, index_from_cursor(&window)?)?;
                Self::Closed
            },

            (Self::Select(mut select), Event::Redo) => {
                action::redo_action(&mut select, true)?;
                Self::Select(select)
            },

            (Self::Select(mut select), Event::Undo) => {
                action::undo_action(&mut select, true)?;
                Self::Select(select)
            },

            (Self::Select(select), Event::View) => Self::View(select.into_view()?),

            /*--------------------------------- VIEW MODE --------------------------------*/
            (Self::View(view), Event::Back) => Self::Select(view.into_select()?),

            (Self::View(view), Event::Copy) => {
                action::copy_config(index_from_cursor(&view.window)?)?;
                Self::Edit(view.into_edit(num_configs)?)
            },

            (Self::View(view), Event::Delete) => {
                action::delete_config(view.index)?;
                Self::Select(view.into_select()?)
            },

            (Self::View(view), Event::Edit) => {
                let index = view.index;
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
            (Self::Edit(mut edit), Event::Redo) => {
                action::redo_action(&mut edit, false)?;
                Self::Edit(edit)
            },

            (Self::Edit(mut edit), Event::Undo) => {
                action::undo_action(&mut edit, false)?;
                Self::Edit(edit)
            },

            _ => {
                return Error::new(format!(
                    "Unsupported transition requested: Event::{event:?} on Launcher::{self:?}"
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

    fn update_ui(&mut self) -> Result<()> {
        self.__update_ui()
    }

    /*---------------- BELOW METHODS SHOULD NOT BE RE-IMPLEMENTED ----------------*/

    fn setup(&mut self) -> Result<()> {
        self.update_ui()?;
        self.window().set_cursor(2, 0)?;
        self.update_callbacks()
    }

    fn __update_ui(&mut self) -> Result<()> {
        // Get mode-specific buffer contents and write them
        let lines = self.create_contents()?;
        buffer::write_lines(self.buffer(), &lines)?;

        // Modify window size to match current buffer content
        // FIX: handle other navigation keymaps like wW, eE, bB etc.
        let bounds = (2, lines.len() as u32);
        self.buffer().set_var("bounds", ::nvim_oxi::Array::from(bounds))?;
        let height = bounds.1 + 1;
        let width = unsafe { lines.iter().map(|l| l.len() + 8).max().unwrap_unchecked() as u32 };
        let (row, col) = float::get_centered_position(width, height)?;
        let win_config = float::config_builder(row, col, width, height).build();
        self.window().set_config(&win_config)?;
        nvim_set_local(self.window(), "cursorline", true)?;

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
            use crate::ui::launcher::{action, state};
            use crate::ui::utils::wrap_cb;
            use ::nvim_oxi::Array;

            nvim::command("call b:remove_callbacks()")?;
            let action_list = Array::from((
                $( Array::from(($key, wrap_cb(
                    action::result_handler,
                    |()| state!().on(action::Event::$event)
                ))) ),+
            ));
            self.buffer.set_var("callbacks", action_list)?;
            nvim::command("call launch#setup_callbacks()")?;

            Ok(())
        }
    };
}
use setup_callbacks;
