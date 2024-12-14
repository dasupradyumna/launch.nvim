/*------------------------------------ RUNTIME CONFIGURATIONS ------------------------------------*/

use std::collections::HashMap;
use std::path::PathBuf;

pub(crate) enum TaskDisplay {
    Float,
    VSplit,
    HSplit,
}

pub(crate) struct TaskConfig {
    name: String,
    command: String,
    args: Vec<String>,
    display: TaskDisplay,
    cwd: PathBuf,
    env: HashMap<String, String>,
    // shell: ???
}

impl TaskConfig {
    pub(crate) fn new(
        name: &'static str,
        command: &'static str,
        args: &'static [&'static str],
        display: TaskDisplay,
        cwd: &'static str,
        env: HashMap<String, String>,
    ) -> Self {
        Self {
            name: name.to_string(),
            command: command.to_string(),
            args: args.iter().map(|e| e.to_string()).collect(),
            display,
            cwd: PathBuf::from(cwd),
            env,
        }
    }
}
