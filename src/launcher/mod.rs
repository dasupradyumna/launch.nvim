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
        if config::state!().list.is_empty() {
            return utils::Error::new("No active configurations found.");
        }
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

    self::open()
}

fn run_config() -> utils::Result<()> {
    let index = self::get_config_index()?;
    self::close()?;

    let config = config::state!().list[index].clone().into();
    ::nvim_oxi::dbg!(&config);
    task::run(config)
}

fn close() -> utils::Result<()> {
    let window = unsafe { self::state!().window.clone().unwrap_unchecked() };
    window.close(true)?;

    config::save()
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
    let lines = if configs.is_empty() {
        Vec::from_iter([config::NO_CONFIGS_MSG])
    } else {
        configs.iter().map(|c| c.name().as_str()).collect()
    };
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();
    nvim::set_option_value("modifiable", true, &opts)?;
    buffer.set_lines(range, true, lines.iter().map(|s| format!("    {s}    ")))?;
    nvim::set_option_value("modifiable", false, &opts)?;

    // Set UI navigation bounds
    let n = lines.len() as u32;
    buffer.set_var("bounds", Array::from((2, n + 1)))?;

    // Open a new floating window if it does not exist already
    let height = n + 2;
    let width = lines.iter().map(|l| l.len()).max().unwrap() as u32 + 8;
    let mut window;
    if state.window.is_none() {
        window = utils::open_float("Task Launcher", &buffer, width, height)?;
        state.window = Some(window.clone());
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
        window = unsafe { state.window.clone().unwrap_unchecked() };
        let win_config = WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .row(row)
            .col(col)
            .width(width)
            .height(height)
            .build();
        window.set_config(&win_config)?;
    };

    window.set_cursor(2, 0)?;
    let opts = OptionOpts::builder().win(window).build();
    nvim::set_option_value("cursorline", !configs.is_empty(), &opts)?;

    Ok(())
}

pub(crate) fn on_bufwipeout() {
    self::state!().buffer = None;
}

pub(crate) fn on_winclosed() {
    self::state!().window = None;
}
