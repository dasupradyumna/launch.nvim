/*------------------------------------ CONFIGURATION LAUNCHER ------------------------------------*/

use crate::config;
use crate::core::task;
use crate::utils;
use ::nvim_oxi::api::opts::{ExecAutocmdsOpts, OptionOpts};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};
use ::nvim_oxi::Array;
use nvim_oxi::{Dictionary, Function};

utils::setup_module_state!(launcher,
{
    buffer: Option<Buffer> = None,
    window: Option<Window> = None,
});

fn run() -> utils::Result<()> {
    let index = Window::current().get_cursor()?.0 - 2;
    self::close()?;

    let config = config::state!().list[index].clone().into();
    ::nvim_oxi::dbg!(&config);
    Ok(task::run(config)?)
}

fn close() -> utils::Result<()> {
    let window = unsafe { self::state!().window.clone().unwrap_unchecked() };
    Ok(window.close(true)?)
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
        let callback_dict =
            Dictionary::from_iter([("run", wrap_function(run)), ("close", wrap_function(close))]);
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
    if state.window.is_none() {
        let height = n + 2;
        let width = configs.iter().map(|c| c.name().len()).max().unwrap() as u32 + 8;
        state.window = Some(utils::open_float("Task Launcher", &buffer, width, height)?);

        nvim::exec_autocmds(
            ["User"],
            &ExecAutocmdsOpts::builder()
                .group("launch_nvim")
                .patterns("LaunchNvimLauncherWindowCreated")
                .build(),
        )?;
    }
    unsafe {
        state.window.as_mut().unwrap_unchecked().set_cursor(2, 0)?;
    }

    Ok(())
}

pub(crate) fn on_bufwipeout() {
    self::state!().buffer = None;
}

pub(crate) fn on_winclosed() {
    self::state!().window = None;
}
