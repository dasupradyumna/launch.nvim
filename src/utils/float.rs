/*----------------------------------- FLOATING WINDOW UTILITIES ----------------------------------*/

use super::Result;
use ::nvim_oxi::api::opts::OptionOpts;
use ::nvim_oxi::api::{self as nvim, Buffer, Window};
use ::nvim_oxi::Function;

pub(crate) fn get_position(width: u32, height: u32) -> Result<(u32, u32)> {
    // Compute top-left row and column for a centered floating window
    let screen_height: u32 = nvim::get_option_value("lines", &OptionOpts::default())?;
    let screen_width: u32 = nvim::get_option_value("columns", &OptionOpts::default())?;
    let row = (screen_height - height) / 2 - 2;
    let col = (screen_width - width) / 2 - 2;

    Ok((row, col))
}

pub(crate) fn centered(title: &str, buffer: &Buffer, width: u32, height: u32) -> Result<Window> {
    use ::nvim_oxi::api::types::*;

    let (row, col) = self::get_position(width, height)?;
    let win_config = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .row(row)
        .col(col)
        .width(width)
        .height(height)
        .title(WindowTitle::SimpleString(format!(" {title} ").into()))
        .title_pos(WindowTitlePosition::Center)
        .footer(WindowTitle::SimpleString(" launch.nvim ".into()))
        .footer_pos(WindowTitlePosition::Right)
        .border(WindowBorder::Rounded)
        .style(WindowStyle::Minimal)
        .zindex(49)
        .build();
    let window = nvim::open_win(buffer, true, &win_config)?;

    // Fix the target buffer
    let opts = OptionOpts::builder().win(window.clone()).build();
    nvim::set_option_value("winfixbuf", true, &opts)?;

    Ok(window)
}

pub(crate) fn prompt(
    prompt: &str,
    default: &str,
    row: u32,
    col: u32,
    callback: Function<::nvim_oxi::String, ()>,
) -> Result<()> {
    let mut buffer = nvim::create_buf(false, true)?;
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();
    nvim::set_option_value("filetype", "launch_nvim_popup_prompt", &opts)?;

    use ::nvim_oxi::api::types::*;
    let win_config = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .row(row)
        .col(col)
        .width(40)
        .height(1)
        .border(WindowBorder::Rounded)
        .style(WindowStyle::Minimal)
        .zindex(49)
        .build();
    let window = nvim::open_win(&buffer, true, &win_config)?;

    // Fix the prompt buffer
    let opts = OptionOpts::builder().win(window.clone()).build();
    nvim::set_option_value("winfixbuf", true, &opts)?;

    // Update the prompt text
    buffer.set_var("prompt", prompt)?;
    buffer.set_var("default", default)?;
    buffer.set_var("callback", callback)?;
    nvim::command("call b:update_prompt()")?;

    Ok(())
}

pub(crate) fn select(
    items: Vec<&str>,
    row: u32,
    col: u32,
    callback: Function<(), ()>,
) -> Result<()> {
    let mut buffer = nvim::create_buf(false, true)?;
    buffer.set_lines(0..1, true, items.iter().map(|i| format!("  {i}  ")))?;
    buffer.set_var("callback", callback)?;
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();
    nvim::set_option_value("filetype", "launch_nvim_popup_select", &opts)?;

    use ::nvim_oxi::api::types::*;
    let win_config = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .row(row)
        .col(col)
        .width(unsafe { items.iter().map(|i| i.len() + 4).max().unwrap_unchecked() as u32 })
        .height(items.len() as u32)
        .border(WindowBorder::Rounded)
        .style(WindowStyle::Minimal)
        .zindex(49)
        .build();
    let window = nvim::open_win(&buffer, true, &win_config)?;

    // Fix the prompt buffer
    let opts = OptionOpts::builder().win(window.clone()).build();
    nvim::set_option_value("winfixbuf", true, &opts)?;
    nvim::set_option_value("cursorline", true, &opts)?;

    Ok(())
}
