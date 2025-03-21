/*------------------------------------ RUNTIME CONFIGURATIONS ------------------------------------*/

mod task;

pub(crate) use task::*;

use crate::utils::{notify, setup_module_state, Result};
use ::nvim_oxi::api::{self as nvim, Buffer};
use ::serde_json as json;
use std::path::PathBuf;
use std::sync::LazyLock;

setup_module_state!(config, [pub(crate)]
{
    filepath: PathBuf = self::get_runtime_filepath(),
    buffer: Buffer = Buffer::from(0),
    pub(crate) list: Vec<TaskConfigJson> = Vec::new(),
});

pub(crate) const NO_CONFIGS_MSG: &str = "-- No active configs --";

pub(crate) fn get_data_dir() -> &'static PathBuf {
    static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        let mut ret = match nvim::call_function::<_, String>("stdpath", ("data",)) {
            Ok(path) => PathBuf::from(path),
            Err(err) => {
                notify!(Error: format!("stdpath('data') failed - {err}"));
                return PathBuf::from("/nvim-stdpath-data-error");
            },
        };
        // WARN: change this to default path before release
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
    let opts = nvim::opts::OptionOpts::builder().buffer(buffer.clone()).build();
    nvim::set_option_value("swapfile", false, &opts)?;
    buffer.set_name(&config.filepath)?;
    config.buffer = buffer;
    Ok(())
}

pub(crate) fn delete_buffer() -> Result<()> {
    let config = &mut self::state!();
    let buffer = config.buffer.clone();
    config.buffer = Buffer::from(0);
    if config.list.is_empty() && config.filepath.is_file() {
        std::fs::remove_file(&config.filepath)?;
    }
    Ok(buffer.delete(&nvim::opts::BufDeleteOpts::builder().force(true).build())?)
}

pub(crate) fn update_buffer() -> Result<()> {
    let (mut buffer, config_str) = {
        let config = self::state!();
        (config.buffer.clone(), json::to_string_pretty(&config.list)?)
    };
    let range = 0..buffer.line_count()?;
    buffer.set_lines(range, true, config_str.split('\n'))?;
    // NOTE: closes undo block to make `nvim_buf_set_lines()` changes undoable
    self::execute_in_buffer("let &l:undolevels = &l:undolevels")
}

fn execute_in_buffer<Cmd: std::fmt::Display>(command: Cmd) -> Result<()> {
    let buffer = &self::state!().buffer;
    let command = format!("silent {command}");
    Ok(buffer.call(move |_| -> Result<()> { Ok(nvim::command(&command)?) })?)
}

fn load_configs_from_buffer() -> Result<()> {
    let mut config = self::state!();
    let buffer = &config.buffer;
    let config_str = buffer
        .get_lines(0..buffer.line_count()?, true)?
        .fold(String::new(), |acc, line| acc + &line.to_string() + "\n");
    if !config_str.trim_ascii_end().is_empty() {
        config.list = json::from_str(&config_str)?;
    }
    Ok(())
}

pub(crate) fn load_configs_from_json() -> Result<()> {
    self::execute_in_buffer("edit! | set nobuflisted")?;
    self::load_configs_from_buffer()
}

pub(super) fn redo_buffer() -> Result<()> {
    self::execute_in_buffer("redo")?;
    self::load_configs_from_buffer()
}

pub(super) fn undo_buffer() -> Result<()> {
    self::execute_in_buffer("undo")?;
    self::load_configs_from_buffer()
}

pub(crate) fn write_buffer() -> Result<()> {
    self::execute_in_buffer("write")
}

pub(crate) fn clear_undo_in_buffer() -> Result<()> {
    self::execute_in_buffer("call LaunchNvimClearUndo()")
}
