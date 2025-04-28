/*------------------------------------- CONFIGURATIONS BUFFER ------------------------------------*/
//! This module contains all the functions related to managing the runtime configurations buffer.
//! This includes creating and deleting the buffer, serializing and deserializing the runtime
//! configurations to and from the buffer, and handling undo-redo operations in the buffer.

use crate::utils::{nvim_set_local, Result};
use ::nvim_oxi::api::{self as nvim, Buffer};
use ::nvim_oxi::Dictionary;
use ::serde_json::{self as json, json, Value};

fn execute_command<Cmd: std::fmt::Display>(command: Cmd) -> Result<()> {
    let buffer = &super::state!().buffer;
    let command = format!("silent {command}");
    Ok(buffer.call(move |_| -> Result<()> { Ok(nvim::command(&command)?) })?)
}

/*----------------------------- CREATION-DELETION ----------------------------*/

pub(crate) fn create() -> Result<()> {
    let mut config = super::state!();
    let mut buffer = nvim::create_buf(false, false)?;
    nvim_set_local(&buffer, "swapfile", false)?;
    buffer.set_name(&config.filepath)?;
    config.buffer = buffer;
    Ok(())
}

pub(crate) fn delete() -> Result<()> {
    let config = &mut super::state!();
    if config.tasks.is_empty() && config.filepath.is_file() {
        std::fs::remove_file(&config.filepath)?;
    }
    let buffer = std::mem::replace(&mut config.buffer, Buffer::from(0));
    Ok(buffer.delete(&nvim::opts::BufDeleteOpts::builder().force(true).build())?)
}

/*----------------------- SERIALIZATION-DESERIALIZATION ----------------------*/

fn serialize() -> Result<String> {
    let config = super::state!();
    let contents = json!({
        "version": config.version,
        "tasks": config.tasks,
    });

    Ok(json::to_string_pretty(&contents)?)
}

fn deserialize(contents: &str) -> Result<()> {
    if !contents.trim_ascii_end().is_empty() {
        let mut config = super::state!();
        let contents: Value = json::from_str(contents)?;
        config.version = json::from_value(contents["version"].clone())?;
        config.tasks = json::from_value(contents["tasks"].clone())?;
    }
    Ok(())
}

pub(crate) fn serialize_to_string() -> Result<()> {
    let contents = self::serialize()?;
    super::state!().buffer.set_lines(.., true, contents.split('\n'))?;
    // NOTE: closes undo block to make `nvim_buf_set_lines()` changes undoable
    self::execute_command("let &l:undolevels = &l:undolevels")
}

fn deserialize_from_string() -> Result<()> {
    let contents = {
        let lines = super::state!().buffer.get_lines(.., true)?;
        lines.fold(String::new(), |acc, line| acc + &line.to_string() + "\n")
    };
    self::deserialize(&contents)
}

pub(crate) fn read_from_file() -> Result<()> {
    self::execute_command("edit! | set nobuflisted")?;
    self::deserialize_from_string()
}

pub(crate) fn write_to_file() -> Result<()> {
    self::execute_command("write")
}

/*--------------------------------- UNDO-REDO --------------------------------*/

fn get_undotree() -> Result<Dictionary> {
    let buffer = &super::state!().buffer;
    Ok(nvim::call_function("undotree", (buffer.handle(),))?)
}

pub(crate) fn redo() -> Result<bool> {
    let undotree = self::get_undotree()?;
    if undotree["seq_cur"] == undotree["seq_last"] {
        return Ok(false);
    }

    self::execute_command("redo")?;
    self::deserialize_from_string()?;
    Ok(true)
}

pub(crate) fn undo() -> Result<bool> {
    let undotree = self::get_undotree()?;
    if unsafe { undotree["seq_cur"].as_integer_unchecked() } == 0 {
        return Ok(false);
    }

    self::execute_command("undo")?;
    self::deserialize_from_string()?;
    Ok(true)
}

pub(crate) fn clear_undo_history() -> Result<()> {
    self::execute_command("call launch#clear_undo_history()")
}
