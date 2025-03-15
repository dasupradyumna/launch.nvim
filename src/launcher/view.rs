/*------------------------------------- LAUNCHER : VIEW MODE -------------------------------------*/

use super::{action, LauncherState};
use crate::{config, utils};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};

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
        ("d", Delete),
        ("e", Edit),
        ("<CR>", Launch),
    }
}
