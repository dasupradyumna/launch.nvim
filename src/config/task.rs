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
// - is there some way to make all of the below TaskDisplay logic more compact?

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) enum TaskDisplay {
    Float,
    VSplit,
    HSplit,
}

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
    pub(crate) fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub(crate) fn set_cmd(&mut self, cmd: String) {
        self.cmd = cmd;
    }
    pub(crate) fn set_arg(&mut self, index: usize, arg: String) {
        if index < self.args.len() {
            self.args[index] = arg;
        } else {
            self.args.push(arg);
        }
    }
    pub(crate) fn set_disp(&mut self, disp: TaskDisplay) {
        self.disp.replace(disp);
    }
    pub(crate) fn set_cwd(&mut self, cwd: String) {
        self.cwd.replace(cwd.into());
    }
    pub(crate) fn set_env(&mut self, old_var: &str, new_var_value: String) {
        self.env.remove(old_var);
        let (new_var, value) = unsafe { new_var_value.split_once('=').unwrap_unchecked() };
        self.env.insert(new_var.into(), value.into());
    }
    pub(crate) fn del_arg(&mut self, index: usize) {
        if index < self.args.len() {
            self.args.remove(index);
        }
    }
    pub(crate) fn del_disp(&mut self) {
        self.disp = None;
    }
    pub(crate) fn del_cwd(&mut self) {
        self.cwd = None;
    }
    pub(crate) fn del_env(&mut self, var: &str) {
        self.env.remove(var);
    }
    pub(crate) fn insert_arg(&mut self, index: usize, arg: String) {
        self.args.insert(index, arg);
    }
}

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

    pub(crate) fn command(&self) -> Object {
        let mut ret = String::new();
        ret.push_str(&self.cmd);
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
