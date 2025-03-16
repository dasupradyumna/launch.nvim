/*------------------------------------ LAUNCHER : SELECT MODE ------------------------------------*/

use super::{action, LauncherState};
use crate::{config, utils};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};

#[derive(Debug, Clone)]
pub(super) struct Select {
    pub(super) buffer: Buffer,
    pub(super) window: Window,
}

impl Select {
    pub(super) fn into_view(self) -> utils::Result<super::View> {
        let index = action::get_config_index(&self.window)?;
        let mut view = super::View {
            buffer: self.buffer,
            window: self.window,
            index,
        };
        view.setup()?;
        Ok(view)
    }

    pub(super) fn into_edit(self, index: usize) -> utils::Result<super::Edit> {
        let mut edit = super::Edit {
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

    fn create_contents(&self) -> utils::Result<Vec<String>> {
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
        ("v", View),
    }
}
