/*------------------------------------ RUNTIME CONFIGURATIONS ------------------------------------*/

mod serde;
mod task;

pub(crate) use task::*;

use crate::utils::{notify, nvim_set_local, setup_module_state, Result};
use ::nvim_oxi::api::{self as nvim, Buffer};
use ::nvim_oxi::Dictionary;
use std::path::PathBuf;
use std::sync::LazyLock;

// FIX: clear modified flag on config buffer on VimLeavePre to prevent save prompts

setup_module_state!(config, [pub(crate)]
{
    filepath: PathBuf = self::get_runtime_filepath(),
    buffer: Buffer = Buffer::from(0),
    version: u8 = 1,
    pub(crate) tasks: Vec<TaskConfigJson> = Vec::new(),
});

pub(crate) const NO_CONFIGS_MSG: &str = "-- No active configs --";

pub(crate) fn on_dirchanged() {
    self::state!().filepath = self::get_runtime_filepath();
}

pub(crate) fn get_data_dir() -> &'static PathBuf {
    static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        let mut ret = match nvim::call_function::<_, String>("stdpath", ("data",)) {
            Ok(path) => PathBuf::from(path),
            Err(err) => {
                notify!(Error: format!("stdpath('data') failed - {err}"));
                return PathBuf::from("/nvim-stdpath-data-error");
            },
        };
        // FIX: change this to default path before release
        ret.push("launch_nvim_rust");

        ret
    });

    &DATA_DIR
}

fn get_runtime_filepath() -> PathBuf {
    let json_filename = match std::env::current_dir() {
        Ok(cwd) => cwd.to_string_lossy().replace("@", "@@"),
        Err(err) => {
            notify!(Error: format!("std::env::current_dir() failed - {err}"));
            return PathBuf::from("/rust-env-cwd-error");
        },
    };
    let re = unsafe { ::regex::Regex::new(r"[\\/:]").unwrap_unchecked() };
    let json_filename = format!("{}.json", re.replace_all(&json_filename, "@"));
    self::get_data_dir().join(json_filename)
}

pub(crate) fn create_buffer() -> Result<()> {
    let mut config = self::state!();
    let mut buffer = nvim::create_buf(false, false)?;
    nvim_set_local(&buffer, "swapfile", false)?;
    buffer.set_name(&config.filepath)?;
    config.buffer = buffer;
    Ok(())
}

pub(crate) fn delete_buffer() -> Result<()> {
    let config = &mut self::state!();
    if config.tasks.is_empty() && config.filepath.is_file() {
        std::fs::remove_file(&config.filepath)?;
    }
    let buffer = std::mem::replace(&mut config.buffer, Buffer::from(0));
    Ok(buffer.delete(&nvim::opts::BufDeleteOpts::builder().force(true).build())?)
}

fn execute_in_buffer<Cmd: std::fmt::Display>(command: Cmd) -> Result<()> {
    let buffer = &self::state!().buffer;
    let command = format!("silent {command}");
    Ok(buffer.call(move |_| -> Result<()> { Ok(nvim::command(&command)?) })?)
}

pub(crate) fn serialize_to_buffer() -> Result<()> {
    let contents = self::serde::serialize()?;
    self::state!().buffer.set_lines(.., true, contents.split('\n'))?;
    // NOTE: closes undo block to make `nvim_buf_set_lines()` changes undoable
    self::execute_in_buffer("let &l:undolevels = &l:undolevels")
}

fn deserialize_from_buffer() -> Result<()> {
    let contents = {
        let lines = self::state!().buffer.get_lines(.., true)?;
        lines.fold(String::new(), |acc, line| acc + &line.to_string() + "\n")
    };
    self::serde::deserialize(&contents)
}

pub(crate) fn load_configs_from_json() -> Result<()> {
    self::execute_in_buffer("edit! | set nobuflisted")?;
    self::deserialize_from_buffer()
}

fn get_undotree() -> Result<Dictionary> {
    let buffer = &self::state!().buffer;
    Ok(nvim::call_function("undotree", (buffer.handle(),))?)
}

pub(super) fn redo_buffer() -> Result<bool> {
    let undotree = self::get_undotree()?;
    if undotree["seq_cur"] == undotree["seq_last"] {
        return Ok(false);
    }

    self::execute_in_buffer("redo")?;
    self::deserialize_from_buffer()?;
    Ok(true)
}

pub(super) fn undo_buffer() -> Result<bool> {
    let undotree = self::get_undotree()?;
    if unsafe { undotree["seq_cur"].as_integer_unchecked() } == 0 {
        return Ok(false);
    }

    self::execute_in_buffer("undo")?;
    self::deserialize_from_buffer()?;
    Ok(true)
}

pub(crate) fn write_buffer() -> Result<()> {
    self::execute_in_buffer("write")
}

pub(crate) fn clear_undo_in_buffer() -> Result<()> {
    self::execute_in_buffer("call launch#clear_undo_history()")
}
