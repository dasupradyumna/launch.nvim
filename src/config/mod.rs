/*------------------------------------ RUNTIME CONFIGURATIONS ------------------------------------*/

mod task;

pub(crate) use task::*;

use crate::core::plugin;
use ::nvim_oxi::api as nvim;
use ::regex::Regex;
use ::serde_json as json;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::LazyLock;

pub(crate) fn data_dir() -> &'static PathBuf {
    static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        let ret: String = nvim::call_function("stdpath", ("data",)).unwrap();
        let mut ret = PathBuf::from(ret);
        ret.push("launch_nvim_rust");

        ret
    });

    &DATA_DIR
}

pub(crate) fn open_file() -> File {
    // get the JSON filename for the current working directory
    // CHECK: if below replace() call can be included into regex pattern
    let json_filename = std::env::current_dir().unwrap().to_string_lossy().replace("@", "@@");
    let re = Regex::new(r"[\\/:]").unwrap();
    let json_filename = format!("{}.json", re.replace_all(&json_filename, "@"));

    // create if file does not exist and open it
    let json_filepath = data_dir().join(json_filename);
    ::nvim_oxi::dbg!(&json_filepath);
    if !json_filepath.is_file() {
        File::create_new(json_filepath).unwrap()
    } else {
        File::options().read(true).write(true).open(json_filepath).unwrap()
    }
}

pub(crate) fn load() -> json::Result<()> {
    let mut state = plugin::state!();
    let reader = BufReader::new(&state.runtime_file);
    let configs = json::from_reader(reader)?;
    state.configs = configs;

    Ok(())
}

pub(crate) fn save() -> json::Result<()> {
    let state = plugin::state!();
    let writer = BufWriter::new(&state.runtime_file);
    json::to_writer(writer, &state.configs)
}
