/*------------------------------------- LAUNCHER : VIEW MODE -------------------------------------*/

use super::{action, LauncherState};
use crate::{config, utils};
use ::nvim_oxi::api::opts::OptionOpts;
use ::nvim_oxi::api::types::{WindowConfig, WindowRelativeTo};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};
use ::nvim_oxi::Array;

#[derive(Debug, Clone)]
pub(super) struct View {
    pub(super) buffer: Buffer,
    pub(super) window: Window,
    pub(super) index: usize,
}

impl TryFrom<super::Select> for View {
    type Error = utils::Error;

    fn try_from(select: super::Select) -> utils::Result<Self> {
        let index = action::get_config_index(&select.window)?;
        let mut new = Self {
            buffer: select.buffer,
            window: select.window,
            index,
        };
        new.setup()?;
        Ok(new)
    }
}

impl TryFrom<super::Edit> for View {
    type Error = utils::Error;

    fn try_from(edit: super::Edit) -> utils::Result<Self> {
        let mut new = Self {
            buffer: edit.buffer,
            window: edit.window,
            index: edit.index,
        };
        new.setup()?;
        Ok(new)
    }
}

impl LauncherState for View {
    fn update_ui(&mut self) -> crate::utils::Result<()> {
        // Create view-mode buffer content
        let config = &config::state!().list[self.index];

        let mut lines = Vec::new();
        lines.push(format!("NAME : {}", config.name()));
        lines.push(format!("CMD  : {}", config.command()));
        if !config.args().is_empty() {
            lines.push("ARGS :".to_string());
            let args = config.args();
            lines.extend(args.iter().enumerate().map(|(i, arg)| {
                if args.len() < 10 {
                    format!("  {}: {arg}", i + 1)
                } else {
                    format!("  {:>2}: {arg}", i + 1)
                }
            }));
        }
        if let Some(display) = config.display() {
            lines.push(format!("DISP : {}", display));
        }
        if let Some(cwd) = config.cwd() {
            lines.push(format!("CWD  : {}", cwd.display()));
        }
        if !config.env().is_empty() {
            lines.push("ENV  :".to_string());
            lines.extend(config.env().iter().map(|(var, value)| format!("  {var}={value}")));
        }

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

    fn update_callbacks(&mut self) -> crate::utils::Result<()> {
        nvim::command("call b:remove_callbacks()")?;

        use action::{wrap_callback, Event};
        let callback_dict = Array::from((
            wrap_callback("b", || super::state!().on(Event::Back)),
            wrap_callback("q", || super::state!().on(Event::Close)),
            wrap_callback("d", || super::state!().on(Event::Delete)),
            wrap_callback("e", || super::state!().on(Event::Edit)),
            wrap_callback("<CR>", || super::state!().on(Event::Launch)),
        ));
        self.buffer.set_var("callbacks", callback_dict)?;
        nvim::command("call b:setup_callbacks()")?;

        Ok(())
    }
}
