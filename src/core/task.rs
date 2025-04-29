/*------------------------------------------ TASK RUNNER -----------------------------------------*/
//!
//! This module contains structs and functions related to launching tasks. This includes the
//! callbacks for various autocommand events, and the primary function to launch tasks.
//! Defines the `ActiveTask` struct, which represents a task that has been launched, and its
//! associated methods for rendering and running tasks.

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

/// Launch the specified task
///
/// Creates a new ActiveTask, renders a task window, and launches a terminal buffer
/// Pushes the new task to the active task list
pub(crate) fn run(config: TaskConfig) -> Result<()> {
    let buffer = buffer::create_scratch("task")?;
    let index = {
        let active_tasks = &mut self::state!().active_list;
        active_tasks.push(ActiveTask::new(buffer, config, Local::now()));
        active_tasks.len() - 1
    };
    ActiveTask::render(index)?;
    ActiveTask::run(index)
}

/*--------------------------- AUTOCOMMAND CALLBACKS --------------------------*/

/// Callback logic for `BufWipeout` event
pub(crate) fn on_bufwipeout(buffer: i32) {
    let active_tasks = &mut self::state!().active_list;
    if let Some(idx) = active_tasks.iter().position(|e| e.buffer.handle() == buffer) {
        active_tasks.remove(idx);
    }
}

/// Callback logic for `WinClosed` event
pub(crate) fn on_winclosed(display: ::nvim_oxi::String) {
    let display = display.to_string().as_str().into();
    self::state!().windows.insert(display, None);
}

/// Callback logic for `DirChanged` event
pub(crate) fn on_dirchanged() {
    let state = &mut self::state!();
    for task in state.active_list.drain(..) {
        _ = task.buffer.delete(&BufDeleteOpts::default());
    }
    state.windows.clear();
}

/*---------------------------- ACTIVE TASK STRUCT ----------------------------*/

/// Represents a task that has been launched
///
/// Stores the task's title, start time, buffer, and config
#[derive(Debug, Clone)]
pub(crate) struct ActiveTask {
    title: String,
    start_time: DateTime<Local>,
    buffer: Buffer,
    config: TaskConfig,
}

impl ActiveTask {
    /// Creates a new `ActiveTask`
    fn new(buffer: Buffer, config: TaskConfig, start_time: DateTime<Local>) -> Self {
        Self {
            title: format!("{} ({})", config.name(), start_time.format("%T")),
            start_time,
            buffer,
            config,
        }
    }

    /// Checks if the active task list is empty
    pub(crate) fn is_list_empty() -> bool {
        self::state!().active_list.is_empty()
    }

    /// Returns the list of active tasks for the UI
    pub(crate) fn get_lines_for_ui() -> Vec<String> {
        const NO_TASKS_MSG: &str = "-- No active tasks --";

        let active_tasks = &self::state!().active_list;
        if active_tasks.is_empty() {
            vec![NO_TASKS_MSG.into()]
        } else {
            active_tasks.iter().map(|task| task.title.clone()).collect()
        }
    }

    /// Renders the task at the specified index in the active task list, based on its display mode
    pub(crate) fn render(index: usize) -> Result<()> {
        let mut state = self::state!();
        let active_task = state.active_list.get_checked(index)?;
        let display = active_task.config.disp().clone();

        // If the required window already exists, switch to it
        if let Some(Some(window)) = state.windows.get(&display) {
            nvim::set_current_win(window)?;
            nvim_set_local(window, "winfixbuf", false)?;
            nvim::set_current_buf(&active_task.buffer)?;
            nvim_set_local(window, "winfixbuf", true)?;

        // If the required window doesn't exist, create it using the display mode
        } else {
            let ui_settings = &settings!().task.ui;

            let screen_width: u32 = nvim::get_option_value("columns", &OptionOpts::default())?;
            let screen_height: u32 = nvim::get_option_value("lines", &OptionOpts::default())?;

            let mut window = match active_task.config.disp() {
                // Open a floating window centered in the editor
                TaskDisplay::Float => {
                    let size = ui_settings.float.size as u32;
                    float::open_centered(
                        &active_task.title,
                        &active_task.buffer,
                        screen_width * size / 100,
                        screen_height * size / 100,
                    )?
                },
                // Vertically split the current window
                TaskDisplay::VSplit => {
                    let win_config = WindowConfig::builder()
                        .split(SplitDirection::Right)
                        .width(screen_width * ui_settings.vsplit_width as u32 / 100)
                        .build();
                    nvim::open_win(&active_task.buffer, true, &win_config)?
                },
                // Horizontally split the current window
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

            // Trigger `LaunchNvimTaskWindowCreated` event
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

    /// Launches the task at the specified index in the active task list
    pub(crate) fn run(index: usize) -> Result<()> {
        let mut state = self::state!();
        let active_task = state.active_list.get_mut_checked(index)?;
        nvim_set_local(&active_task.buffer, "modified", false)?;

        // Launch the task
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
