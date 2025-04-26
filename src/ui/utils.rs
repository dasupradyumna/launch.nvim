/*----------------------------------------- UI UTILITIES -----------------------------------------*/

use crate::utils::Result;
use ::nvim_oxi::api::{Buffer, Window};
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
