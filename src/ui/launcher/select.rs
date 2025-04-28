/*------------------------------------ LAUNCHER : SELECT MODE ------------------------------------*/

use super::{Edit, LauncherState, View};
use crate::config;
use crate::ui::utils::index_from_cursor;
use crate::utils::{nvim_set_local, Result};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};

#[derive(Debug, Clone)]
pub(super) struct Select {
    pub(super) buffer: Buffer,
    pub(super) window: Window,
}

impl Select {
    pub(super) fn into_view(self) -> Result<View> {
        config::buffer::clear_undo_history()?;
        let index = index_from_cursor(&self.window)?;
        let mut view = View {
            buffer: self.buffer,
            window: self.window,
            index,
        };
        view.setup()?;
        Ok(view)
    }

    pub(super) fn into_edit(self, index: usize) -> Result<Edit> {
        config::buffer::clear_undo_history()?;
        let mut edit = Edit {
            buffer: self.buffer,
            window: self.window,
            index,
            from_select: true,
        };
        edit.setup()?;
        Ok(edit)
    }
}

impl LauncherState for Select {
    super::setup_getters!();

    fn create_contents(&self) -> Vec<String> {
        const NO_CONFIGS_MSG: &str = "-- No active configs --";

        let configs = &config::state!().tasks;
        if configs.is_empty() {
            vec![NO_CONFIGS_MSG.into()]
        } else {
            configs.iter().map(|c| c.name().into()).collect()
        }
    }

    fn update_ui(&mut self) -> Result<()> {
        self.__update_ui()?;
        nvim_set_local(&self.window, "cursorline", !config::state!().tasks.is_empty())
    }

    super::setup_callbacks! {
        ("a", Add, "Add new config"),
        ("q", Close, "Close launcher"),
        ("c", Copy, "Copy config"),
        ("d", Delete, "Delete config"),
        ("e", Edit, "Edit config"),
        ("h", Help, "Open help"),
        ("<CR>", Launch, "Launch config"),
        ("U", Redo, "Redo action"),
        ("u", Undo, "Undo action"),
        ("v", View, "View config"),
    }
}
