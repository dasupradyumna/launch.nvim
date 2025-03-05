/*------------------------------------ LAUNCHER : SELECT MODE ------------------------------------*/

use crate::config;
use crate::utils;
use ::nvim_oxi::api::opts::OptionOpts;
use ::nvim_oxi::api::types::{WindowConfig, WindowRelativeTo};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};
use ::nvim_oxi::{Array, Dictionary};
use nvim_oxi::Function;

pub(super) fn enter(mut buffer: Buffer, mut window: Window) -> utils::Result<()> {
    let configs = &config::state!().list;

    // List all configurations in select mode
    let range = 1..buffer.line_count()?;
    let lines = if configs.is_empty() {
        Vec::from_iter([config::NO_CONFIGS_MSG])
    } else {
        configs.iter().map(|c| c.name().as_str()).collect()
    };
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();
    nvim::set_option_value("modifiable", true, &opts)?;
    buffer.set_lines(range, true, lines.iter().map(|s| format!("    {s}    ")))?;
    nvim::set_option_value("modifiable", false, &opts)?;

    // Set navigation bounds
    let n = lines.len() as u32;
    buffer.set_var("bounds", Array::from((2, n + 1)))?;

    // Modify window size
    let height = n + 2;
    let width = lines.iter().map(|l| l.len()).max().unwrap() as u32 + 8;
    let (row, col) = utils::get_float_position(width, height)?;
    let win_config = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .row(row)
        .col(col)
        .width(width)
        .height(height)
        .build();
    window.set_config(&win_config)?;
    window.set_cursor(2, 0)?;
    let opts = OptionOpts::builder().win(window.clone()).build();
    nvim::set_option_value("cursorline", !configs.is_empty(), &opts)?;

    // Set callbacks for current mode
    let close = {
        let window = window.clone();
        wrap_function(move || {
            window.close(true)?;
            config::save()?;

            super::state!().update(super::Status::Closed)
        })
    };
    let callback_dict = Dictionary::from_iter([("q", close)]);
    buffer.set_var("callbacks", callback_dict)?;

    Ok(())
}

fn wrap_function<F>(func: F) -> Function<(), ()>
where
    F: FnOnce() -> utils::Result<()> + 'static,
{
    Function::from_fn_once(move |_| {
        if let Err(e) = func() {
            utils::notify::send!(Warn: {format!("{e}")});
        }
    })
}

pub(super) fn exit() {}
