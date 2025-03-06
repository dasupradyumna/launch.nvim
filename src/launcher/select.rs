/*------------------------------------ LAUNCHER : SELECT MODE ------------------------------------*/

use crate::config;
use crate::core::task;
use crate::utils;
use ::nvim_oxi::api::opts::{BufDeleteOpts, OptionOpts};
use ::nvim_oxi::api::types::{WindowConfig, WindowRelativeTo};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};
use ::nvim_oxi::{Array, Function};

pub(super) fn enter(mut buffer: Buffer, mut window: Window) -> utils::Result<()> {
    self::update_ui(&mut buffer, &mut window)?;
    self::define_callbacks(buffer, window)?;

    Ok(())
}

pub(super) fn exit() -> utils::Result<()> {
    Ok(nvim::command("call b:remove_callbacks()")?)
}

fn update_ui(buffer: &mut Buffer, window: &mut Window) -> utils::Result<()> {
    let configs = &config::state!().list;

    // Display all configurations in the buffer
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
    // FIX: handle other navigation keymaps like wW, eE, bB etc.
    let n = lines.len() as u32;
    buffer.set_var("bounds", Array::from((2, n + 1)))?;

    // Modify window size to match current config list
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

    Ok(())
}

fn get_config_index(window: &Window) -> utils::Result<usize> {
    if config::state!().list.is_empty() {
        return utils::Error::new("No active configurations found.");
    }

    Ok(window.get_cursor()?.0 - 2)
}

fn wrap_callback<F>(key: &str, func: F) -> Array
where
    F: FnOnce() -> utils::Result<()> + 'static,
{
    let wrapped = Function::from_fn_once(move |()| {
        if let Err(e) = func() {
            utils::notify::send!(Warn: {format!("{e}")});
        }
    });

    Array::from((key, wrapped))
}

fn define_callbacks(mut buffer: Buffer, window: Window) -> utils::Result<()> {
    let close = {
        let buffer = buffer.clone();
        move || {
            buffer.delete(&BufDeleteOpts::builder().force(true).build())?;
            config::save()?;

            super::state!().update(super::Status::Closed)
        }
    };
    let run = {
        let close = close.clone();
        let window = window.clone();
        move || {
            let index = self::get_config_index(&window)?;
            close()?;

            let config = config::state!().list[index].clone().into();
            ::nvim_oxi::dbg!(&config);
            task::run(config)
        }
    };
    let delete = {
        let mut window = window.clone();
        let mut buffer = buffer.clone();
        move || {
            let index = self::get_config_index(&window)?;
            {
                config::state!().list.remove(index);
            }
            config::save()?;

            self::update_ui(&mut buffer, &mut window)
        }
    };

    let callback_dict = Array::from((
        wrap_callback("q", close),
        wrap_callback("<CR>", run),
        wrap_callback("d", delete),
    ));
    buffer.set_var("callbacks", callback_dict)?;

    Ok(nvim::command("call b:setup_callbacks()")?)
}
