/*------------------------------------ CONFIGURATION LAUNCHER ------------------------------------*/

use crate::config;
use crate::core::task;
use crate::utils;
use ::nvim_oxi::api as nvim;
use ::nvim_oxi::api::opts::{BufDeleteOpts, OptionOpts, SetKeymapOpts};
use ::nvim_oxi::api::types::Mode;
use ::nvim_oxi::Array;

fn launch_task() -> ::nvim_oxi::Result<()> {
    let launcher_win = nvim::get_current_win();
    let index = launcher_win.get_cursor()?.0 - 2;
    let config = config::state!().list[index].clone().into();
    ::nvim_oxi::dbg!(&config);

    launcher_win.get_buf()?.delete(&BufDeleteOpts::default())?;
    // TODO: add a WinClosed autocommand to wipeout the launcher buffer or cache-reuse buffer ID
    // launcher_win.close(true)?;
    task::run(config)
}

pub(crate) fn open() -> ::nvim_oxi::Result<()> {
    let configs = &config::state!().list;

    let mut buffer = nvim::create_buf(false, true)?;
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();
    nvim::set_option_value("filetype", "launch_nvim_launcher", &opts)?;
    buffer.set_lines(1..1, true, configs.iter().map(|c| format!("    {}    ", c.name())))?;
    let opts = SetKeymapOpts::builder()
        .desc("Launch the config under cursor")
        .noremap(true)
        .callback(|_| {
            if let Err(e) = launch_task() {
                utils::notify::send!(Warn: {format!("{e}")});
            }
        })
        .build();
    buffer.set_keymap(Mode::Normal, "<CR>", "", &opts)?;
    buffer.set_var("bounds", Array::from((2, configs.len() as u32 + 1)))?;

    let height = configs.len() as u32 + 2;
    let width = configs.iter().map(|c| c.name().len()).max().unwrap() as u32 + 8;
    let mut window = utils::open_float("Task Launcher", &buffer, width, height)?;
    window.set_cursor(2, 0)?;

    Ok(())
}
