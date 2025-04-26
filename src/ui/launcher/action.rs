/*--------------------------------------- LAUNCHER ACTIONS ---------------------------------------*/

use super::{Edit, Launcher, Select, View};
use crate::config;
use crate::ui::launcher::LauncherState;
use crate::ui::utils::wrap_cb_once;
use crate::utils::{float, notify, Error, Result};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};
use ::nvim_oxi::conversion::FromObject;

#[derive(Debug)]
pub(super) enum Event {
    Add,
    Back,
    Close,
    Copy,
    Delete,
    Edit,
    Help,
    InsertArg,
    Launch,
    Open,
    Redo,
    Save,
    Undo,
    View,
}

pub(super) fn result_handler(result: Result<()>) {
    let Err(msg) = result else {
        return;
    };
    notify!(Error: msg);

    match std::mem::take(&mut *super::state!()) {
        Launcher::Select(Select { buffer, .. })
        | Launcher::View(View { buffer, .. })
        | Launcher::Edit(Edit { buffer, .. }) => {
            _ = self::close_launcher(buffer);
        },
        _ => {},
    }
}

/*----------------------------- ACTION FUNCTIONS -----------------------------*/

pub(super) fn show_help(state: &Launcher) -> Result<()> {
    let (buffer, window) = match state {
        Launcher::Closed => return Error::new("Cannot show help when launcher is closed"),
        Launcher::Select(Select { buffer, window })
        | Launcher::View(View { buffer, window, .. })
        | Launcher::Edit(Edit { buffer, window, .. }) => (buffer, window),
    };

    let (row, col) = self::get_popup_pos(window, false)?;
    let callbacks: Vec<::nvim_oxi::Array> = buffer.get_var("callbacks")?;
    let lines = callbacks
        .into_iter()
        .map(|array| {
            let mut iter = array.into_iter().flat_map(String::from_object);
            let key = unsafe { iter.next().unwrap_unchecked() };
            let desc = unsafe { iter.next().unwrap_unchecked() };
            format!("{key}: {desc}")
        })
        .collect();

    float::open_help(lines, row, col)
}

pub(super) fn redo_action<LS: LauncherState>(state: &mut LS, write: bool) -> Result<()> {
    if !config::redo_buffer()? {
        return Ok(());
    }
    if write {
        config::write_buffer()?;
    }
    state.update_ui()
}

pub(super) fn undo_action<LS: LauncherState>(state: &mut LS, write: bool) -> Result<()> {
    if !config::undo_buffer()? {
        return Ok(());
    }
    if write {
        config::write_buffer()?;
    }
    state.update_ui()
}

pub(super) fn add_config() -> Result<()> {
    {
        config::state!().tasks.push(config::TaskConfigJson::default());
    }
    config::serialize_to_buffer()
}

pub(super) fn copy_config(index: usize) -> Result<()> {
    {
        let configs = &mut config::state!().tasks;
        let config = configs[index].clone();
        configs.push(config);
    }
    config::serialize_to_buffer()
}

pub(super) fn close_launcher(buffer: Buffer) -> Result<()> {
    buffer.delete(&nvim::opts::BufDeleteOpts::default())?;
    config::delete_buffer()
}

pub(super) fn delete_config(index: usize) -> Result<()> {
    {
        config::state!().tasks.remove(index);
    }
    config::serialize_to_buffer()?;
    config::write_buffer()
}

pub(super) fn launch_config(buffer: Buffer, index: usize) -> Result<()> {
    self::close_launcher(buffer)?;

    let config = config::state!().tasks[index].clone().try_into()?;
    ::nvim_oxi::dbg!(&config);
    crate::core::task::run(config)
}

fn get_popup_pos(window: &Window, on_cursor_row: bool) -> Result<(u32, u32)> {
    let (row, col) = window.get_position()?;
    let offset = if on_cursor_row { window.get_cursor()?.0 as u32 - 1 } else { 0 };
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
    let config = &mut config::state!().tasks[index];
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
    let config = &config::state!().tasks[index];
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
    let (row, col) = self::get_popup_pos(&edit.window, true)?;
    let mut field = self::match_field_regex()?;
    if field == "+" {
        field = (config::state!().tasks[edit.index].args().len() + 1).to_string();
    };
    let field = field.as_str();
    let value = self::get_config_field(edit.index, field);

    match field {
        "ARGS" | "ENV" | "" => Ok(()),
        "DISP" => {
            let callback = wrap_cb_once(self::result_handler, move |()| {
                {
                    let choice = nvim::get_current_line()?.trim_ascii().into();
                    config::state!().tasks[edit.index].set_disp(choice);
                }
                config::serialize_to_buffer()?;
                edit.update_ui()
            });
            float::open_select(vec!["float", "hsplit", "vsplit"], row, col, callback)
        },
        env_var if field.ends_with('=') => {
            let f = field.to_string();
            let callback = wrap_cb_once(self::result_handler, move |input: ::nvim_oxi::String| {
                let callback =
                    wrap_cb_once(self::result_handler, move |input: ::nvim_oxi::String| {
                        let mut buffer = nvim::get_current_buf();
                        let env_var: String = buffer.get_var("env_var")?;
                        buffer.del_var("env_var")?;

                        let input: String = input.to_string_lossy().trim_ascii().into();
                        self::set_config_field(edit.index, &f, format!("{env_var}={input}"));
                        config::serialize_to_buffer()?;
                        edit.update_ui()
                    });

                let mut buffer = nvim::get_current_buf();
                // TODO: input must be a valid enviroment variable name
                let input: String = input.to_string_lossy().trim_ascii().into();
                buffer.set_var("env_var", input)?;
                buffer.set_var("prompt", "VALUE")?;
                buffer.set_var("default", value.as_str())?;
                buffer.set_var("callback", callback)?;
                Ok(nvim::command("call b:update_prompt()")?)
            });
            float::open_prompt("VAR", env_var.trim_end_matches('='), row, col, callback)
        },
        _ => {
            let f = field.to_string();
            let callback = wrap_cb_once(self::result_handler, move |input: ::nvim_oxi::String| {
                self::set_config_field(edit.index, &f, input.to_string_lossy().trim_ascii().into());
                config::serialize_to_buffer()?;
                edit.update_ui()
            });
            float::open_prompt(field, value.as_str(), row, col, callback)
        },
    }
}

fn del_config_field(index: usize, field: &str) -> bool {
    let config = &mut config::state!().tasks[index];
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
    config::serialize_to_buffer()?;
    edit.update_ui()
}

pub(super) fn insert_arg(mut edit: Edit) -> Result<()> {
    let (row, col) = self::get_popup_pos(&edit.window, true)?;
    let field = self::match_field_regex()?;
    let Ok(index) = field.parse::<usize>() else {
        return Ok(());
    };
    let callback = wrap_cb_once(self::result_handler, move |input: ::nvim_oxi::String| {
        {
            let config = &mut config::state!().tasks[edit.index];
            config.insert_arg(index - 1, input.to_string_lossy().trim_ascii().into());
        }
        config::serialize_to_buffer()?;
        edit.update_ui()
    });

    float::open_prompt(format!("{field}-new").as_str(), "", row, col, callback)
}
