/*------------------------------------------ TASK RUNNER -----------------------------------------*/

use super::plugin;
use crate::config::{TaskConfig, TaskDisplay};
use crate::utils;
use ::nvim_oxi::api::opts::{ExecAutocmdsOpts, OptionOpts};
use ::nvim_oxi::api::types::{
    Mode, SplitDirection, WindowBorder, WindowConfig, WindowRelativeTo, WindowStyle, WindowTitle,
    WindowTitlePosition,
};
use ::nvim_oxi::api::{self as nvim, Buffer};

#[derive(Debug)]
pub(crate) struct ActiveTask {
    buffer: Buffer,
    config: TaskConfig,
}

fn render(buffer: &Buffer, config: &TaskConfig) -> ::nvim_oxi::Result<()> {
    let mut state = plugin::state!();
    let display_id = config.display() as usize;

    if let Some(ref window) = state.task.windows[display_id] {
        let opts = OptionOpts::builder().win(window.clone()).build();

        nvim::set_current_win(window)?;
        nvim::set_option_value("winfixbuf", false, &opts)?;
        nvim::set_current_buf(buffer)?;
        nvim::set_option_value("winfixbuf", true, &opts)?;
    } else {
        let ui_settings = &state.settings.task.ui;

        let screen_width: u32 = nvim::get_option_value("columns", &OptionOpts::default())?;
        let screen_height: u32 = nvim::get_option_value("lines", &OptionOpts::default())?;

        let mut config_builder = WindowConfig::builder();
        let win_config = match config.display() {
            TaskDisplay::Float => {
                let [row, col, width, height] = utils::get_float_position_size(
                    ui_settings.float.size as u32,
                    screen_height,
                    screen_width,
                );
                config_builder
                    .relative(WindowRelativeTo::Editor)
                    .title(WindowTitle::SimpleString(format!(" {} ", config.name()).into()))
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
                let width = (screen_width * ui_settings.vsplit_width as u32) / 100;
                config_builder.split(SplitDirection::Right).width(width).build()
            },
            TaskDisplay::HSplit => {
                let height = (screen_height * ui_settings.hsplit_height as u32) / 100;
                config_builder.split(SplitDirection::Below).height(height).build()
            },
        };

        // ::nvim_oxi::dbg!(&config);
        let mut window = nvim::open_win(buffer, true, &win_config)?;
        let opts = OptionOpts::builder().win(window.clone()).build();
        nvim::set_option_value("winfixbuf", true, &opts)?;
        window.set_var("launch_nvim_taskdisplay", display_id)?;

        nvim::exec_autocmds(
            ["User"],
            &ExecAutocmdsOpts::builder()
                .group("launch_nvim")
                .patterns("LaunchNvimTaskWindowCreated")
                .build(),
        )?;

        state.task.windows[display_id] = Some(window);
    }

    Ok(())
}

pub(crate) fn run(config: TaskConfig) -> ::nvim_oxi::Result<ActiveTask> {
    // create a new task buffer
    let buffer = nvim::create_buf(false, true)?;
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();
    nvim::set_option_value("filetype", "launch_nvim_task", &opts)?;

    // open the UI window and load the task buffer
    render(&buffer, &config)?;

    // launch the task in a terminal buffer
    // TODO: handle failure here with a default command that displays an error message
    let command = config.command();
    let term_options = config.term_options();
    // ::nvim_oxi::dbg!(&command);
    // ::nvim_oxi::dbg!(&term_options);
    let _job: i32 = nvim::call_function("termopen", (command, term_options))?;

    // enter insert mode after launching the task
    let task_settings = &plugin::state!().settings.task;
    if task_settings.insert_mode_on_launch {
        nvim::feedkeys("i", Mode::Normal, false);
    }

    Ok(ActiveTask { buffer, config })
}

pub(crate) fn on_bufwipeout(buffer: i32) {
    let active_tasks = &mut plugin::state!().task.active_list;
    if let Some(idx) = active_tasks.iter().position(|e| e.buffer.handle() == buffer) {
        active_tasks.swap_remove(idx);
    }
}

pub(crate) fn on_winclosed(display_id: i32) {
    plugin::state!().task.windows[display_id as usize] = None;
}
