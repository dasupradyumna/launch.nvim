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

macro_rules! get_target_item {
    ($target:ident, $window:expr) => {{
        use crate::config::TaskConfigJson;
        use crate::core::task::ActiveTask;
        use crate::ui::utils::TargetItem;

        // CHECK: might have to make offset (2 here) a parameter
        $target::at_index($window.get_cursor()?.0 - 2)
    }};
}
pub(super) use get_target_item;
