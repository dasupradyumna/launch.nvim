/*----------------------------------------- UI UTILITIES -----------------------------------------*/

use crate::utils::{float, Result};
use ::nvim_oxi::api::{Buffer, Window};
use ::nvim_oxi::conversion::FromObject;
use ::nvim_oxi::Function;

pub(super) const NAVIGATION_OFFSET: usize = 3;

// TODO: refactor launcher appropriately

pub(super) struct Float {
    pub(super) buffer: Buffer,
    pub(super) window: Window,
}

impl Default for Float {
    fn default() -> Self {
        Self {
            buffer: Buffer::from(0),
            window: Window::from(0),
        }
    }
}

pub(super) fn get_popup_pos(window: &Window, on_cursor_row: bool) -> Result<(u32, u32)> {
    let (row, col) = window.get_position()?;
    let offset = if on_cursor_row { window.get_cursor()?.0 as u32 - 1 } else { 0 };
    let width = window.get_width()?;
    let row = row as u32 + offset;
    let col = col as u32 + width + 2;

    Ok((row, col))
}

pub(super) fn show_help(buffer: &Buffer, window: &Window) -> Result<()> {
    let (row, col) = get_popup_pos(window, false)?;
    let callbacks: Vec<::nvim_oxi::Array> = buffer.get_var("callbacks")?;
    let entries = callbacks
        .into_iter()
        .map(|array| {
            let mut iter = array.into_iter().flat_map(String::from_object);
            let key = unsafe { iter.next().unwrap_unchecked() };
            let desc = unsafe { iter.next().unwrap_unchecked() };
            format!("{key:^4} : {desc}")
        })
        .collect();

    float::open_help(entries, row, col)
}

pub(super) fn wrap_cb<F, T>(handler: fn(Result<()>), func: F) -> Function<T, ()>
where
    F: Fn(T) -> Result<()> + 'static,
    T: ::nvim_oxi::lua::Poppable,
{
    Function::from_fn(move |arg: T| handler(func(arg)))
}

pub(super) fn wrap_cb_once<F, T>(handler: fn(Result<()>), func: F) -> Function<T, ()>
where
    F: FnOnce(T) -> Result<()> + 'static,
    T: ::nvim_oxi::lua::Poppable,
{
    Function::from_fn_once(move |arg: T| handler(func(arg)))
}

pub(super) fn index_from_cursor(window: &Window) -> Result<usize> {
    Ok(window.get_cursor()?.0 - NAVIGATION_OFFSET)
}
