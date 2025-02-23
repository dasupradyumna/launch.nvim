/*------------------------------------ CONFIGURATION LAUNCHER ------------------------------------*/

use crate::config;
use crate::utils;
use ::nvim_oxi::api as nvim;
use ::nvim_oxi::api::opts::OptionOpts;

pub(crate) fn open() -> ::nvim_oxi::Result<()> {
    let configs = &config::state!().list;

    let mut buffer = nvim::create_buf(false, true)?;
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();
    nvim::set_option_value("filetype", "launch_nvim_launcher", &opts)?;
    buffer.set_lines(1..1, true, configs.iter().map(|c| format!("    {}    ", c.name())))?;

    let height = configs.len() as u32 + 2;
    let width = configs.iter().map(|c| c.name().len()).max().unwrap() as u32 + 8;
    utils::open_float("Task Launcher", &buffer, width, height)?;

    Ok(())
}
