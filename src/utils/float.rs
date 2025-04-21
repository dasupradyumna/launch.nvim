/*----------------------------------- FLOATING WINDOW UTILITIES ----------------------------------*/

use super::{buffer, Result};
use ::nvim_oxi::api::opts::OptionOpts;
use ::nvim_oxi::api::{self as nvim, Buffer, Window};
use ::nvim_oxi::Function;

impl super::ScopeOpts for Window {
    fn opts(&self) -> OptionOpts {
        OptionOpts::builder().win(self.clone()).build()
    }
}

pub(crate) fn get_centered_position(width: u32, height: u32) -> Result<(u32, u32)> {
    // Compute top-left row and column for a centered floating window
    let screen_height: u32 = nvim::get_option_value("lines", &OptionOpts::default())?;
    let screen_width: u32 = nvim::get_option_value("columns", &OptionOpts::default())?;
    let row = (screen_height - height) / 2 - 2;
    let col = (screen_width - width) / 2 - 2;

    Ok((row, col))
}

pub(crate) fn config_builder(
    row: u32,
    col: u32,
    width: u32,
    height: u32,
) -> ::nvim_oxi::api::types::WindowConfigBuilder {
    use ::nvim_oxi::api::types::*;
    let mut win_config = WindowConfig::builder();
    win_config
        .relative(WindowRelativeTo::Editor)
        .row(row)
        .col(col)
        .width(width)
        .height(height)
        .border(WindowBorder::Rounded)
        .style(WindowStyle::Minimal)
        .zindex(49);
    win_config
}

pub(crate) fn open_centered(
    title: &str,
    buffer: &Buffer,
    width: u32,
    height: u32,
) -> Result<Window> {
    use ::nvim_oxi::api::types::*;

    let (row, col) = self::get_centered_position(width, height)?;
    let win_config = self::config_builder(row, col, width, height)
        .title(WindowTitle::SimpleString(format!(" {title} ").into()))
        .title_pos(WindowTitlePosition::Center)
        .footer(WindowTitle::SimpleString(" launch.nvim ".into()))
        .footer_pos(WindowTitlePosition::Right)
        .build();
    let window = nvim::open_win(buffer, true, &win_config)?;

    super::nvim_set_local(&window, "winfixbuf", true)?;
    Ok(window)
}

pub(crate) fn open_prompt(
    prompt: &str,
    default: &str,
    row: u32,
    col: u32,
    callback: Function<::nvim_oxi::String, ()>,
) -> Result<()> {
    // Create prompt buffer
    let mut buffer = buffer::create_scratch("popup_prompt")?;

    // Open the float and fix the buffer
    let win_config = self::config_builder(row, col, 40, 1).build();
    let window = nvim::open_win(&buffer, true, &win_config)?;
    super::nvim_set_local(&window, "winfixbuf", true)?;

    // Update the prompt text
    buffer.set_var("prompt", prompt)?;
    buffer.set_var("default", default)?;
    buffer.set_var("callback", callback)?;
    Ok(nvim::command("call b:update_prompt()")?)
}

pub(crate) fn open_select(
    items: Vec<&str>,
    row: u32,
    col: u32,
    callback: Function<(), ()>,
) -> Result<()> {
    // Create selection buffer
    let mut buffer = buffer::create_scratch("popup_select")?;
    buffer::write_lines(&mut buffer, &items)?;
    buffer.set_var("callback", callback)?; // TODO: separate into accept/cancel callbacks

    // Open the float and fix the buffer
    let width = unsafe { items.iter().map(|i| i.len() + 8).max().unwrap_unchecked() as u32 };
    let height = items.len() as u32;
    let win_config = self::config_builder(row, col, width, height).build();
    let window = nvim::open_win(&buffer, true, &win_config)?;
    super::nvim_set_local(&window, "winfixbuf", true)?;
    super::nvim_set_local(&window, "cursorline", true)
}

pub(crate) fn open_help(mut lines: Vec<String>, row: u32, col: u32) -> Result<()> {
    // Create help buffer
    let mut buffer = buffer::create_scratch("help")?;
    lines.insert(0, "".into());
    buffer::write_lines(&mut buffer, &lines)?;

    // Open the float and fix the buffer
    use ::nvim_oxi::api::types::*;
    let width = unsafe { lines.iter().map(|i| i.len() + 8).max().unwrap_unchecked() as u32 };
    let height = lines.len() as u32 + 1;
    let win_config = self::config_builder(row, col, width, height)
        .title(WindowTitle::SimpleString("Help".into()))
        .title_pos(WindowTitlePosition::Center)
        .build();
    let window = nvim::open_win(&buffer, true, &win_config)?;
    super::nvim_set_local(&window, "winfixbuf", true)
}
