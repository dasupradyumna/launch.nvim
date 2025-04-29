/*-------------------------------------- TASK CONFIGURATION --------------------------------------*/
//!
//! This module contains all structs, their methods, and the trait implementations related to
//! specifying a task configuration.
//!
//! TODO: refactor TaskDisplay and TaskDisplayFloatSize into setup_deserializable_structs! macro
//! or create a separate setup_deserializable_enum! macro, and make the struct macro modular
//! Is there some way to make all of the below TaskDisplay logic more compact?

use crate::settings::state as settings;
use crate::utils::serde::StructVisitor;
use ::nvim_oxi::{Dictionary, Object};
use ::serde::de::{EnumAccess, Error, Visitor};
use ::serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::path::PathBuf;

/// Enumerates possible display modes of a task window
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) enum TaskDisplay {
    Float,
    VSplit,
    HSplit,
}

/// Converts a string to a TaskDisplay
impl From<&str> for TaskDisplay {
    fn from(value: &str) -> Self {
        match value {
            "float" => Self::Float,
            "hsplit" => Self::HSplit,
            "vsplit" => Self::VSplit,
            _ => Self::Float,
        }
    }
}

/// Converts a TaskDisplay to a string
impl std::fmt::Display for TaskDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self {
            Self::Float => "float",
            Self::HSplit => "hsplit",
            Self::VSplit => "vsplit",
        };
        write!(f, "{}", fmt)
    }
}

impl Serialize for TaskDisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Float => serializer.serialize_str("float"),
            Self::HSplit => serializer.serialize_str("hsplit"),
            Self::VSplit => serializer.serialize_str("vsplit"),
        }
    }
}

impl<'de> Deserialize<'de> for TaskDisplay {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_enum("", &[], StructVisitor::<TaskDisplay>::new())
    }
}

impl<'de> Visitor<'de> for StructVisitor<TaskDisplay> {
    type Value = TaskDisplay;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a `TaskDisplay` enum string.")
    }

    fn visit_enum<E>(self, data: E) -> Result<Self::Value, E::Error>
    where
        E: EnumAccess<'de>,
    {
        let (variant, _) = data.variant::<String>()?;
        match variant.as_str() {
            "float" => Ok(Self::Value::Float),
            "hsplit" => Ok(Self::Value::HSplit),
            "vsplit" => Ok(Self::Value::VSplit),
            _ => Err(E::Error::unknown_variant(&variant, &["float", "hsplit", "vsplit"])),
        }
    }
}

/// Enumerates possible sizes of a floating task window
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub(crate) enum TaskDisplayFloatSize {
    Small = 45,
    Medium = 65,
    Large = 85,
}

impl<'de> Deserialize<'de> for TaskDisplayFloatSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_enum("", &[], StructVisitor::<TaskDisplayFloatSize>::new())
    }
}

impl<'de> Visitor<'de> for StructVisitor<TaskDisplayFloatSize> {
    type Value = TaskDisplayFloatSize;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a `TaskDisplayFloatSize` enum string.")
    }

    fn visit_enum<E>(self, data: E) -> Result<Self::Value, E::Error>
    where
        E: EnumAccess<'de>,
    {
        let (variant, _) = data.variant::<String>()?;
        match variant.as_str() {
            "small" => Ok(Self::Value::Small),
            "medium" => Ok(Self::Value::Medium),
            "large" => Ok(Self::Value::Large),
            _ => Err(E::Error::unknown_variant(&variant, &["small", "medium", "large"])),
        }
    }
}

/// JSON representation of a task config
///
/// - This struct is used for serialization and deserialization of task configs to/from a JSON file
/// - Represents optional fields using `Option<T>`, and skips their serialization if they are `None`
/// - Mutated by the launcher UI, allowing the user to conveniently edit the task config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TaskConfigJson {
    name: String,
    cmd: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disp: Option<TaskDisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cwd: Option<PathBuf>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    env: HashMap<String, String>,
    // shell: Option<???>
}

/// Default implementation for `TaskConfigJson`
impl Default for TaskConfigJson {
    fn default() -> Self {
        Self {
            name: "New Config".into(),
            cmd: "echo 'hello'".into(),
            args: vec![],
            disp: None,
            cwd: None,
            env: HashMap::new(),
        }
    }
}

impl TaskConfigJson {
    /*------------------------ GETTERS -----------------------*/

    pub(crate) fn name(&self) -> &String {
        &self.name
    }
    pub(crate) fn cmd(&self) -> &String {
        &self.cmd
    }
    pub(crate) fn args(&self) -> &Vec<String> {
        &self.args
    }
    pub(crate) fn disp(&self) -> &Option<TaskDisplay> {
        &self.disp
    }
    pub(crate) fn cwd(&self) -> &Option<PathBuf> {
        &self.cwd
    }
    pub(crate) fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    /*------------------------ SETTERS -----------------------*/

    /// Set the name of the task
    pub(crate) fn set_name(&mut self, name: String) {
        self.name = name;
    }
    /// Set the command of the task
    pub(crate) fn set_cmd(&mut self, cmd: String) {
        self.cmd = cmd;
    }
    /// Set the specified argument in the task arguments
    pub(crate) fn set_arg(&mut self, index: usize, new_arg: String) {
        match self.args.get_mut(index) {
            Some(arg) => *arg = new_arg,
            None => self.args.push(new_arg),
        }
    }
    /// Insert an argument at the specified index in the task arguments
    pub(crate) fn insert_arg(&mut self, index: usize, arg: String) {
        self.args.insert(index, arg);
    }
    /// Set the display mode of the task
    pub(crate) fn set_disp(&mut self, disp: TaskDisplay) {
        self.disp.replace(disp);
    }
    /// Set the working directory of the task
    pub(crate) fn set_cwd(&mut self, cwd: String) {
        self.cwd.replace(cwd.into());
    }
    /// Replace the older environment variable with the new one
    ///
    /// - `old_var`: name of the environment variable to replace
    /// - `new_var_value`: name-value pair of the new environment variable
    pub(crate) fn set_env(&mut self, old_var: &str, new_var_value: String) {
        self.env.remove(old_var);
        let (new_var, value) = unsafe { new_var_value.split_once('=').unwrap_unchecked() };
        self.env.insert(new_var.into(), value.into());
    }

    /*----------------------- DELETERS -----------------------*/

    /// Delete the argument at the specified index
    pub(crate) fn del_arg(&mut self, index: usize) {
        if index < self.args.len() {
            self.args.remove(index);
        }
    }
    /// Remove the display mode of the task
    pub(crate) fn del_disp(&mut self) {
        self.disp = None;
    }
    /// Remove the working directory of the task
    pub(crate) fn del_cwd(&mut self) {
        self.cwd = None;
    }
    /// Remove the specified environment variable
    pub(crate) fn del_env(&mut self, var: &str) {
        self.env.remove(var);
    }
}

/// Conversion from `TaskConfigJson` to `TaskConfig` ; consumes `TaskConfigJson`
impl TryFrom<TaskConfigJson> for TaskConfig {
    type Error = crate::utils::Error;

    fn try_from(config: TaskConfigJson) -> crate::utils::Result<Self> {
        let task_settings = &settings!().task;
        let cwd = match config.cwd.map(|path| path.canonicalize()) {
            Some(path) => path?,
            None => std::env::current_dir()?,
        };
        Ok(TaskConfig {
            name: config.name,
            cmd: config.cmd,
            args: config.args,
            disp: config.disp.unwrap_or_else(|| task_settings.ui.display.clone()),
            cwd,
            env: config.env,
        })
    }
}

/// Fully specified configuration of a task
///
/// This is created from `TaskConfigJson` replacing optional fields with their default values, and
/// is used to launch the task
#[derive(Debug, Clone)]
pub(crate) struct TaskConfig {
    name: String,
    cmd: String,
    args: Vec<String>,
    disp: TaskDisplay,
    cwd: PathBuf,
    env: HashMap<String, String>,
    // shell: ???
}

impl TaskConfig {
    pub(crate) fn name(&self) -> &String {
        &self.name
    }

    pub(crate) fn disp(&self) -> &TaskDisplay {
        &self.disp
    }

    /// Constructs the command string to launch the task, and returns it as an Object
    pub(crate) fn command(&self) -> Object {
        let mut ret = String::new();
        ret.push_str(&self.cmd);
        ret.push(' ');
        ret += &self.args.join(" ");

        ret.into()
    }

    /// Constructs the terminal options to launch the task, and returns it as an Object
    pub(crate) fn term_options(&self) -> Object {
        let env = Dictionary::from_iter(self.env.iter().map(|(k, v)| (k.as_str(), v.as_str())));
        let ret = Dictionary::from_iter::<[(_, Object); 3]>([
            ("clear_env", false.into()),
            ("cwd", self.cwd.to_str().into()),
            ("env", env.into()),
        ]);

        ret.into()
    }
}
