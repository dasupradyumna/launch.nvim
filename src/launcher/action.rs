/*--------------------------------------- LAUNCHER ACTIONS ---------------------------------------*/

use super::{Edit, Launcher, Select, View};
use crate::launcher::LauncherState;
use crate::{config, utils};
use ::nvim_oxi::api::opts::BufDeleteOpts;
use ::nvim_oxi::api::{self as nvim, Buffer, Window};
use ::nvim_oxi::{Array, Function};

#[derive(Debug)]
pub(super) enum Event {
    Back,
    Close,
    Delete,
    Edit,
    Launch,
    Open,
    View,
}

/*----------------------------- CALLBACK HELPERS -----------------------------*/

pub(super) fn wrap_callback<F>(key: &str, func: F) -> Array
where
    F: Fn() -> utils::Result<()> + 'static,
{
    Array::from((key, Function::from_fn(move |()| self::handle_result(func()))))
}

pub(super) fn wrap_callback_<F, T>(func: F) -> Function<T, ()>
where
    F: FnOnce(T) -> utils::Result<()> + 'static,
    T: ::nvim_oxi::lua::Poppable,
{
    Function::from_fn_once(move |arg: T| self::handle_result(func(arg)))
}

pub(super) fn handle_result(result: utils::Result<()>) {
    if let Err(e) = result {
        utils::notify::send!(Warn: {format!("{e}")});

        match std::mem::replace(&mut *super::state!(), Launcher::Closed) {
            Launcher::Select(Select { buffer, .. })
            | Launcher::View(View { buffer, .. })
            | Launcher::Edit(Edit { buffer, .. }) => {
                let _ = self::close(buffer);
            },
            _ => {},
        }
    }
}

/*----------------------------- ACTION FUNCTIONS -----------------------------*/

pub(super) fn get_config_index(window: &Window) -> utils::Result<usize> {
    if config::state!().list.is_empty() {
        // FIX: this should not be an error, since it is a valid state for the launcher
        return utils::Error::new("No active configurations found.");
    }

    Ok(window.get_cursor()?.0 - 2)
}

pub(super) fn close(buffer: Buffer) -> utils::Result<()> {
    Ok(buffer.delete(&BufDeleteOpts::builder().force(true).build())?)
}

pub(super) fn delete(index: usize) -> utils::Result<()> {
    {
        config::state!().list.remove(index);
    }
    config::save()
}

pub(super) fn launch(buffer: Buffer, index: usize) -> utils::Result<()> {
    self::close(buffer)?;

    let config = config::state!().list[index].clone().into();
    ::nvim_oxi::dbg!(&config);
    crate::core::task::run(config)
}

fn set_config_field(index: usize, field: &str, value: String) {
    let config = &mut config::state!().list[index];
    match field {
        "NAME" => config.set_name(value),
        "CMD" => config.set_command(value),
        _ => {},
    }
}

fn get_config_field(index: usize, field: &str) -> String {
    let config = &config::state!().list[index];
    match field.trim_ascii_end() {
        "NAME" => config.name().to_string(),
        "CMD" => config.command().to_string(),
        _ => String::new(),
    }
}

pub(super) fn edit(mut edit: Edit) -> utils::Result<()> {
    let (row, col) = edit.window.get_position()?;
    let offset = edit.window.get_cursor()?.0 as u32 - 1;
    let width = edit.window.get_width()?;
    let row = row as u32 + offset;
    let col = col as u32 + width + 2;

    let re = ::regex::Regex::new(r"^\s+([A-Z]+\s+):.*$").unwrap();
    let line = nvim::get_current_line()?;
    // TODO: Support other lines as well
    let field = if let Some(caps) = re.captures(&line) {
        caps.get(1).unwrap().as_str().trim_ascii_end()
    } else {
        return Ok(());
    };
    let value = self::get_config_field(edit.index, field);
    match field {
        "NAME" | "CMD" => {
            let f = field.to_string();
            let callback = self::wrap_callback_(move |input: ::nvim_oxi::String| {
                if !input.is_empty() {
                    self::set_config_field(edit.index, &f, input.to_string());
                    config::save()?;
                    // FIX: resets cursor to top of window
                    edit.update_ui()?;
                }

                Ok(())
            });
            utils::open_popup(field, value.as_str(), row, col, callback)?;
        },
        _ => {},
    }

    Ok(())
}
