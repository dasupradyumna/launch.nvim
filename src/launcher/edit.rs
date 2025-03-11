/*------------------------------------- LAUNCHER : EDIT MODE -------------------------------------*/

use super::{action, LauncherState};
use crate::{config, utils};
use ::nvim_oxi::api::opts::OptionOpts;
use ::nvim_oxi::api::types::{WindowConfig, WindowRelativeTo};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};
use ::nvim_oxi::Array;

#[derive(Debug, Clone)]
pub(super) struct Edit {
    pub(super) buffer: Buffer,
    pub(super) window: Window,
    pub(super) index: usize,
    pub(super) from_select: bool,
}

impl TryFrom<super::Select> for Edit {
    type Error = utils::Error;

    fn try_from(select: super::Select) -> utils::Result<Self> {
        let index = action::get_config_index(&select.window)?;
        let mut new = Self {
            buffer: select.buffer,
            window: select.window,
            index,
            from_select: true,
        };
        new.setup()?;
        Ok(new)
    }
}

impl TryFrom<super::View> for Edit {
    type Error = utils::Error;

    fn try_from(view: super::View) -> utils::Result<Self> {
        let mut new = Self {
            buffer: view.buffer,
            window: view.window,
            index: view.index,
            from_select: false,
        };
        new.setup()?;
        Ok(new)
    }
}

impl LauncherState for Edit {
    fn update_ui(&mut self) -> utils::Result<()> {
        // Create edit-mode buffer content
        let config = &config::state!().list[self.index];
        const NONE: &str = "---";
        let mut lines = Vec::new();
        lines.push(format!("NAME : {}", config.name()));
        lines.push(format!("CMD  : {}", config.command()));
        lines.push("ARGS :".to_string());
        if let Some(args) = config.args() {
            lines.extend(args.iter().map(|arg| format!("  - {arg}")));
        }
        lines.push("  + Add new".to_string());
        let fmt = match config.display() {
            Some(d) => d.to_string(),
            None => NONE.to_string(),
        };
        lines.push(format!("DISP : {fmt}"));
        let fmt = match config.cwd() {
            Some(d) => d.display().to_string(),
            None => NONE.to_string(),
        };
        lines.push(format!("CWD  : {fmt}"));
        lines.push("ENV  :".to_string());
        if let Some(env) = config.env() {
            lines.extend(env.iter().map(|(var, value)| format!("  {var}={value}")));
        }
        lines.push("  + Add new".to_string());

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

        Ok(())
    }

    fn update_callbacks(&mut self) -> utils::Result<()> {
        nvim::command("call b:remove_callbacks()")?;

        use action::{wrap_callback, Event};
        let callback_dict = Array::from((
            wrap_callback("q", || super::state!().on(Event::Close)),
            wrap_callback("b", || super::state!().on(Event::Back)),
        ));
        self.buffer.set_var("callbacks", callback_dict)?;
        nvim::command("call b:setup_callbacks()")?;

        Ok(())
    }
}
