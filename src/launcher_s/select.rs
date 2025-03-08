/*------------------------------------ LAUNCHER : SELECT MODE ------------------------------------*/

use crate::core::task;
use crate::{config, utils};
use ::nvim_oxi::api::opts::{BufDeleteOpts, OptionOpts};
use ::nvim_oxi::api::types::{WindowConfig, WindowRelativeTo};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};
use ::nvim_oxi::{Array, Function};

#[derive(Debug)]
pub(super) struct Select {
    pub(super) buffer: Buffer,
    pub(super) window: Window,
}

impl super::LauncherState for Select {
    fn update_ui(&mut self) -> utils::Result<()> {
        let configs = &config::state!().list;

        // Display all configurations in the buffer
        let range = 1..self.buffer.line_count()?;
        let lines = if configs.is_empty() {
            Vec::from_iter([config::NO_CONFIGS_MSG])
        } else {
            configs.iter().map(|c| c.name().as_str()).collect()
        };
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
        let width = lines.iter().map(|l| l.len()).max().unwrap() as u32 + 8;
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
        Ok(())
    }
}
