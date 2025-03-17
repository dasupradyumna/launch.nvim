/*--------------------------------------- LAUNCHER ACTIONS ---------------------------------------*/

use super::{Edit, Launcher, Select, View};
use crate::config;
use crate::launcher::LauncherState;
use crate::utils::{float, notify, Error, Result};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};
use ::nvim_oxi::Function;

#[derive(Debug)]
pub(super) enum Event {
    Add,
    Back,
    Close,
    Copy,
    Delete,
    Edit,
    InsertArg,
    Launch,
    Open,
    Save,
    View,
}

/*----------------------------- CALLBACK HELPERS -----------------------------*/

pub(super) fn wrap_cb<F, T>(func: F) -> Function<T, ()>
where
    F: Fn(T) -> Result<()> + 'static,
    T: ::nvim_oxi::lua::Poppable,
{
    Function::from_fn(move |arg: T| self::handle_result(func(arg)))
}

pub(super) fn wrap_cb_once<F, T>(func: F) -> Function<T, ()>
where
    F: FnOnce(T) -> Result<()> + 'static,
    T: ::nvim_oxi::lua::Poppable,
{
    Function::from_fn_once(move |arg: T| self::handle_result(func(arg)))
}

pub(super) fn handle_result(result: Result<()>) {
    let Err(msg) = result else {
        return;
    };
    notify!(Error: msg);

    match std::mem::replace(&mut *super::state!(), Launcher::Closed) {
        Launcher::Select(Select { buffer, .. })
        | Launcher::View(View { buffer, .. })
        | Launcher::Edit(Edit { buffer, .. }) => {
            _ = self::close_launcher(buffer);
        },
        _ => {},
    }
}

/*----------------------------- ACTION FUNCTIONS -----------------------------*/

pub(super) fn get_config_index(window: &Window) -> Result<usize> {
    if config::state!().list.is_empty() {
        // FIX: this should not be an error, since it is a valid state for the launcher
        return Error::new("No active configurations found.");
    }

    Ok(window.get_cursor()?.0 - 2)
}

pub(super) fn add_config() -> Result<()> {
    config::state!().list.push(config::TaskConfigJson::default());
    Ok(())
}

pub(super) fn copy_config(index: usize) -> Result<()> {
    let configs = &mut config::state!().list;
    let copied = configs[index].clone();
    configs.push(copied);
    Ok(())
}

pub(super) fn close_launcher(buffer: Buffer) -> Result<()> {
    buffer.delete(&nvim::opts::BufDeleteOpts::default())?;
    config::close_buffer()
}

pub(super) fn delete_config(index: usize) -> Result<()> {
    {
        config::state!().list.remove(index);
    }
    config::update_buffer()?;
    config::write_buffer()
}

pub(super) fn launch_config(buffer: Buffer, index: usize) -> Result<()> {
    self::close_launcher(buffer)?;

    let config = config::state!().list[index].clone().try_into()?;
    ::nvim_oxi::dbg!(&config);
    crate::core::task::run(config)
}

fn get_popup_pos(window: &Window) -> Result<(u32, u32)> {
    let (row, col) = window.get_position()?;
    let offset = window.get_cursor()?.0 as u32 - 1;
    let width = window.get_width()?;
    let row = row as u32 + offset;
    let col = col as u32 + width + 2;

    Ok((row, col))
}

fn match_field_regex() -> Result<String> {
    let line = nvim::get_current_line()?;
    let patterns = [
        r"^\ +([A-Z]+)\ +:",
        r"^\ +([0-9]+):",
        r"^\ +(\+) New Arg...",
        r"^\ +([a-zA-Z_][[:word:]]+=)",
        r"^\ +\+ New Var(=)...",
    ];
    let re = unsafe { ::regex::Regex::new(&patterns.join("|")).unwrap_unchecked() };
    let r#match = unsafe { re.captures(&line).unwrap_unchecked().iter().flatten().nth(1) };
    let field = unsafe { r#match.unwrap_unchecked().as_str().to_string() };
    Ok(field)
}

fn set_config_field(index: usize, field: &str, value: String) {
    let config = &mut config::state!().list[index];
    match field {
        "NAME" => config.set_name(value),
        "CMD" => config.set_cmd(value),
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
        "CMD" => config.cmd().to_string(),
        "CWD" if config.cwd().is_some() => unsafe {
            config.cwd().as_ref().unwrap_unchecked().display().to_string()
        },
        arg_index if field.parse::<usize>().is_ok() => unsafe {
            let index = arg_index.parse::<usize>().unwrap_unchecked();
            config.args().get(index - 1).map_or_else(String::new, |a| a.into())
        },
        env_var if field.ends_with('=') => config
            .env()
            .get(env_var.trim_end_matches('='))
            .map_or_else(String::new, |v| v.clone()),
        _ => String::new(),
    }
}

pub(super) fn edit_field(mut edit: Edit) -> Result<()> {
    let (row, col) = self::get_popup_pos(&edit.window)?;
    let mut field = self::match_field_regex()?;
    if field == "+" {
        field = (config::state!().list[edit.index].args().len() + 1).to_string();
    };
    let field = field.as_str();
    let value = self::get_config_field(edit.index, field);

    match field {
        "ARGS" | "ENV" | "" => Ok(()),
        "DISP" => {
            let callback = self::wrap_cb_once(move |()| {
                {
                    let choice = nvim::get_current_line()?.trim_ascii().into();
                    config::state!().list[edit.index].set_disp(choice);
                }
                config::update_buffer()?;
                edit.update_ui()
            });
            float::select(vec!["float", "hsplit", "vsplit"], row, col, callback)
        },
        env_var if field.ends_with('=') => {
            let f = field.to_string();
            let callback = self::wrap_cb_once(move |input: ::nvim_oxi::String| {
                let callback = self::wrap_cb_once(move |input: ::nvim_oxi::String| {
                    let mut buffer = nvim::get_current_buf();
                    let env_var: String = buffer.get_var("env_var")?;
                    buffer.del_var("env_var")?;

                    self::set_config_field(edit.index, &f, format!("{env_var}={input}"));
                    config::update_buffer()?;
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
            float::prompt("VAR", env_var.trim_end_matches('='), row, col, callback)
        },
        _ => {
            let f = field.to_string();
            let callback = self::wrap_cb_once(move |input: ::nvim_oxi::String| {
                self::set_config_field(edit.index, &f, input.to_string());
                config::update_buffer()?;
                edit.update_ui()
            });
            float::prompt(field, value.as_str(), row, col, callback)
        },
    }
}

fn del_config_field(index: usize, field: &str) -> bool {
    let config = &mut config::state!().list[index];
    match field {
        "CWD" => config.del_cwd(),
        "DISP" => config.del_disp(),
        arg_index if field.parse::<usize>().is_ok() => unsafe {
            config.del_arg(arg_index.parse::<usize>().unwrap_unchecked() - 1);
        },
        env_var if field.ends_with('=') && env_var != "=" => {
            config.del_env(env_var.trim_end_matches('='))
        },
        _ => return false,
    }
    true
}

pub(super) fn delete_field(edit: &mut Edit) -> Result<()> {
    let field = self::match_field_regex()?;
    if !self::del_config_field(edit.index, field.as_str()) {
        return Ok(());
    }
    config::update_buffer()?;
    edit.update_ui()
}

pub(super) fn insert_arg(mut edit: Edit) -> Result<()> {
    let (row, col) = self::get_popup_pos(&edit.window)?;
    let field = self::match_field_regex()?;
    let Ok(index) = field.parse::<usize>() else {
        return Ok(());
    };
    let callback = self::wrap_cb_once(move |input: ::nvim_oxi::String| {
        {
            let config = &mut config::state!().list[edit.index];
            config.insert_arg(index - 1, input.to_string());
        }
        config::update_buffer()?;
        edit.update_ui()
    });

    float::prompt(format!("{field}-new").as_str(), "", row, col, callback)
}
