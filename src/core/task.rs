/*------------------------------------------ TASK RUNNER -----------------------------------------*/

use super::plugin;
use crate::config::{TaskConfig, TaskDisplay, TaskDisplayFloatSize};
use crate::settings::SettingsTaskUI;
use crate::utils::notify;
use ::nvim_oxi::api::opts::{CreateAutocmdOpts, OptionOpts};
use ::nvim_oxi::api::types::{
    AutocmdCallbackArgs, Mode, SplitDirection, WindowBorder, WindowConfig, WindowRelativeTo,
    WindowStyle, WindowTitle, WindowTitlePosition,
};
use ::nvim_oxi::api::{self, Buffer, Window};
use nvim_oxi::Function;

#[derive(Debug)]
pub(crate) struct ActiveTask {
    buffer: Buffer,
    config: TaskConfig,
}

fn get_float_specs(size: TaskDisplayFloatSize, lines: u32, columns: u32) -> [u32; 4] {
    let width = columns * (size as u32) / 100;
    let height = lines * (size as u32) / 100;
    let col = (columns - width) / 2 - 2;
    let row = (lines - height) / 2 - 2;

    [row, col, width, height]
}

fn render(
    ui_settings: &SettingsTaskUI,
    name: &str,
    buffer: &Buffer,
    display: &TaskDisplay,
) -> ::nvim_oxi::Result<Window> {
    let columns: u32 = api::get_option_value("columns", &OptionOpts::default())?;
    let lines: u32 = api::get_option_value("lines", &OptionOpts::default())?;

    let mut config_builder = WindowConfig::builder();
    let config = match display {
        TaskDisplay::Float => {
            let [row, col, width, height] = get_float_specs(ui_settings.float.size, lines, columns);
            config_builder
                .relative(WindowRelativeTo::Editor)
                .title(WindowTitle::SimpleString(format!(" {name} ").into()))
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
                .build()
        },
        TaskDisplay::VSplit => {
            let w = (columns * ui_settings.vsplit_width as u32) / 100;
            config_builder.split(SplitDirection::Right).width(w).build()
        },
        TaskDisplay::HSplit => {
            let h = (lines * ui_settings.hsplit_height as u32) / 100;
            config_builder.split(SplitDirection::Below).height(h).build()
        },
    };
    // ::nvim_oxi::dbg!(&config);
    let window = api::open_win(buffer, true, &config)?;
    let opts = OptionOpts::builder().win(window.clone()).build();
    api::set_option_value("winfixbuf", true, &opts)?;

    Ok(window)
}

pub(crate) fn run(config: TaskConfig) -> ::nvim_oxi::Result<ActiveTask> {
    let task_settings = &plugin::state!().settings.task;

    // create a new task buffer
    let buffer = api::create_buf(false, true)?;
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();
    api::set_option_value("filetype", "launch_nvim_task", &opts)?;
    // TODO: refactor this into ftplugin logic, with exposed internal functions
    let opts = CreateAutocmdOpts::builder()
        .desc("Remove task from plugin active task list when wiped out")
        .buffer(buffer.clone())
        .callback(Function::from_fn_mut(|args: AutocmdCallbackArgs| {
            notify::send!(Warn: "Closing task buffer");
            let active_tasks = &mut plugin::state!().active_tasks;
            if let Some(idx) = active_tasks.iter().position(|e| e.buffer == args.buffer) {
                active_tasks.swap_remove(idx);
            }
            true
        }))
        .group("launch_nvim")
        .build();
    api::create_autocmd(["BufWipeout"], &opts)?;

    // open the UI window and load the task buffer
    let _window = render(&task_settings.ui, &config.name, &buffer, &config.display)?;

    // launch the task in a terminal buffer
    // TODO: handle failure here with a default command that displays an error message
    let command = config.command();
    let term_options = config.term_options();
    // ::nvim_oxi::dbg!(&command);
    // ::nvim_oxi::dbg!(&term_options);
    let _job: i32 = api::call_function("termopen", (command, term_options))?;

    // enter insert mode after launching the task
    if task_settings.insert_mode_on_launch {
        api::feedkeys("i", Mode::Normal, false);
    }

    Ok(ActiveTask { buffer, config })
}
