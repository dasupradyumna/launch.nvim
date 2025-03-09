/*------------------------------------------ TASK RUNNER -----------------------------------------*/

use crate::config::{TaskConfig, TaskDisplay};
use crate::settings::state as settings;
use crate::utils;
use ::nvim_oxi::api::opts::{ExecAutocmdsOpts, OptionOpts};
use ::nvim_oxi::api::types::{Mode, SplitDirection, WindowConfig};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};

utils::setup_module_state!(core::task, [pub(crate)]
{
    active_list: Vec<ActiveTask> = Vec::new(),
    windows: [Option<Window>; 3] = [const { None }; 3],
});

pub(crate) fn run(config: TaskConfig) -> utils::Result<()> {
    // create a new task buffer
    let buffer = nvim::create_buf(false, true)?;
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();
    nvim::set_option_value("filetype", "launch_nvim_task", &opts)?;

    // open the task window and launch a terminal buffer with current config
    let task = ActiveTask { buffer, config };
    task.render()?;
    task.run()?;

    // enter insert mode after launching the task
    let task_settings = &settings!().task;
    if task_settings.insert_mode_on_launch {
        nvim::feedkeys("i", Mode::Normal, false);
    }

    self::state!().active_list.push(task);
    Ok(())
}

#[derive(Debug)]
pub(crate) struct ActiveTask {
    buffer: Buffer,
    config: TaskConfig,
}

impl ActiveTask {
    fn render(&self) -> utils::Result<()> {
        let task_windows = &mut self::state!().windows;
        let display_id = self.config.display() as usize;

        if let Some(ref window) = task_windows[display_id] {
            let opts = OptionOpts::builder().win(window.clone()).build();

            nvim::set_current_win(window)?;
            nvim::set_option_value("winfixbuf", false, &opts)?;
            nvim::set_current_buf(&self.buffer)?;
            nvim::set_option_value("winfixbuf", true, &opts)?;
        } else {
            let ui_settings = &settings!().task.ui;

            let screen_width: u32 = nvim::get_option_value("columns", &OptionOpts::default())?;
            let screen_height: u32 = nvim::get_option_value("lines", &OptionOpts::default())?;

            let mut window = match self.config.display() {
                TaskDisplay::Float => {
                    let size = ui_settings.float.size as u32;
                    utils::open_float(
                        self.config.name(),
                        &self.buffer,
                        screen_width * size / 100,
                        screen_height * size / 100,
                    )?
                },
                TaskDisplay::VSplit => {
                    let win_config = WindowConfig::builder()
                        .split(SplitDirection::Right)
                        .width(screen_width * ui_settings.vsplit_width as u32 / 100)
                        .build();
                    nvim::open_win(&self.buffer, true, &win_config)?
                },
                TaskDisplay::HSplit => {
                    let win_config = WindowConfig::builder()
                        .split(SplitDirection::Below)
                        .height(screen_height * ui_settings.hsplit_height as u32 / 100)
                        .build();
                    nvim::open_win(&self.buffer, true, &win_config)?
                },
            };

            // ::nvim_oxi::dbg!(&self.config);
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

            task_windows[display_id] = Some(window);
        }

        Ok(())
    }

    fn run(&self) -> utils::Result<i32> {
        // TODO: handle failure here with a default command that displays an error message
        let command = self.config.command();
        let term_options = self.config.term_options();
        // ::nvim_oxi::dbg!(&command);
        // ::nvim_oxi::dbg!(&term_options);

        Ok(nvim::call_function("termopen", (command, term_options))?)
    }
}

pub(crate) fn on_bufwipeout(buffer: i32) {
    let active_tasks = &mut self::state!().active_list;
    if let Some(idx) = active_tasks.iter().position(|e| e.buffer.handle() == buffer) {
        active_tasks.swap_remove(idx);
    }
}

pub(crate) fn on_winclosed(display_id: i32) {
    self::state!().windows[display_id as usize] = None;
}
