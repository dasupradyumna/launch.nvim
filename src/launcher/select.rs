/*------------------------------------ LAUNCHER : SELECT MODE ------------------------------------*/

use super::{action, LauncherState};
use crate::{config, utils};
use ::nvim_oxi::api::opts::OptionOpts;
use ::nvim_oxi::api::types::{WindowConfig, WindowRelativeTo};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};
use ::nvim_oxi::Array;

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
    fn update_ui(&mut self) -> utils::Result<()> {
        // Create select-mode buffer content
        let configs = &config::state!().list;
        let lines = if configs.is_empty() {
            Vec::from_iter([config::NO_CONFIGS_MSG])
        } else {
            configs.iter().map(|c| c.name().as_str()).collect()
        };

        // Display buffer content
        let range = 1..self.buffer.line_count()?;
        let opts = OptionOpts::builder().buffer(self.buffer.clone()).build();
        nvim::set_option_value("modifiable", true, &opts)?;
        self.buffer
            .set_lines(range, true, lines.iter().map(|s| format!("    {s}    ")))?;
        nvim::set_option_value("modifiable", false, &opts)?;

        // Set navigation bounds
        let n = lines.len() as u32;
        // FIX: handle other navigation keymaps like wW, eE, bB etc.
        self.buffer.set_var("bounds", Array::from((2, n + 1)))?;

        // Modify window size to match current config list
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
        self.window.set_config(&win_config)?;
        self.window.set_cursor(2, 0)?;
        let opts = OptionOpts::builder().win(self.window.clone()).build();
        nvim::set_option_value("cursorline", !configs.is_empty(), &opts)?;

        Ok(())
    }

    fn update_callbacks(&mut self) -> utils::Result<()> {
        nvim::command("call b:remove_callbacks()")?;

        let action_list = action::map_events! {
            ("q", Close),
            ("d", Delete),
            ("e", Edit),
            ("<CR>", Launch),
            ("v", View),
        };
        self.buffer.set_var("callbacks", action_list)?;
        nvim::command("call b:setup_callbacks()")?;

        Ok(())
    }
}
