/*------------------------------------------ TASK RUNNER -----------------------------------------*/

use crate::config::TaskConfig;
use crate::core::plugin::api;
use crate::utils::notify;
use ::nvim_oxi::api;
use ::nvim_oxi::api::opts::{CreateAutocmdOptsBuilder, OptionOptsBuilder};
use ::nvim_oxi::Array;
use ::nvim_oxi::Result;

pub(crate) fn run(config: TaskConfig) -> Result<()> {
    // create a new task buffer
    let buffer = api::create_buf(false, true)?;
    let opts = OptionOptsBuilder::default().buffer(buffer.clone()).build();
    api::set_option_value("filetype", "launch_nvim_task", &opts)?;
    let opts = CreateAutocmdOptsBuilder::default()
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
    api::command("wincmd n")?;

    // launch the task in a terminal buffer
    api::call_function("termopen", Array::from_iter([config.command()]))?;

    // enter insert mode after launching the task
    if api!().settings.task.insert_mode_on_launch {
        api::command("startinsert")?;
        api::feedkeys("i", api::types::Mode::Normal, false);
    }

    Ok(())
}
