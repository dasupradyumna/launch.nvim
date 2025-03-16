/*------------------------------------- LAUNCHER : VIEW MODE -------------------------------------*/

use super::LauncherState;
use crate::{config, utils};
use ::nvim_oxi::api::opts::OptionOpts;
use ::nvim_oxi::api::{self as nvim, Buffer, Window};

#[derive(Debug, Clone)]
pub(super) struct View {
    pub(super) buffer: Buffer,
    pub(super) window: Window,
    pub(super) index: usize,
}

impl View {
    pub(super) fn into_select(self) -> utils::Result<super::Select> {
        let mut select = super::Select {
            buffer: self.buffer,
            window: self.window,
        };
        select.setup()?;
        let opts = OptionOpts::builder().win(select.window.clone()).build();
        nvim::set_option_value("cursorline", !config::state!().list.is_empty(), &opts)?;
        Ok(select)
    }

    pub(super) fn into_edit(self, index: usize) -> utils::Result<super::Edit> {
        let mut edit = super::Edit {
            buffer: self.buffer,
            window: self.window,
            index,
            from_select: false,
        };
        edit.setup()?;
        Ok(edit)
    }
}

impl LauncherState for View {
    super::setup_getters!();

    fn create_contents(&self) -> crate::utils::Result<Vec<String>> {
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

        Ok(lines)
    }

    super::setup_callbacks! {
        ("b", Back),
        ("q", Close),
        ("c", Copy),
        ("d", Delete),
        ("e", Edit),
        ("<CR>", Launch),
    }
}
