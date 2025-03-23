/*----------------------------------------- UI UTILITIES -----------------------------------------*/

use crate::config;
use crate::core::task;
use crate::utils::Result;
use ::nvim_oxi::api::{Buffer, Window};
use ::nvim_oxi::Function;

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
    Ok(window.get_cursor()?.0 - 2)
}

pub(super) trait TargetItem
where
    Self: Sized,
{
    fn at_index(index: usize) -> Option<Self>;
}

impl TargetItem for task::ActiveTask {
    fn at_index(index: usize) -> Option<Self> {
        task::state!().active_list.get(index).cloned()
    }
}

impl TargetItem for config::TaskConfigJson {
    fn at_index(index: usize) -> Option<Self> {
        config::state!().list.get(index).cloned()
    }
}

macro_rules! get_item {
    (@ ActiveTask $index:expr) => {{
        use crate::ui::utils::TargetItem;
        crate::core::task::ActiveTask::at_index($index)
    }};

    (@ TaskConfig $index:expr) => {{
        use crate::ui::utils::TargetItem;
        crate::config::TaskConfigJson::at_index($index)
    }};

    ($target:ident, $window:expr) => {{
        // CHECK: might have to make offset (2 here) a parameter
        crate::ui::utils::get_item!(@ $target $window.get_cursor()?.0 - 2)
    }};
}
pub(super) use get_item;
