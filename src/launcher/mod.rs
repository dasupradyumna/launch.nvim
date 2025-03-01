/*------------------------------------ CONFIGURATION LAUNCHER ------------------------------------*/

use crate::config;
use crate::core::task;
use crate::utils;
use ::nvim_oxi::api::opts::{ExecAutocmdsOpts, OptionOpts};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};
use ::nvim_oxi::{Array, Dictionary, Function};

utils::setup_module_state!(launcher,
{
    buffer: Option<Buffer> = None,
    window: Option<Window> = None,
});

fn get_config_index() -> utils::Result<usize> {
    if let Some(ref window) = self::state!().window {
        Ok(window.get_cursor()?.0 - 2)
    } else {
        utils::Error::new("launcher::get_config_index() called when window ID is unset")
    }
}

fn delete_config() -> utils::Result<()> {
    {
        let index = self::get_config_index()?;
        config::state!().list.remove(index);
    }
    config::save()?;

    Ok(self::open()?)
}

fn run_config() -> utils::Result<()> {
    let index = self::get_config_index()?;
    self::close()?;

    let config = config::state!().list[index].clone().into();
    ::nvim_oxi::dbg!(&config);
    Ok(task::run(config)?)
}

fn close() -> utils::Result<()> {
    let window = unsafe { self::state!().window.clone().unwrap_unchecked() };
    window.close(true)?;

    Ok(config::save()?)
}

fn wrap_function(func: fn() -> utils::Result<()>) -> Function<(), ()> {
    Function::from_fn(move |_| {
        if let Err(e) = func() {
            utils::notify::send!(Warn: {format!("{e}")});
        }
    })
}

pub(crate) fn open() -> utils::Result<()> {
    let configs = &config::state!().list;
    let mut state = self::state!();

    // Create buffer for launcher UI
    if state.buffer.is_none() {
        let mut buffer = nvim::create_buf(false, true)?;
        let callback_dict = Dictionary::from_iter([
            ("delete", wrap_function(delete_config)),
            ("run", wrap_function(run_config)),
            ("close", wrap_function(close)),
        ]);
        buffer.set_var("callbacks", callback_dict)?;

        let opts = OptionOpts::builder().buffer(buffer.clone()).build();
        nvim::set_option_value("filetype", "launch_nvim_launcher", &opts)?;

        state.buffer = Some(buffer);
    }
    let mut buffer = unsafe { state.buffer.as_ref().unwrap_unchecked().clone() };

    // Write out configurations in the UI buffer
    let range = 1..buffer.line_count()?;
    buffer.set_lines(range, true, configs.iter().map(|c| format!("    {}    ", c.name())))?;

    // Set UI navigation bounds
    let n = configs.len() as u32;
    buffer.set_var("bounds", Array::from((2, n + 1)))?;

    // Open a new floating window if it does not exist already
    let height = n + 2;
    let width = configs.iter().map(|c| c.name().len()).max().unwrap() as u32 + 8;
    if state.window.is_none() {
        state.window = Some(utils::open_float("Task Launcher", &buffer, width, height)?);
        nvim::exec_autocmds(
            ["User"],
            &ExecAutocmdsOpts::builder()
                .group("launch_nvim")
                .patterns("LaunchNvimLauncherWindowCreated")
                .build(),
        )?;
    } else {
        use ::nvim_oxi::api::types::*;

        let (row, col) = utils::get_float_position(width, height)?;
        let window = unsafe { state.window.as_mut().unwrap_unchecked() };
        let win_config = WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .row(row)
            .col(col)
            .width(width)
            .height(height)
            .build();
        window.set_config(&win_config)?;
        window.set_cursor(2, 0)?;
    }

    Ok(())
}

pub(crate) fn on_bufwipeout() {
    self::state!().buffer = None;
}

pub(crate) fn on_winclosed() {
    self::state!().window = None;
}
