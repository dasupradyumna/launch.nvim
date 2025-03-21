/*------------------------------------ LAUNCHER : SELECT MODE ------------------------------------*/

use super::{action, Edit, LauncherState, View};
use crate::config;
use crate::utils::Result;
use ::nvim_oxi::api::{self as nvim, Buffer, Window};

#[derive(Debug, Clone)]
pub(super) struct Select {
    pub(super) buffer: Buffer,
    pub(super) window: Window,
}

impl Select {
    pub(super) fn into_view(self) -> Result<View> {
        config::clear_undo_in_buffer()?;
        let index = action::get_config_index(&self.window)?;
        let mut view = View {
            buffer: self.buffer,
            window: self.window,
            index,
        };
        view.setup()?;
        Ok(view)
    }

    pub(super) fn into_edit(self, index: usize) -> Result<Edit> {
        config::clear_undo_in_buffer()?;
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

    fn create_contents(&self) -> Result<Vec<String>> {
        let configs = &config::state!().list;
        let lines = if configs.is_empty() {
            Vec::from_iter([config::NO_CONFIGS_MSG.into()])
        } else {
            configs.iter().map(|c| c.name().into()).collect()
        };

        Ok(lines)
    }

    super::setup_callbacks! {
        ("a", Add),
        ("q", Close),
        ("c", Copy),
        ("d", Delete),
        ("e", Edit),
        ("<CR>", Launch),
        ("U", Redo),
        ("u", Undo),
        ("v", View),
    }
}
