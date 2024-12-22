/*------------------------------------------ TASK RUNNER -----------------------------------------*/

use super::plugin::plugin as state;
use crate::config::{TaskConfig, TaskDisplay};
use crate::utils::notify;
use ::nvim_oxi::api::opts::{CreateAutocmdOpts, OptionOpts};
use ::nvim_oxi::api::types::{Mode, SplitDirection, WindowConfig};
use ::nvim_oxi::api::{self, Buffer, Window};
use ::nvim_oxi::{Array, Result};

fn render(buffer: &Buffer, display: &TaskDisplay) -> Result<Window> {
    let mut config_builder = WindowConfig::builder();
    let config = match display {
        TaskDisplay::Float => config_builder
            .relative(api::types::WindowRelativeTo::Editor)
            .row(10)
            .col(10)
            .width(80)
            .height(30)
            .style(api::types::WindowStyle::Minimal)
            .border(api::types::WindowBorder::Rounded)
            .build(),
        TaskDisplay::VSplit => config_builder.split(SplitDirection::Right).width(40).build(),
        TaskDisplay::HSplit => config_builder.split(SplitDirection::Below).height(10).build(),
    };
    // ::nvim_oxi::dbg!(&config);
    let window = api::open_win(buffer, true, &config)?;

    Ok(window)
}

// TODO: next steps
// * save the window ID as part of the TaskDisplay variant, and save it ActiveTask list
pub(crate) fn run(config: TaskConfig) -> Result<()> {
    // create a new task buffer
    let buffer = api::create_buf(false, true)?;
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();
    api::set_option_value("filetype", "launch_nvim_task", &opts)?;
    let opts = CreateAutocmdOpts::builder()
        .desc("Remove task from plugin active task list when wiped out")
        .buffer(buffer.clone())
        .callback(|_| {
            notify::send!(Warn: "Closing task buffer");
            true
        })
        .group("launch_nvim")
        .build();
    api::create_autocmd(["BufWipeout"], &opts)?;

    // open the UI window and load the task buffer
    let _window = render(&buffer, &config.display)?;

    // launch the task in a terminal buffer
    // CHECK: if Array::from_iter can be removed altogether to use Into<Array> bound
    let command = config.command();
    let term_options = config.term_options();
    // ::nvim_oxi::dbg!(&command);
    // ::nvim_oxi::dbg!(&term_options);
    api::call_function::<_, i32>("termopen", Array::from_iter([command, term_options]))?;

    // enter insert mode after launching the task
    // FIX: state!() is hanging neovim instance
    // if state!().settings.task.insert_mode_on_launch {
    //     api::feedkeys("i", Mode::Normal, false);
    // }

    Ok(())
}
