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
    let Err(err_msg) = result else {
        return;
    };
    utils::notify::send!(Warn: {format!("{err_msg}")});

    match std::mem::replace(&mut *super::state!(), Launcher::Closed) {
        Launcher::Select(Select { buffer, .. })
        | Launcher::View(View { buffer, .. })
        | Launcher::Edit(Edit { buffer, .. }) => {
            let _ = self::close(buffer);
        },
        _ => {},
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
        "CWD" => config.set_cwd(value),
        arg_index if field.parse::<usize>().is_ok() => unsafe {
            let index = arg_index.parse::<usize>().unwrap_unchecked();
            config.set_arg(index - 1, value)
        },
        env_var if field.ends_with('=') => config.set_env(env_var.trim_end_matches('='), value),
        _ => {},
    }
}

fn get_config_field(index: usize, field: &str) -> String {
    let config = &config::state!().list[index];
    match field {
        "NAME" => config.name().to_string(),
        "CMD" => config.command().to_string(),
        "CWD" if config.cwd().is_some() => unsafe {
            config.cwd().as_ref().unwrap_unchecked().display().to_string()
        },
        arg_index if field.parse::<usize>().is_ok() => unsafe {
            let index = arg_index.parse::<usize>().unwrap_unchecked();
            config.args().get(index - 1).map_or_else(String::new, |a| a.into())
        },
        env_var if field.ends_with('=') => config.env()[env_var.trim_end_matches('=')].clone(),
        _ => String::new(),
    }
}

pub(super) fn edit_field(mut edit: Edit) -> utils::Result<()> {
    let (row, col) = edit.window.get_position()?;
    let offset = edit.window.get_cursor()?.0 as u32 - 1;
    let width = edit.window.get_width()?;
    let row = row as u32 + offset;
    let col = col as u32 + width + 2;

    let re_scalar = ::regex::Regex::new(r"^\ +([A-Z]+)\ +:").unwrap();
    let re_array = ::regex::Regex::new(r"^\ +([0-9]+):").unwrap();
    let re_array_new = ::regex::Regex::new(r"^\ +\+ New Arg").unwrap();
    let re_dict = ::regex::Regex::new(r"^\ +([a-zA-Z_][[:word:]]+=)").unwrap();
    let line = nvim::get_current_line()?;
    // TODO: merge all regexes or use RegexSet?
    let field = if let Some(caps) = re_scalar.captures(&line) {
        unsafe { caps.get(1).unwrap_unchecked().as_str().to_string() }
    } else if let Some(caps) = re_array.captures(&line) {
        unsafe { caps.get(1).unwrap_unchecked().as_str().to_string() }
    } else if re_array_new.is_match(&line) {
        let index = 1 + config::state!().list[edit.index].args().len();
        index.to_string()
    } else if let Some(caps) = re_dict.captures(&line) {
        unsafe { caps.get(1).unwrap_unchecked().as_str().to_string() }
    } else {
        return Ok(());
    };
    let field = field.as_str();
    let value = self::get_config_field(edit.index, field);

    match field {
        "ARGS" | "ENV" | "DISP" => {},
        env_var if field.ends_with('=') => {
            let f = field.to_string();
            let callback = self::wrap_callback_(move |input: ::nvim_oxi::String| {
                let callback = self::wrap_callback_(move |input: ::nvim_oxi::String| {
                    let mut buffer = nvim::get_current_buf();
                    let env_var: String = buffer.get_var("env_var")?;
                    buffer.del_var("env_var")?;

                    self::set_config_field(edit.index, &f, format!("{env_var}={input}"));
                    config::save()?;
                    edit.update_ui()
                });

                let mut buffer = nvim::get_current_buf();
                // TODO: input must be a valid enviroment variable name
                buffer.set_var("env_var", input)?;
                buffer.set_var("prompt", "VALUE")?;
                buffer.set_var("default", value.as_str())?;
                buffer.set_var("callback", callback)?;
                Ok(nvim::command("call b:update_prompt()")?)
            });
            utils::open_popup("VAR", env_var.trim_end_matches('='), row, col, callback)?;
        },
        _ => {
            let f = field.to_string();
            let callback = self::wrap_callback_(move |input: ::nvim_oxi::String| {
                self::set_config_field(edit.index, &f, input.to_string());
                config::save()?;
                // FIX: resets cursor to top of window
                edit.update_ui()
            });
            utils::open_popup(field, value.as_str(), row, col, callback)?;
        },
    }

    Ok(())
}

fn del_config_field(index: usize, field: &str) {
    let config = &mut config::state!().list[index];
    match field {
        "CWD" => config.del_cwd(),
        arg_index if field.parse::<usize>().is_ok() => unsafe {
            config.del_arg(arg_index.parse::<usize>().unwrap_unchecked() - 1);
        },
        env_var if field.ends_with('=') => config.del_env(env_var.trim_end_matches('=')),
        _ => {},
    }
}

pub(super) fn delete_field(edit: &mut Edit) -> utils::Result<()> {
    let re_scalar = ::regex::Regex::new(r"^\ +([A-Z]+)\ +:").unwrap();
    let re_array = ::regex::Regex::new(r"^\ +([0-9]+):").unwrap();
    let re_dict = ::regex::Regex::new(r"^\ +([a-zA-Z_][[:word:]]+=)").unwrap();
    let line = nvim::get_current_line()?;
    // TODO: merge all regexes or use RegexSet?
    let field = if let Some(caps) = re_scalar.captures(&line) {
        unsafe { caps.get(1).unwrap_unchecked().as_str() }
    } else if let Some(caps) = re_array.captures(&line) {
        unsafe { caps.get(1).unwrap_unchecked().as_str() }
    } else if let Some(caps) = re_dict.captures(&line) {
        unsafe { caps.get(1).unwrap_unchecked().as_str() }
    } else {
        return Ok(());
    };

    self::del_config_field(edit.index, field);
    config::save()?;
    edit.update_ui()
}
