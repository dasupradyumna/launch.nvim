/*------------------------------------ RUNTIME CONFIGURATIONS ------------------------------------*/

use ::nvim_oxi::api::StringOrInt;
use ::nvim_oxi::conversion::ToObject;
use ::nvim_oxi::{Dictionary, Object};
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

    // REMOVE: all unwrap() calls and handle them better

    pub(crate) fn command(&self) -> Object {
        let mut ret = String::new();
        ret.push_str(&self.command);
        ret.push(' ');
        ret += &self
            .args
            .clone()
            .iter_mut()
            .reduce(|acc, e| {
                acc.push(' ');
                acc.push_str(&e);
                acc
            })
            .unwrap();

        StringOrInt::to_object(ret)
    }

    pub(crate) fn term_options(&self) -> Object {
        let env = Dictionary::from_iter(self.env.iter().map(|(k, v)| (k.as_str(), v.as_str())));
        Dictionary::from_iter([
            ("clear_env", Object::from(false)),
            ("cwd", Object::from(self.cwd.to_str())),
            ("env", env.to_object().unwrap()),
        ])
        .to_object()
        .unwrap()
    }
}
