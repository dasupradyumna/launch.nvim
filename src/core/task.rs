/*------------------------------------------ TASK RUNNER -----------------------------------------*/

use crate::config::{TaskConfig, TaskDisplay};
use crate::settings::state as settings;
use crate::utils::{float, notify, setup_module_state, Result};
use ::nvim_oxi::api::opts::{BufDeleteOpts, ExecAutocmdsOpts, OptionOpts};
use ::nvim_oxi::api::types::{Mode, SplitDirection, WindowConfig};
use ::nvim_oxi::api::{self as nvim, Buffer, Window};

setup_module_state!(core::task, [pub(crate)]
{
    active_list: Vec<ActiveTask> = Vec::new(),
    windows: [Option<Window>; 3] = [const { None }; 3],
});

pub(crate) fn run(config: TaskConfig) -> Result<()> {
    // Create a new task buffer
    let buffer = nvim::create_buf(false, true)?;
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();
    nvim::set_option_value("filetype", "launch_nvim_task", &opts)?;

    // Open the task window and launch a terminal buffer with current config
    let task = ActiveTask { buffer, config };
    task.render()?;
    task.run()?;

    // Enter insert mode after launching the task
    let task_settings = &settings!().task;
    if task_settings.insert_mode_on_launch {
        nvim::feedkeys("i", Mode::Normal, false);
    }

    self::state!().active_list.push(task);
    Ok(())
}

pub(crate) fn list_active_tasks() {
    let Err(msg) = ActiveTask::open_list() else {
        return;
    };
    notify!(Error: msg);
}

#[derive(Debug)]
struct ActiveTask {
    buffer: Buffer,
    config: TaskConfig,
}

impl ActiveTask {
    fn open_list() -> Result<()> {
        let NO_TASKS_MSG = "-- No active tasks --";

        let state = &mut self::state!();
        let lines: Vec<String> = if state.active_list.is_empty() {
            Vec::from_iter([NO_TASKS_MSG.into()])
        } else {
            state.active_list.iter().map(|c| c.config.name().into()).collect()
        };

        // Create buffer
        let mut buffer = nvim::create_buf(false, true)?;
        let opts = OptionOpts::builder().buffer(buffer.clone()).build();
        nvim::set_option_value("filetype", "launch_nvim_active_task_list", &opts)?;

        // Set buffer contents
        let range = 1..buffer.line_count()?;
        nvim::set_option_value("modifiable", true, &opts)?;
        buffer.set_lines(range, true, lines.iter().map(|s| format!("    {s}    ")))?;
        nvim::set_option_value("modifiable", false, &opts)?;

        // Open a centered floating window
        let n = lines.len() as u32;
        // FIX: handle other navigation keymaps like wW, eE, bB etc.
        buffer.set_var("bounds", ::nvim_oxi::Array::from((2, n + 1)))?;
        let height = n + 2;
        let width = unsafe { lines.iter().map(|l| l.len() + 8).max().unwrap_unchecked() as u32 };
        let mut window = float::centered("Active Tasks", &buffer, width, height)?;
        window.set_cursor(2, 0)?;
        let opts = OptionOpts::builder().win(window.clone()).build();
        nvim::set_option_value("cursorline", !state.active_list.is_empty(), &opts)?;

        // Set buffer keymaps for actions
        use crate::launcher::action;
        use ::nvim_oxi::Array;
        let action_list = {
            let buf = buffer.clone();
            Array::from((Array::from((
                "q",
                action::wrap_cb_once(move |()| {
                    Ok(buf.delete(&BufDeleteOpts::builder().force(true).build())?)
                }),
            )),))
        };
        buffer.set_var("callbacks", action_list)?;
        nvim::command("call b:setup_callbacks()")?;

        Ok(())
    }

    fn render(&self) -> Result<()> {
        let task_windows = &mut self::state!().windows;
        let display_id = self.config.disp() as usize;

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

            let mut window = match self.config.disp() {
                TaskDisplay::Float => {
                    let size = ui_settings.float.size as u32;
                    float::centered(
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

            let opts = OptionOpts::builder().win(window.clone()).build();
            nvim::set_option_value("winfixbuf", true, &opts)?;
            nvim::set_option_value("signcolumn", "yes:1", &opts)?;
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

    fn run(&self) -> Result<i32> {
        // TODO: handle failure here with a default command that displays an error message
        let command = self.config.command();
        let term_options = self.config.term_options();

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
