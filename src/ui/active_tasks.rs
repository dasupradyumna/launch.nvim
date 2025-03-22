/*-------------------------------------- ACTIVE TASKS VIEWER -------------------------------------*/

use crate::core::task;
use crate::utils::{buffer, float, notify, Result};
use ::nvim_oxi::api as nvim;
use ::nvim_oxi::api::opts::{BufDeleteOpts, OptionOpts};

pub(crate) fn open() {
    if let Err(msg) = self::_open() {
        notify!(Error: msg);
    }
}

fn _open() -> Result<()> {
    let NO_TASKS_MSG = "-- No active tasks --";

    let state = &mut task::state!();
    let lines: Vec<String> = if state.active_list.is_empty() {
        vec!["".into(), NO_TASKS_MSG.into()]
    } else {
        let mut vec = vec!["".into()];
        vec.extend(state.active_list.iter().map(|c| c.config.name().into()));
        vec
    };
    let mut buffer = buffer::create_scratch("active_tasks")?;
    buffer::write_lines(&mut buffer, &lines)?;

    // Open a centered floating window
    // FIX: handle other navigation keymaps like wW, eE, bB etc.
    let bounds = (2u32, lines.len() as u32);
    buffer.set_var("bounds", ::nvim_oxi::Array::from(bounds))?;
    let width = unsafe { lines.iter().map(|l| l.len() + 8).max().unwrap_unchecked() as u32 };
    let mut window = float::open_centered("Active Tasks", &buffer, width, bounds.1 + 1)?;
    window.set_cursor(bounds.0 as usize, 0)?;
    let opts = OptionOpts::builder().win(window.clone()).build();
    nvim::set_option_value("cursorline", !state.active_list.is_empty(), &opts)?;

    // Set buffer keymaps for actions
    use crate::ui::launcher::action;
    use ::nvim_oxi::Array;
    let action_list = {
        let buf = buffer.clone();
        Array::from((Array::from((
            "q",
            action::wrap_cb_once(move |()| {
                Ok(buf.delete(&BufDeleteOpts::builder().force(true).build())?)
            }),
        )),))
    };
    buffer.set_var("callbacks", action_list)?;
    nvim::command("call launch#setup_callbacks()")?;

    Ok(())
}
