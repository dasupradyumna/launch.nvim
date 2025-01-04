/*------------------------------------------ TASK RUNNER -----------------------------------------*/

use super::plugin;
use crate::config::{TaskConfig, TaskDisplay, TaskDisplayFloatSize};
use ::nvim_oxi::api::opts::OptionOpts;
use ::nvim_oxi::api::types::{
    Mode, SplitDirection, WindowBorder, WindowConfig, WindowRelativeTo, WindowStyle, WindowTitle,
    WindowTitlePosition,
};
use ::nvim_oxi::api::{self, Buffer};

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

fn render(buffer: &Buffer, config: &TaskConfig) -> ::nvim_oxi::Result<()> {
    let mut state = plugin::state!();

    // TODO: 1. replace with if-let, 2. autocommand that removes cached window
    match &state.task.windows[config.display as usize] {
        Some(window) => {
            let opts = OptionOpts::builder().win(window.clone()).build();

            api::set_current_win(window)?;
            api::set_option_value("winfixbuf", false, &opts)?;
            api::set_current_buf(buffer)?;
            api::set_option_value("winfixbuf", true, &opts)?;
        },
        None => {
            let ui_settings = &state.settings.task.ui;

            let screen_w: u32 = api::get_option_value("columns", &OptionOpts::default())?;
            let screen_h: u32 = api::get_option_value("lines", &OptionOpts::default())?;

            let mut config_builder = WindowConfig::builder();
            let win_config = match config.display {
                TaskDisplay::Float => {
                    let [r, c, w, h] = get_float_specs(ui_settings.float.size, screen_h, screen_w);
                    config_builder
                    .relative(WindowRelativeTo::Editor)
                    .title(WindowTitle::SimpleString(format!(" {} ", &config.name).into()))
                    .footer(WindowTitle::SimpleString(" launch.nvim ".into()))
                    .row(r)
                    .col(c)
                    .width(w)
                    .height(h)
                    .style(WindowStyle::Minimal)
                    .title_pos(WindowTitlePosition::Center) // TODO: move to settings
                    .footer_pos(WindowTitlePosition::Right) // TODO: ...
                    .border(WindowBorder::Rounded) // TODO: ...
                    .zindex(49) // TODO: ...
                    .build()
                },
                TaskDisplay::VSplit => {
                    let w = (screen_w * ui_settings.vsplit_width as u32) / 100;
                    config_builder.split(SplitDirection::Right).width(w).build()
                },
                TaskDisplay::HSplit => {
                    let h = (screen_h * ui_settings.hsplit_height as u32) / 100;
                    config_builder.split(SplitDirection::Below).height(h).build()
                },
            };

            // ::nvim_oxi::dbg!(&config);
            let window = api::open_win(buffer, true, &win_config)?;
            let opts = OptionOpts::builder().win(window.clone()).build();
            api::set_option_value("winfixbuf", true, &opts)?;

            state.task.windows[config.display as usize] = Some(window);
        },
    }

    Ok(())
}

pub(crate) fn run(config: TaskConfig) -> ::nvim_oxi::Result<ActiveTask> {
    // create a new task buffer
    let buffer = api::create_buf(false, true)?;
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();
    api::set_option_value("filetype", "launch_nvim_task", &opts)?;

    // open the UI window and load the task buffer
    render(&buffer, &config)?;

    // launch the task in a terminal buffer
    // TODO: handle failure here with a default command that displays an error message
    let command = config.command();
    let term_options = config.term_options();
    // ::nvim_oxi::dbg!(&command);
    // ::nvim_oxi::dbg!(&term_options);
    let _job: i32 = api::call_function("termopen", (command, term_options))?;

    // enter insert mode after launching the task
    let task_settings = &plugin::state!().settings.task;
    if task_settings.insert_mode_on_launch {
        api::feedkeys("i", Mode::Normal, false);
    }

    Ok(ActiveTask { buffer, config })
}

pub(crate) fn on_bufwipeout(buffer: i32) {
    let active_tasks = &mut plugin::state!().task.active_list;
    if let Some(idx) = active_tasks.iter().position(|e| e.buffer.handle() == buffer) {
        active_tasks.swap_remove(idx);
    }
}
