/*------------------------------------ RUNTIME CONFIGURATIONS ------------------------------------*/

mod task;

pub(crate) use task::*;

use crate::utils;
use ::nvim_oxi::api::{self as nvim, Buffer};
use ::serde_json as json;
use std::path::PathBuf;
use std::sync::LazyLock;

utils::setup_module_state!(config, [pub(crate)]
{
    filepath: PathBuf = self::get_runtime_filepath(),
    buffer: Buffer = Buffer::from(0),
    pub(crate) list: Vec<TaskConfigJson> = Vec::new(),
});

pub(crate) const NO_CONFIGS_MSG: &str = "-- No active configs --";

pub(crate) fn get_data_dir() -> &'static PathBuf {
    static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        let ret: String = nvim::call_function("stdpath", ("data",)).unwrap();
        let mut ret = PathBuf::from(ret);
        ret.push("launch_nvim_rust");

        ret
    });

    &DATA_DIR
}

fn get_runtime_filepath() -> PathBuf {
    // get the JSON filename for the current working directory
    // CHECK: if below replace() call can be included into regex pattern
    let json_filename = std::env::current_dir().unwrap().to_string_lossy().replace("@", "@@");
    let re = ::regex::Regex::new(r"[\\/:]").unwrap();
    let json_filename = format!("{}.json", re.replace_all(&json_filename, "@"));

    // get the full JSON filepath in the plugin data directory
    self::get_data_dir().join(json_filename)
}

pub(crate) fn setup_buffer_and_configs() -> utils::Result<()> {
    {
        let mut config = self::state!();
        let mut buffer = nvim::create_buf(false, false)?;
        let opts = nvim::opts::OptionOpts::builder().buffer(buffer.clone()).build();
        nvim::set_option_value("swapfile", false, &opts)?;
        buffer.set_name(&config.filepath)?;
        config.buffer = buffer;
        ::nvim_oxi::dbg!(&config);
    }
    self::load_configs_from_json()
}

pub(crate) fn update_buffer() -> utils::Result<()> {
    let mut config = self::state!();
    let config_str = json::to_string_pretty(&config.list)?;
    let range = 0..config.buffer.line_count()?;
    Ok(config.buffer.set_lines(range, true, config_str.split('\n'))?)
}

pub(crate) fn write_buffer() -> utils::Result<()> {
    let buffer = &self::state!().buffer;
    Ok(buffer.call(|_| -> utils::Result<()> { Ok(nvim::command("silent write")?) })?)
}

pub(crate) fn load_configs_from_json() -> utils::Result<()> {
    let mut config = self::state!();
    let buffer = &config.buffer;
    () = buffer.call(|_| -> utils::Result<()> { Ok(nvim::command("edit! | set nobuflisted")?) })?;

    let config_str = buffer
        .get_lines(0..buffer.line_count()?, true)?
        .fold(String::new(), |acc, line| acc + &line.to_string() + "\n");
    let config_str = config_str.trim_ascii_end(); // Handles whitespace from empty buffer
    if !config_str.is_empty() {
        config.list = json::from_str(config_str)?;
    }
    Ok(())
}

pub(crate) fn close_buffer() -> utils::Result<()> {
    let config = &mut self::state!();
    let buffer = config.buffer.clone();
    config.buffer = Buffer::from(0);
    if config.list.is_empty() && config.filepath.is_file() {
        std::fs::remove_file(&config.filepath)?;
    }
    Ok(buffer.delete(&nvim::opts::BufDeleteOpts::default())?)
}
