/*------------------------------------ RUNTIME CONFIGURATIONS ------------------------------------*/

mod task;

pub(crate) use task::*;

use crate::utils;
use ::nvim_oxi::api as nvim;
use ::regex::Regex;
use ::serde_json as json;
use std::path::PathBuf;
use std::sync::LazyLock;

utils::setup_module_state!(config,
{
    filepath: PathBuf = self::get_runtime_filepath(),
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

pub(crate) fn get_runtime_filepath() -> PathBuf {
    // get the JSON filename for the current working directory
    // CHECK: if below replace() call can be included into regex pattern
    let json_filename = std::env::current_dir().unwrap().to_string_lossy().replace("@", "@@");
    let re = Regex::new(r"[\\/:]").unwrap();
    let json_filename = format!("{}.json", re.replace_all(&json_filename, "@"));

    // create if file does not exist and open it
    let json_filepath = self::get_data_dir().join(json_filename);
    ::nvim_oxi::dbg!(&json_filepath);
    json_filepath
}

pub(crate) fn load() -> utils::Result<()> {
    let mut config = self::state!();
    if config.filepath.is_file() {
        let config_str = std::fs::read_to_string(&config.filepath)?;
        config.list = json::from_str(&config_str)?;
    }

    ::nvim_oxi::dbg!(&config.list);
    Ok(())
}

pub(crate) fn save() -> utils::Result<()> {
    let config = self::state!();
    let config_str = json::to_string_pretty(&config.list)?;
    std::fs::write(&config.filepath, config_str)?;

    Ok(())
}
