/*------------------------------------------ TASK RUNNER -----------------------------------------*/

use crate::config::{TaskConfig, TaskDisplay};
use crate::settings::state as settings;
use crate::utils::{buffer, float, nvim_set_local, setup_module_state, Result};
use ::nvim_oxi::api::opts::{ExecAutocmdsOpts, OptionOpts};
use ::nvim_oxi::api::types::{Mode, SplitDirection, WindowConfig};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};

setup_module_state!(core::task,
{
    active_list: Vec<ActiveTask> = Vec::new(),
    windows: [Option<Window>; 3] = [const { None }; 3],
});

pub(crate) fn run(config: TaskConfig) -> Result<()> {
    // Open the task window and launch a terminal buffer with current config
    let buffer = buffer::create_scratch("task")?;
    let index = {
        let active_tasks = &mut self::state!().active_list;
        active_tasks.push(ActiveTask { buffer, config });
        active_tasks.len() - 1
    };
    ActiveTask::render(index)?;
    ActiveTask::run(index)?;

    // Enter insert mode after launching the task
    let task_settings = &settings!().task;
    if task_settings.insert_mode_on_launch {
        nvim::feedkeys("i", Mode::Normal, false);
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct ActiveTask {
    buffer: Buffer,
    config: TaskConfig,
}

impl ActiveTask {
    pub(crate) fn is_list_empty() -> bool {
        self::state!().active_list.is_empty()
    }

    pub(crate) fn get_lines_for_ui(no_tasks_msg: &str) -> Result<Vec<String>> {
        let lines: Vec<String> = if ActiveTask::is_list_empty() {
            vec!["".into(), no_tasks_msg.into()]
        } else {
            let active_tasks = &self::state!().active_list;
            let mut vec = vec!["".into()];
            vec.extend(active_tasks.iter().map(|task| task.config.name().into()));
            vec
        };

        Ok(lines)
    }

    pub(crate) fn render(index: usize) -> Result<()> {
        let mut state = self::state!();
        let active_task = &state.active_list[index];
        let display_id = active_task.config.disp() as usize;

        if let Some(ref window) = state.windows[display_id] {
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
                        active_task.config.name(),
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
            window.set_var("taskdisplay", display_id)?;

            nvim::exec_autocmds(
                ["User"],
                &ExecAutocmdsOpts::builder()
                    .group("launch_nvim")
                    .patterns("LaunchNvimTaskWindowCreated")
                    .build(),
            )?;

            state.windows[display_id] = Some(window);
        }

        Ok(())
    }

    pub(crate) fn run(index: usize) -> Result<()> {
        let active_task = &mut self::state!().active_list[index];
        nvim_set_local(&active_task.buffer, "modified", false)?;

        let command = active_task.config.command();
        let term_options = active_task.config.term_options();
        nvim::call_function::<_, i32>("termopen", (command, term_options))?;

        Ok(())
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
