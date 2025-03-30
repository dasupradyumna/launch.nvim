/*----------------------------------- SCRATCH BUFFER UTILITIES -----------------------------------*/

use super::result::Result;
use ::nvim_oxi::api::{self as nvim, opts::OptionOpts, Buffer};

impl super::ScopeOpts for Buffer {
    fn opts(&self) -> OptionOpts {
        OptionOpts::builder().buffer(self.clone()).build()
    }
}

pub(crate) fn create_scratch(filetype: &str) -> Result<Buffer> {
    let buffer = nvim::create_buf(false, true)?;
    super::nvim_set_local(&buffer, "modifiable", false)?;
    super::nvim_set_local(&buffer, "filetype", format!("launch_nvim_{filetype}"))?;

    Ok(buffer)
}

pub(crate) fn write_lines<Line>(buffer: &mut Buffer, lines: &[Line]) -> Result<()>
where
    Line: std::fmt::Display,
{
    super::nvim_set_local(buffer, "modifiable", true)?;
    buffer.set_lines(.., true, lines.iter().map(|s| format!("    {s}    ")))?;
    super::nvim_set_local(buffer, "modifiable", false)?;

    Ok(())
}
