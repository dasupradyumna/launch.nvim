/*------------------------------------ CONFIGURATION LAUNCHER ------------------------------------*/

use crate::config;
use ::nvim_oxi::api as nvim;
use ::nvim_oxi::api::opts::OptionOpts;
use ::nvim_oxi::api::types::{
    WindowBorder, WindowConfig, WindowRelativeTo, WindowStyle, WindowTitle, WindowTitlePosition,
};

pub(crate) fn open() -> ::nvim_oxi::Result<()> {
    let configs = &config::state!().list;

    let mut buffer = nvim::create_buf(false, true)?;
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();
    nvim::set_option_value("filetype", "launch_nvim_launcher", &opts)?;
    buffer.set_lines(1..1, true, configs.iter().map(|c| format!("    {}    ", c.name())))?;

    let screen_width: u32 = nvim::get_option_value("columns", &OptionOpts::default())?;
    let screen_height: u32 = nvim::get_option_value("lines", &OptionOpts::default())?;

    let height = configs.len() as u32 + 2;
    let width = configs.iter().map(|c| c.name().len()).max().unwrap() as u32 + 8;
    let row = (screen_height - height) / 2 - 2;
    let col = (screen_width - width) / 2 - 2;
    let mut config_builder = WindowConfig::builder();
    let win_config = config_builder
        .relative(WindowRelativeTo::Editor)
        .title(WindowTitle::SimpleString(" Task Launcher ".into()))
        .footer(WindowTitle::SimpleString(" launch.nvim ".into()))
        .row(row)
        .col(col)
        .width(width)
        .height(height)
        .style(WindowStyle::Minimal)
        .title_pos(WindowTitlePosition::Center) // TODO: move to settings
        .footer_pos(WindowTitlePosition::Right) // TODO: ...
        .border(WindowBorder::Rounded) // TODO: ...
        .zindex(49) // TODO: ...
        .build();
    let window = nvim::open_win(&buffer, true, &win_config)?;
    let opts = OptionOpts::builder().win(window.clone()).build();
    nvim::set_option_value("winfixbuf", true, &opts)?;

    Ok(())
}
