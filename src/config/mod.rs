/*------------------------------------ RUNTIME CONFIGURATIONS ------------------------------------*/

use crate::utils::serde::StructVisitor;
use ::nvim_oxi::{Dictionary, Object};
use ::serde::{de::EnumAccess, de::Error, de::Visitor, Deserialize};
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

impl<'de> Deserialize<'de> for TaskDisplay {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
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
        D: serde::Deserializer<'de>,
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

#[derive(Debug)]
pub(crate) struct TaskConfigUser {
    name: String,
    command: String,
    args: Option<Vec<String>>,
    display: Option<TaskDisply>,
    cwd: Option<PathBuf>,
    env: Option<HashMap<String, String>>,
    // shell: Option<???>
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

    pub(crate) fn name(&self) -> &String {
        &self.name
    }

    pub(crate) fn display(&self) -> TaskDisplay {
        self.display
    }

    // REMOVE: all unwrap() calls with Result<Object> as return types

    pub(crate) fn command(&self) -> Object {
        let mut ret = String::new();
        ret.push_str(&self.command);
        ret.push(' ');
        ret += self
            .args
            .clone()
            .iter_mut()
            .reduce(|acc, e| {
                acc.push(' ');
                acc.push_str(e);
                acc
            })
            .unwrap();

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
