/*--------------------------------------- CONFIG JSON SERDE --------------------------------------*/

use crate::utils::Result;
use ::serde_json::{self as json, json, Value};

pub(super) fn serialize() -> Result<String> {
    let config = super::state!();
    let contents = json!({
        "version": config.version,
        "tasks": config.tasks,
    });

    Ok(json::to_string_pretty(&contents)?)
}

pub(super) fn deserialize(contents: &str) -> Result<()> {
    if !contents.trim_ascii_end().is_empty() {
        let mut config = super::state!();
        let contents: Value = json::from_str(contents)?;
        config.version = json::from_value(contents["version"].clone())?;
        config.tasks = json::from_value(contents["tasks"].clone())?;
    }
    Ok(())
}
