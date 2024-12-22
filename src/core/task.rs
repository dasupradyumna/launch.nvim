/*------------------------------------------ TASK RUNNER -----------------------------------------*/

use crate::config::{TaskConfig, TaskDisplay};
use crate::settings::SettingsTask;
use crate::utils::notify;
use ::nvim_oxi::api::opts::{CreateAutocmdOpts, OptionOpts};
use ::nvim_oxi::api::types::{Mode, SplitDirection, WindowConfig};
use ::nvim_oxi::api::{self, Buffer, Window};

fn render(buffer: &Buffer, display: &TaskDisplay) -> ::nvim_oxi::Result<Window> {
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
pub(crate) fn run(settings: &SettingsTask, config: TaskConfig) -> ::nvim_oxi::Result<()> {
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
    let command = config.command();
    let term_options = config.term_options();
    // ::nvim_oxi::dbg!(&command);
    // ::nvim_oxi::dbg!(&term_options);
    let _job: i32 = api::call_function("termopen", (command, term_options))?;

    // enter insert mode after launching the task
    if settings.insert_mode_on_launch {
        api::feedkeys("i", Mode::Normal, false);
    }

    Ok(())
}
