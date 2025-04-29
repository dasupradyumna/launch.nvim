/*------------------------------------ CONFIGURATION LAUNCHER ------------------------------------*/
//!
//! This modules provides the event-driven state machine implementation for the configuration
//! launcher UI. It handles the state transitions between different states of the UI.
//!
//! ## Submodules
//! - [`action`] - contains items related to user actions in the launcher UI
//! - [`edit`] - contains items related to the edit mode of the launcher UI
//! - [`select`] - contains items related to the select mode of the launcher UI
//! - [`view`] - contains items related to the view mode of the launcher UI

mod action;
mod edit;
mod select;
mod view;

use self::edit::Edit;
use self::select::Select;
use self::view::View;
use super::utils::{index_from_cursor, show_help, NAVIGATION_OFFSET};
use crate::config;
use crate::utils::{buffer, float, notify, nvim_set_local, setup_module_state, Error, Result};
use ::nvim_oxi::api::{Buffer, Window};

setup_module_state!(ui::launcher, Launcher);

/// Opens the configuration launcher UI, with error handling
pub(crate) fn open() {
    let result = { self::state!().on(action::Event::Open) };
    action::result_handler(result);
}

/// Callback logic for `DirChanged` event
pub(crate) fn on_dirchanged() {
    let result = { self::state!().on(action::Event::Close) };
    action::result_handler(result);
}

/// The event-driven state machine for the launcher UI
///
/// ## Possible states
/// - `Closed`: UI is closed
/// - `Select`: UI displays a list of task configurations
/// - `View`: UI displays the specified task configuration
/// - `Edit`: UI allows the user to edit the specified task configuration
#[derive(Debug, Clone)]
enum Launcher {
    Closed,
    Select(Select),
    View(View),
    Edit(Edit),
}

/// Default Launcher state is `Closed`
impl Default for Launcher {
    fn default() -> Self {
        Self::Closed
    }
}

impl Launcher {
    /// Handles triggered events and transitions to the next state
    fn on(&mut self, event: action::Event) -> Result<()> {
        use action::Event;

        let num_configs = { config::state!().tasks.len() };

        let current = self.clone();
        let next = match (current, &event) {
            /*-------------------------------- OPEN-CLOSE --------------------------------*/
            // Open the launcher
            (Self::Closed, Event::Open) => {
                config::buffer::create()?;
                config::buffer::read_from_file()?;

                let buffer = buffer::create_scratch("launcher")?;
                let window = float::open_centered("Task Launcher", &buffer, 1, 1)?;

                let mut select = Select { buffer, window };
                select.setup()?;
                Self::Select(select)
            },

            // Close the launcher
            (Self::Select(Select { buffer, .. }), Event::Close)
            | (Self::View(View { buffer, .. }), Event::Close)
            | (Self::Edit(Edit { buffer, .. }), Event::Close) => {
                action::close_launcher(buffer)?;
                Self::Closed
            },

            // Ignore `Open` on all states other than `Closed`, and `Close` on `Closed` state
            (state, Event::Open | Event::Close) => state,

            /*--------------------------------- SHOW-HELP --------------------------------*/
            (Self::Select(select), Event::Help) => {
                show_help(&select.buffer, &select.window)?;
                Self::Select(select)
            },
            (Self::View(view), Event::Help) => {
                show_help(&view.buffer, &view.window)?;
                Self::View(view)
            },
            (Self::Edit(edit), Event::Help) => {
                show_help(&edit.buffer, &edit.window)?;
                Self::Edit(edit)
            },

            /*-------------------------------- SELECT MODE -------------------------------*/
            // Handle the case when there are no task configurations
            (
                Self::Select(select),
                Event::Copy | Event::Delete | Event::Edit | Event::Launch | Event::View,
            ) if num_configs == 0 => {
                notify!(Warn: "No task configurations found.");
                Self::Select(select)
            },

            // Add a new task configuration
            (Self::Select(select), Event::Add) => {
                action::add_config()?;
                Self::Edit(select.into_edit(num_configs)?)
            },

            // Copy the currently selected task configuration
            (Self::Select(select), Event::Copy) => {
                action::copy_config(index_from_cursor(&select.window)?)?;
                Self::Edit(select.into_edit(num_configs)?)
            },

            // Delete the currently selected task configuration
            (Self::Select(mut select), Event::Delete) => {
                action::delete_config(index_from_cursor(&select.window)?)?;
                select.setup()?;
                Self::Select(select)
            },

            // Edit the currently selected task configuration
            (Self::Select(select), Event::Edit) => {
                let index = index_from_cursor(&select.window)?;
                Self::Edit(select.into_edit(index)?)
            },

            // Launch the currently selected task configuration
            (Self::Select(Select { buffer, window }), Event::Launch) => {
                action::launch_config(buffer, index_from_cursor(&window)?)?;
                Self::Closed
            },

            // Redo the last undo action
            (Self::Select(mut select), Event::Redo) => {
                action::redo_action(&mut select, true)?;
                Self::Select(select)
            },

            // Undo the last action
            (Self::Select(mut select), Event::Undo) => {
                action::undo_action(&mut select, true)?;
                Self::Select(select)
            },

            // View the currently selected task configuration
            (Self::Select(select), Event::View) => Self::View(select.into_view()?),

            /*--------------------------------- VIEW MODE --------------------------------*/
            // Go back to the select mode
            (Self::View(view), Event::Back) => Self::Select(view.into_select()?),

            // Copy the displayed task configuration
            (Self::View(view), Event::Copy) => {
                action::copy_config(view.index)?;
                Self::Edit(view.into_edit(num_configs)?)
            },

            // Delete the displayed task configuration
            (Self::View(view), Event::Delete) => {
                action::delete_config(view.index)?;
                Self::Select(view.into_select()?)
            },

            // Edit the displayed task configuration
            (Self::View(view), Event::Edit) => {
                let index = view.index;
                Self::Edit(view.into_edit(index)?)
            },

            // Launch the displayed task configuration
            (Self::View(View { buffer, index, .. }), Event::Launch) => {
                action::launch_config(buffer, index)?;
                Self::Closed
            },

            /*--------------------------------- EDIT MODE --------------------------------*/
            // Go back to the previous mode, with or without saving
            (Self::Edit(edit), event @ (Event::Back | Event::Save)) => {
                match event {
                    Event::Save => config::buffer::write_to_file()?,
                    Event::Back => config::buffer::read_from_file()?,
                    _ => (),
                }

                // HACK: Workaround for below issue -
                // https://github.com/dasupradyumna/launch.nvim/issues/49#issuecomment-2833621644
                if edit.from_select || edit.index == { config::state!().tasks.len() } {
                    Self::Select(edit.into_select()?)
                } else {
                    Self::View(edit.into_view()?)
                }
            },

            // Delete the currently highlighted field
            (Self::Edit(mut edit), Event::Delete) => {
                action::delete_field(&mut edit)?;
                Self::Edit(edit)
            },

            // Edit the currently highlighted field
            (Self::Edit(edit), Event::Edit) => {
                action::edit_field(edit.clone())?;
                Self::Edit(edit)
            },

            // Insert a new argument in the current task configuration
            (Self::Edit(edit), Event::InsertArg) => {
                action::insert_arg(edit.clone())?;
                Self::Edit(edit)
            },

            // Redo the last undone action
            (Self::Edit(mut edit), Event::Redo) => {
                action::redo_action(&mut edit, false)?;
                Self::Edit(edit)
            },

            // Undo the last action
            (Self::Edit(mut edit), Event::Undo) => {
                action::undo_action(&mut edit, false)?;
                Self::Edit(edit)
            },

            // Raise an error for unsupported transitions
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

/// Trait defining the behaviour for the different states of the launcher UI
trait LauncherState {
    /// Returns a mutable reference to the current state buffer
    fn buf(&mut self) -> &mut Buffer;
    /// Returns a mutable reference to the current state window
    fn win(&mut self) -> &mut Window;
    /// Create and return the contents for the current state
    fn create_contents(&self) -> Vec<String>;
    /// Update the callbacks for the current state
    fn update_callbacks(&mut self) -> Result<()>;

    /// Custom update UI behavior ; can be overridden for different states
    fn update_ui(&mut self) -> Result<()> {
        self.__update_ui()
    }

    /*---------------- BELOW METHODS SHOULD NOT BE RE-IMPLEMENTED ----------------*/

    /// Setup the UI for the current state
    fn setup(&mut self) -> Result<()> {
        self.update_ui()?;
        self.win().set_cursor(NAVIGATION_OFFSET, 0)?;
        self.update_callbacks()
    }

    /// Common update UI implementation
    fn __update_ui(&mut self) -> Result<()> {
        // Get mode-specific buffer contents and write them
        let lines: Vec<_> = vec!["h : open help", ""]
            .into_iter()
            .map(Into::into)
            .chain(self.create_contents())
            .collect();
        buffer::write_lines(self.buf(), &lines)?;
        self.buf().add_highlight(crate::nvim_namespace(), "Comment", 0, ..)?;

        // Modify window size to match current buffer content
        let bounds = (NAVIGATION_OFFSET as u32, lines.len() as u32);
        self.buf().set_var("bounds", ::nvim_oxi::Array::from(bounds))?;
        let height = bounds.1 + 1;
        let width = unsafe { lines.iter().map(|l| l.len() + 8).max().unwrap_unchecked() as u32 };
        let (row, col) = float::get_centered_position(width, height)?;
        let win_config = float::config_builder(row, col, width, height).build();
        self.win().set_config(&win_config)?;
        nvim_set_local(self.win(), "cursorline", true)?;

        Ok(())
    }
}

/// Helper macro for defining getter methods
macro_rules! setup_getters {
    () => {
        fn buf(&mut self) -> &mut Buffer {
            &mut self.buffer
        }

        fn win(&mut self) -> &mut Window {
            &mut self.window
        }
    };
}
use setup_getters;

/// Helper macro for defining state callbacks
macro_rules! setup_callbacks {
    { $( ($key:expr, $event:ident, $desc:expr) ,)* } => {

        fn update_callbacks(&mut self) -> crate::utils::Result<()> {
            use crate::ui::launcher::{action, state};
            use crate::ui::utils::wrap_cb;
            use ::nvim_oxi::Array;

            nvim::command("call b:remove_callbacks()")?;
            let action_list = Array::from((
                $( Array::from(($key, $desc, wrap_cb(
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
