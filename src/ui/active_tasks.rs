/*-------------------------------------- ACTIVE TASKS VIEWER -------------------------------------*/

use crate::core::task;
use crate::utils::{float, notify, Result};
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
        Vec::from_iter([NO_TASKS_MSG.into()])
    } else {
        state.active_list.iter().map(|c| c.config.name().into()).collect()
    };

    // Create buffer
    let mut buffer = nvim::create_buf(false, true)?;
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();
    nvim::set_option_value("filetype", "launch_nvim_active_task_list", &opts)?;

    // Set buffer contents
    let range = 1..buffer.line_count()?;
    nvim::set_option_value("modifiable", true, &opts)?;
    buffer.set_lines(range, true, lines.iter().map(|s| format!("    {s}    ")))?;
    nvim::set_option_value("modifiable", false, &opts)?;

    // Open a centered floating window
    let n = lines.len() as u32;
    // FIX: handle other navigation keymaps like wW, eE, bB etc.
    buffer.set_var("bounds", ::nvim_oxi::Array::from((2, n + 1)))?;
    let height = n + 2;
    let width = unsafe { lines.iter().map(|l| l.len() + 8).max().unwrap_unchecked() as u32 };
    let mut window = float::centered("Active Tasks", &buffer, width, height)?;
    window.set_cursor(2, 0)?;
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
    nvim::command("call b:setup_callbacks()")?;

    Ok(())
}
