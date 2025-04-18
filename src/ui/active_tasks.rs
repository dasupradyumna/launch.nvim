/*-------------------------------------- ACTIVE TASKS VIEWER -------------------------------------*/

use super::utils::{index_from_cursor, Float};
use crate::core::task::ActiveTask;
use crate::utils::{buffer, float, notify, nvim_set_local, setup_module_state, Result};
use ::nvim_oxi::api as nvim;
use ::nvim_oxi::api::opts::BufDeleteOpts;

setup_module_state!(ui::active_tasks, Float);

const NO_TASKS_MSG: &str = "-- No active tasks --";

pub(crate) fn open() {
    if { self::state!().buffer.handle() } == 0 {
        self::result_handler(self::_open());
    }
}

fn _open() -> Result<()> {
    let lines = ActiveTask::get_lines_for_ui(NO_TASKS_MSG)?;
    let mut buffer = buffer::create_scratch("active_tasks")?;
    buffer::write_lines(&mut buffer, &lines)?;

    // Open a centered floating window
    // FIX: handle other navigation keymaps like wW, eE, bB etc.
    let bounds = (2, lines.len() as u32);
    buffer.set_var("bounds", ::nvim_oxi::Array::from(bounds))?;
    let width = unsafe { lines.iter().map(|l| l.len() + 8).max().unwrap_unchecked() as u32 };
    let mut window = float::open_centered("Active Tasks", &buffer, width, bounds.1 + 1)?;
    window.set_cursor(bounds.0 as usize, 0)?;
    nvim_set_local(&window, "cursorline", !ActiveTask::is_list_empty())?;

    // Set buffer keymaps for actions
    use super::utils::wrap_cb;
    use ::nvim_oxi::Array;
    let action_list = Array::from((
        Array::from(("q", wrap_cb(result_handler, |()| self::close()))),
        Array::from(("r", wrap_cb(result_handler, |()| self::relaunch()))),
        Array::from(("<CR>", wrap_cb(result_handler, |()| self::view()))),
    ));
    buffer.set_var("callbacks", action_list)?;
    nvim::command("call launch#setup_callbacks()")?;

    let state = &mut self::state!();
    state.buffer = buffer;
    state.window = window;
    Ok(())
}

fn close() -> Result<()> {
    let float = std::mem::take(&mut *self::state!());
    float.window.close(true)?;
    Ok(float.buffer.delete(&BufDeleteOpts::builder().force(true).build())?)
}

fn relaunch() -> Result<()> {
    if ActiveTask::is_list_empty() {
        notify!(Warn: "No active tasks found.");
        return Ok(());
    }

    let index = { index_from_cursor(&self::state!().window)? };
    self::close()?; // NOTE: close float before rendering : vsplit / hsplit will fail otherwise
    ActiveTask::render(index)?;
    ActiveTask::run(index)
}

fn view() -> Result<()> {
    if ActiveTask::is_list_empty() {
        notify!(Warn: "No active tasks found.");
        return Ok(());
    }

    let index = { index_from_cursor(&self::state!().window)? };
    self::close()?;
    ActiveTask::render(index)
}

fn result_handler(result: Result<()>) {
    if let Err(msg) = result {
        notify!(Error: msg);
    }
}
