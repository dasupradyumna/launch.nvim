/*------------------------------------- LAUNCHER : EDIT MODE -------------------------------------*/

use super::{LauncherState, Select, View};
use crate::config;
use crate::ui::utils::NAVIGATION_OFFSET;
use crate::utils::Result;
use ::nvim_oxi::api::{self as nvim, Buffer, Window};

#[derive(Debug, Clone)]
pub(super) struct Edit {
    pub(super) buffer: Buffer,
    pub(super) window: Window,
    pub(super) index: usize,
    pub(super) from_select: bool,
}

impl Edit {
    pub(super) fn into_select(self) -> Result<Select> {
        config::buffer::clear_undo_history()?;
        let mut select = Select {
            buffer: self.buffer,
            window: self.window,
        };
        select.setup()?;
        let line_nr = NAVIGATION_OFFSET + self.index % { config::state!().tasks.len() };
        select.window.set_cursor(line_nr, 0)?;
        Ok(select)
    }

    pub(super) fn into_view(self) -> Result<View> {
        config::buffer::clear_undo_history()?;
        let mut view = View {
            buffer: self.buffer,
            window: self.window,
            index: self.index,
        };
        view.setup()?;
        Ok(view)
    }
}

impl LauncherState for Edit {
    super::setup_getters!();

    fn create_contents(&self) -> Vec<String> {
        let config = &config::state!().tasks[self.index];
        const NONE: &str = "---";

        let mut lines = Vec::new();
        lines.push(format!("NAME : {}", config.name()));
        lines.push(format!("CMD  : {}", config.cmd()));
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
        let fmt = match config.disp() {
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

        lines
    }

    super::setup_callbacks! {
        ("b", Back, "Back to previous window"),
        ("q", Close, "Close launcher"),
        ("d", Delete, "Delete / reset field"),
        ("<CR>", Edit, "Edit field"),
        ("h", Help, "Open help"),
        ("i", InsertArg, "Insert argument in `ARGS`"),
        ("U", Redo, "Redo action"),
        ("s", Save, "Save and go back"),
        ("u", Undo, "Undo action"),
    }
}
