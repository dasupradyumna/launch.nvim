/*----------------------------------- SCRATCH BUFFER UTILITIES -----------------------------------*/
//!
//! This module contains utility functions related to scratch buffer managemenet

use super::result::Result;
use ::nvim_oxi::api::{self as nvim, opts::OptionOpts, Buffer};

impl super::ScopeOpts for Buffer {
    fn opts(&self) -> OptionOpts {
        OptionOpts::builder().buffer(self.clone()).build()
    }
}

/// Create a non-modifiable scratch buffer with the specified filetype
pub(crate) fn create_scratch(filetype: &str) -> Result<Buffer> {
    let buffer = nvim::create_buf(false, true)?;
    super::nvim_set_local(&buffer, "modifiable", false)?;
    super::nvim_set_local(&buffer, "filetype", format!("launch_nvim_{filetype}"))?;

    Ok(buffer)
}

/// Write lines (padded with spaces) to the specified buffer
pub(crate) fn write_lines<Line>(buffer: &mut Buffer, lines: &[Line]) -> Result<()>
where
    Line: std::fmt::Display,
{
    super::nvim_set_local(buffer, "modifiable", true)?;
    buffer.set_lines(.., true, lines.iter().map(|s| format!("    {s}    ")))?;
    super::nvim_set_local(buffer, "modifiable", false)?;

    Ok(())
}
