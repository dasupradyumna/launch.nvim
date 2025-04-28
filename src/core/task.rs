/*------------------------------------------ TASK RUNNER -----------------------------------------*/

use crate::config::{TaskConfig, TaskDisplay};
use crate::settings::state as settings;
use crate::utils::{buffer, float, nvim_set_local, setup_module_state, IndexChecked, Result};
use ::chrono::{DateTime, Local};
use ::nvim_oxi::api::opts::{BufDeleteOpts, ExecAutocmdsOpts, OptionOpts};
use ::nvim_oxi::api::types::{Mode, SplitDirection, WindowConfig};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};
use std::collections::HashMap;

setup_module_state!(core::task,
{
    active_list: Vec<ActiveTask> = Vec::new(),
    windows: HashMap<TaskDisplay, Option<Window>> = HashMap::new(),
});

pub(crate) fn run(config: TaskConfig) -> Result<()> {
    // Open the task window and launch a terminal buffer with current config
    let buffer = buffer::create_scratch("task")?;
    let index = {
        let active_tasks = &mut self::state!().active_list;
        active_tasks.push(ActiveTask::new(buffer, config, Local::now()));
        active_tasks.len() - 1
    };
    ActiveTask::render(index)?;
    ActiveTask::run(index)
}

pub(crate) fn on_bufwipeout(buffer: i32) {
    let active_tasks = &mut self::state!().active_list;
    if let Some(idx) = active_tasks.iter().position(|e| e.buffer.handle() == buffer) {
        active_tasks.remove(idx);
    }
}

pub(crate) fn on_winclosed(display: ::nvim_oxi::String) {
    let display = display.to_string().as_str().into();
    self::state!().windows.insert(display, None);
}

pub(crate) fn on_dirchanged() {
    let state = &mut self::state!();
    for task in state.active_list.drain(..) {
        _ = task.buffer.delete(&BufDeleteOpts::default());
    }
    state.windows.clear();
}

#[derive(Debug, Clone)]
pub(crate) struct ActiveTask {
    title: String,
    start_time: DateTime<Local>,
    buffer: Buffer,
    config: TaskConfig,
}

impl ActiveTask {
    fn new(buffer: Buffer, config: TaskConfig, start_time: DateTime<Local>) -> Self {
        Self {
            title: format!("{} ({})", config.name(), start_time.format("%T")),
            start_time,
            buffer,
            config,
        }
    }

    pub(crate) fn is_list_empty() -> bool {
        self::state!().active_list.is_empty()
    }

    pub(crate) fn get_lines_for_ui() -> Vec<String> {
        const NO_TASKS_MSG: &str = "-- No active tasks --";

        let active_tasks = &self::state!().active_list;
        if active_tasks.is_empty() {
            vec![NO_TASKS_MSG.into()]
        } else {
            active_tasks.iter().map(|task| task.title.clone()).collect()
        }
    }

    pub(crate) fn render(index: usize) -> Result<()> {
        let mut state = self::state!();
        let active_task = state.active_list.get_checked(index)?;
        let display = active_task.config.disp().clone();

        if let Some(Some(window)) = state.windows.get(&display) {
            nvim::set_current_win(window)?;
            nvim_set_local(window, "winfixbuf", false)?;
            nvim::set_current_buf(&active_task.buffer)?;
            nvim_set_local(window, "winfixbuf", true)?;
        } else {
            let ui_settings = &settings!().task.ui;

            let screen_width: u32 = nvim::get_option_value("columns", &OptionOpts::default())?;
            let screen_height: u32 = nvim::get_option_value("lines", &OptionOpts::default())?;

            let mut window = match active_task.config.disp() {
                TaskDisplay::Float => {
                    let size = ui_settings.float.size as u32;
                    float::open_centered(
                        &active_task.title,
                        &active_task.buffer,
                        screen_width * size / 100,
                        screen_height * size / 100,
                    )?
                },
                TaskDisplay::VSplit => {
                    let win_config = WindowConfig::builder()
                        .split(SplitDirection::Right)
                        .width(screen_width * ui_settings.vsplit_width as u32 / 100)
                        .build();
                    nvim::open_win(&active_task.buffer, true, &win_config)?
                },
                TaskDisplay::HSplit => {
                    let win_config = WindowConfig::builder()
                        .split(SplitDirection::Below)
                        .height(screen_height * ui_settings.hsplit_height as u32 / 100)
                        .build();
                    nvim::open_win(&active_task.buffer, true, &win_config)?
                },
            };

            nvim_set_local(&window, "winfixbuf", true)?;
            nvim_set_local(&window, "signcolumn", "yes:1")?;
            window.set_var("taskdisplay", display.to_string())?;

            nvim::exec_autocmds(
                ["User"],
                &ExecAutocmdsOpts::builder()
                    .group("launch_nvim")
                    .patterns("LaunchNvimTaskWindowCreated")
                    .build(),
            )?;

            state.windows.insert(display, Some(window));
        }

        Ok(())
    }

    pub(crate) fn run(index: usize) -> Result<()> {
        let mut state = self::state!();
        let active_task = state.active_list.get_mut_checked(index)?;
        nvim_set_local(&active_task.buffer, "modified", false)?;

        let command = active_task.config.command();
        let term_options = active_task.config.term_options();
        nvim::call_function::<_, i32>("termopen", (command, term_options))?;

        // Set buffer name to the task title
        let new_name = format!("[launch.nvim] {}", active_task.title);
        active_task.buffer.set_name(new_name)?;
        let old_buffer = Buffer::from(nvim::call_function::<_, i32>("bufnr", ("#",))?);
        if old_buffer.is_valid() {
            old_buffer.delete(&BufDeleteOpts::default())?;
        }

        // Enter insert mode after launching the task
        let task_settings = &settings!().task;
        if task_settings.insert_mode_on_launch {
            nvim::feedkeys("i", Mode::Normal, false);
        }
        Ok(())
    }
}
