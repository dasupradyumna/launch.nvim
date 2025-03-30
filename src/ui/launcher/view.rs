/*------------------------------------- LAUNCHER : VIEW MODE -------------------------------------*/

use super::{Edit, LauncherState, Select};
use crate::config;
use crate::utils::{nvim_set_local, Result};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};

#[derive(Debug, Clone)]
pub(in crate::ui) struct View {
    pub(in crate::ui) buffer: Buffer,
    pub(in crate::ui) window: Window,
    pub(in crate::ui) index: usize,
}

impl View {
    pub(super) fn into_select(self) -> Result<Select> {
        let mut select = Select {
            buffer: self.buffer,
            window: self.window,
        };
        select.setup()?;
        nvim_set_local(&select.window, "cursorline", !config::state!().tasks.is_empty())?;

        Ok(select)
    }

    pub(super) fn into_edit(self, index: usize) -> Result<Edit> {
        let mut edit = Edit {
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

    fn create_contents(&self) -> Result<Vec<String>> {
        let config = &config::state!().tasks[self.index];

        let mut lines = vec!["".into()];
        lines.push(format!("NAME : {}", config.name()));
        lines.push(format!("CMD  : {}", config.cmd()));
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
        if let Some(display) = config.disp() {
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
