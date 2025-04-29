/*--------------------------------------- LAUNCHER ACTIONS ---------------------------------------*/
//!
//! This module provides the supported events and action behaviors for all launcher UI states.
//!
//! CHECK: can this be refactored into the modes?

use super::{Edit, Launcher, Select, View};
use crate::config;
use crate::ui::launcher::LauncherState;
use crate::ui::utils::{get_popup_pos, wrap_cb_once};
use crate::utils::{float, notify, IndexChecked, Result};
use ::nvim_oxi::api::{self as nvim, Buffer};

/// Defines the supported events for the launcher UI
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

/// Handles the result of a launcher action, and closes the launcher if an error occurs
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

/*------------------------------ COMMON ACTIONS ------------------------------*/

/// Close the launcher UI
///
/// Delete the launcher and the config buffers
pub(super) fn close_launcher(buffer: Buffer) -> Result<()> {
    buffer.delete(&nvim::opts::BufDeleteOpts::default())?;
    config::buffer::delete()
}

/// Redo the last undone action
///
/// If the action is successful, the UI is updated.
/// Only write to the config file if `write` is set.
pub(super) fn redo_action<LS: LauncherState>(state: &mut LS, write: bool) -> Result<()> {
    if !config::buffer::redo()? {
        return Ok(());
    }
    if write {
        config::buffer::write_to_file()?;
    }
    state.update_ui()
}

/// Undo the last action
///
/// If the action is successful, the UI is updated.
/// Only write to the config file if `write` is set.
pub(super) fn undo_action<LS: LauncherState>(state: &mut LS, write: bool) -> Result<()> {
    if !config::buffer::undo()? {
        return Ok(());
    }
    if write {
        config::buffer::write_to_file()?;
    }
    state.update_ui()
}

/*---------------------------- SELECT-VIEW ACTIONS ---------------------------*/

/// Create a new configuration and add it to the runtime list
///
/// Update the config buffer after adding the new configuration
pub(super) fn add_config() -> Result<()> {
    {
        config::state!().tasks.push(config::TaskConfigJson::default());
    }
    config::buffer::serialize_to_string()
}

/// Copy the configuration at the given index, and add it to the runtime list
///
/// Update the config buffer after adding the new configuration
pub(super) fn copy_config(index: usize) -> Result<()> {
    {
        let configs = &mut config::state!().tasks;
        let config = configs.get_checked(index)?.clone();
        configs.push(config);
    }
    config::buffer::serialize_to_string()
}

/// Delete the configuration at the given index
///
/// Update the config buffer after deleting the configuration, and write to the config file
pub(super) fn delete_config(index: usize) -> Result<()> {
    {
        config::state!().tasks.remove(index);
    }
    config::buffer::serialize_to_string()?;
    config::buffer::write_to_file()
}

/// Close the launcher and launch the configuration at the given index
pub(super) fn launch_config(buffer: Buffer, index: usize) -> Result<()> {
    self::close_launcher(buffer)?;

    let config = config::state!().tasks.get_checked(index)?.clone().try_into()?;
    ::nvim_oxi::dbg!(&config);
    crate::core::task::run(config)
}

/*------------------------------- EDIT ACTIONS -------------------------------*/

/// Get the field name from the current line, using regular expression pattern matching
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

/// Set the value of the field at the given index in the configuration
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

/// Get the value of the field at the given index in the configuration
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

/// Edit the field of the current configuration under the cursor
///
/// DISP opens up a selection out of the three options: float, hsplit, vsplit
/// ARGS and ENV opens up a prompt to add or edit arguments or environment variables
/// All other fields open up a prompt to edit the current value
pub(super) fn edit_field(mut edit: Edit) -> Result<()> {
    let (row, col) = get_popup_pos(&edit.window, true)?;
    let mut field = self::match_field_regex()?;
    if field == "+" {
        field = (config::state!().tasks[edit.index].args().len() + 1).to_string();
    };
    let field = field.as_str();
    let value = self::get_config_field(edit.index, field);

    match field {
        "ARGS" | "ENV" | "" => Ok(()),
        // Get user choice for display mode
        "DISP" => {
            let callback = wrap_cb_once(self::result_handler, move |()| {
                {
                    let choice = nvim::get_current_line()?.trim_ascii().into();
                    config::state!().tasks[edit.index].set_disp(choice);
                }
                config::buffer::serialize_to_string()?;
                edit.update_ui()
            });
            float::open_select(vec!["float", "hsplit", "vsplit"], row, col, callback)
        },
        // Get user input for editing or adding a new enviroment variable
        env_var if field.ends_with('=') => {
            let f = field.to_string();
            // This callback is used to get user input for the variable name
            let callback = wrap_cb_once(self::result_handler, move |input: ::nvim_oxi::String| {
                // This callback is used to get user input for the variable value
                let callback =
                    wrap_cb_once(self::result_handler, move |input: ::nvim_oxi::String| {
                        let mut buffer = nvim::get_current_buf();
                        let env_var: String = buffer.get_var("env_var")?;
                        buffer.del_var("env_var")?;

                        let input: String = input.to_string_lossy().trim_ascii().into();
                        self::set_config_field(edit.index, &f, format!("{env_var}={input}"));
                        config::buffer::serialize_to_string()?;
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
        // Get user input for editing fields
        _ => {
            let f = field.to_string();
            let callback = wrap_cb_once(self::result_handler, move |input: ::nvim_oxi::String| {
                self::set_config_field(edit.index, &f, input.to_string_lossy().trim_ascii().into());
                config::buffer::serialize_to_string()?;
                edit.update_ui()
            });
            float::open_prompt(field, value.as_str(), row, col, callback)
        },
    }
}

/// Delete the field at the given index in the current configuration
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

/// Delete the field of the current configuration under the cursor
pub(super) fn delete_field(edit: &mut Edit) -> Result<()> {
    let field = self::match_field_regex()?;
    if !self::del_config_field(edit.index, field.as_str()) {
        return Ok(());
    }
    config::buffer::serialize_to_string()?;
    edit.update_ui()
}

/// Add a new argument to the current configuration argument list
pub(super) fn insert_arg(mut edit: Edit) -> Result<()> {
    let (row, col) = get_popup_pos(&edit.window, true)?;
    let field = self::match_field_regex()?;
    let Ok(index) = field.parse::<usize>() else {
        return Ok(());
    };
    let callback = wrap_cb_once(self::result_handler, move |input: ::nvim_oxi::String| {
        {
            let config = &mut config::state!().tasks[edit.index];
            config.insert_arg(index - 1, input.to_string_lossy().trim_ascii().into());
        }
        config::buffer::serialize_to_string()?;
        edit.update_ui()
    });

    float::open_prompt(format!("{field}-new").as_str(), "", row, col, callback)
}
