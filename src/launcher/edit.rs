/*------------------------------------- LAUNCHER : EDIT MODE -------------------------------------*/

use super::{action, LauncherState};
use crate::{config, utils};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};

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
    super::setup_getters!();

    fn create_contents(&self) -> utils::Result<Vec<String>> {
        let config = &config::state!().list[self.index];
        const NONE: &str = "---";

        let mut lines = Vec::new();
        lines.push(format!("NAME : {}", config.name()));
        lines.push(format!("CMD  : {}", config.command()));
        lines.push("ARGS :".to_string());
        let args = config.args();
        lines.extend(args.iter().enumerate().map(|(i, arg)| {
            if args.len() < 10 {
                format!("  {}: {arg}", i + 1)
            } else {
                format!("  {:>2}: {arg}", i + 1)
            }
        }));
        lines.push("  + New Arg...".to_string());
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
        lines.extend(config.env().iter().map(|(var, value)| format!("  {var}={value}")));
        lines.push("  + New Var=...".to_string());

        Ok(lines)
    }

    super::setup_callbacks! {
        ("b", Back),
        ("q", Close),
        ("d", Delete),
        ("<CR>", Edit),
        ("i", InsertArg),
    }
}
