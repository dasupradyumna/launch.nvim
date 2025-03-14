/*-------------------------------------- TASK CONFIGURATION --------------------------------------*/

use crate::settings::state as settings;
use crate::utils::serde::StructVisitor;
use ::nvim_oxi::{Dictionary, Object};
use ::serde::de::{EnumAccess, Error, Visitor};
use ::serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::path::PathBuf;

// TODO: refactor TaskDisplay and TaskDisplayFloatSize into setup_deserializable_structs! macro
// or create a separate setup_deserializable_enum! macro, and make the struct macro modular

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub(crate) enum TaskDisplay {
    Float,
    VSplit,
    HSplit,
}

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

// REMOVE: all unwrap() calls with Result<...> as return types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TaskConfigJson {
    name: String,
    command: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display: Option<TaskDisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cwd: Option<PathBuf>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    env: HashMap<String, String>,
    // shell: Option<???>
}

impl TaskConfigJson {
    pub(crate) fn name(&self) -> &String {
        &self.name
    }
    pub(crate) fn command(&self) -> &String {
        &self.command
    }
    pub(crate) fn args(&self) -> &Vec<String> {
        &self.args
    }
    pub(crate) fn display(&self) -> &Option<TaskDisplay> {
        &self.display
    }
    pub(crate) fn cwd(&self) -> &Option<PathBuf> {
        &self.cwd
    }
    pub(crate) fn env(&self) -> &HashMap<String, String> {
        &self.env
    }
    pub(crate) fn set_name(&mut self, name: String) {
        self.name = if !name.is_empty() { name } else { return };
    }
    pub(crate) fn set_command(&mut self, command: String) {
        self.command = if !command.is_empty() { command } else { return };
    }
    pub(crate) fn set_arg(&mut self, index: usize, arg: String) {
        if index < self.args.len() {
            if arg.is_empty() {
                self.args.remove(index);
            } else {
                self.args[index] = arg;
            }
        } else if !arg.is_empty() {
            self.args.push(arg);
        }
    }
    pub(crate) fn set_cwd(&mut self, cwd: String) {
        self.cwd = if cwd.is_empty() { None } else { Some(PathBuf::from(cwd)) };
    }
}

impl From<TaskConfigJson> for TaskConfig {
    fn from(value: TaskConfigJson) -> Self {
        let task_settings = &settings!().task;
        TaskConfig {
            name: value.name,
            command: value.command,
            args: value.args,
            display: value.display.unwrap_or(task_settings.ui.display),
            cwd: value
                .cwd
                .map(|path| path.canonicalize().unwrap())
                .unwrap_or_else(|| std::env::current_dir().unwrap()),
            env: value.env,
        }
    }
}

#[derive(Debug)]
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
    pub(crate) fn name(&self) -> &String {
        &self.name
    }

    pub(crate) fn display(&self) -> TaskDisplay {
        self.display
    }

    pub(crate) fn command(&self) -> Object {
        let mut ret = String::new();
        ret.push_str(&self.command);
        ret.push(' ');
        ret += &self.args.join(" ");

        ret.into()
    }

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
