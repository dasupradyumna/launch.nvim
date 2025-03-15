/*------------------------------------ LAUNCHER : SELECT MODE ------------------------------------*/

use super::LauncherState;
use crate::{config, utils};
use ::nvim_oxi::api::opts::OptionOpts;
use ::nvim_oxi::api::{self as nvim, Buffer, Window};

#[derive(Debug, Clone)]
pub(super) struct Select {
    pub(super) buffer: Buffer,
    pub(super) window: Window,
}

impl TryFrom<super::View> for Select {
    type Error = utils::Error;

    fn try_from(view: super::View) -> utils::Result<Self> {
        let mut new = Self {
            buffer: view.buffer,
            window: view.window,
        };
        new.setup()?;
        let opts = OptionOpts::builder().win(new.window.clone()).build();
        nvim::set_option_value("cursorline", !config::state!().list.is_empty(), &opts)?;
        Ok(new)
    }
}

impl TryFrom<super::Edit> for Select {
    type Error = utils::Error;

    fn try_from(edit: super::Edit) -> utils::Result<Self> {
        let mut new = Self {
            buffer: edit.buffer,
            window: edit.window,
        };
        new.setup()?;
        Ok(new)
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
        ("q", Close),
        ("d", Delete),
        ("e", Edit),
        ("<CR>", Launch),
        ("v", View),
    }
}
