/*---------------------------------------- PLUGIN SETTINGS ---------------------------------------*/

use ::nvim_oxi::conversion::FromObject;
use ::nvim_oxi::print as notify;
use ::nvim_oxi::{Dictionary, Object};

use ::serde::de::value::MapDeserializer;
use ::serde::{de::Error, de::MapAccess, de::Visitor};
use ::serde::{Deserialize, Deserializer};

const WIKI_URL: &str = "https://github.com/dasupradyumna/launch.nvim/wiki/Plugin-Settings";

#[derive(Debug)]
struct SettingsTask {
    insert_mode_on_launch: bool,
}

impl SettingsTask {
    const fn new() -> Self {
        Self { insert_mode_on_launch: false }
    }
}

#[derive(Debug)]
pub(crate) struct Settings {
    confirm_choice: bool,
    task: SettingsTask,
}

impl Settings {
    pub(crate) const fn new() -> Self {
        Self {
            confirm_choice: false,
            task: SettingsTask::new(),
        }
    }

    pub(crate) fn apply(&mut self, user_settings: Object) {
        // TODO: improve error messages when settings cannot be loaded

        let Ok(user_settings) = Dictionary::from_object(user_settings) else {
            notify!("User settings must be a `Settings` table!");
            notify!("Refer to {WIKI_URL} for documentation.\nUsing default settings...");
            return;
        };

        let des = MapDeserializer::new(user_settings.into_iter().map(|(k, v)| (k.to_string(), v)));
        match Settings::deserialize(des) {
            Ok(value) => *self = value,
            Err(err) => {
                notify!("Deserialization failed! {err}");
                notify!("Refer to {WIKI_URL} for documentation.\nUsing default settings...");
            },
        };
    }
}

impl<'de> Deserialize<'de> for Settings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SettingsVisitor)
    }
}

struct SettingsVisitor;

impl<'de> Visitor<'de> for SettingsVisitor {
    type Value = Settings;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(format!("a `Settings` table. Refer to {WIKI_URL}").as_str())
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut confirm_choice = None;
        let mut task = None;

        while let Some(key) = map.next_key::<std::string::String>()? {
            match key.as_str() {
                "confirm_choice" => confirm_choice = Some(map.next_value()?),
                "task" => task = Some(map.next_value()?),
                _ => return Err(Error::unknown_field(&key, &["confirm_choice", "task"])),
            }
        }

        Ok(Settings {
            confirm_choice: confirm_choice.unwrap_or(false),
            task: task.unwrap_or(SettingsTask::new()),
        })
    }
}

impl<'de> Deserialize<'de> for SettingsTask {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SettingsTaskVisitor)
    }
}

struct SettingsTaskVisitor;

impl<'de> Visitor<'de> for SettingsTaskVisitor {
    type Value = SettingsTask;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(format!("a `TaskSettings` table. Refer to {WIKI_URL}").as_str())
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut insert_mode_on_launch = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "insert_mode_on_launch" => insert_mode_on_launch = Some(map.next_value()?),
                _ => return Err(Error::unknown_field(&key, &["insert_mode_on_launch"])),
            }
        }

        Ok(SettingsTask {
            insert_mode_on_launch: insert_mode_on_launch.unwrap_or(false),
        })
    }
}
