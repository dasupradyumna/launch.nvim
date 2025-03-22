/*----------------------------------- SCRATCH BUFFER UTILITIES -----------------------------------*/

pub(crate) use super::result::Result;
pub(crate) use ::nvim_oxi::api::{self as nvim, opts::OptionOpts, Buffer};

pub(crate) fn create_scratch(filetype: &str) -> Result<Buffer> {
    let buffer = nvim::create_buf(false, true)?;
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();
    nvim::set_option_value("modifiable", false, &opts)?;
    nvim::set_option_value("filetype", format!("launch_nvim_{filetype}"), &opts)?;

    Ok(buffer)
}

pub(crate) fn write_lines<Line>(buffer: &mut Buffer, lines: &[Line]) -> Result<()>
where
    Line: std::fmt::Display,
{
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();
    nvim::set_option_value("modifiable", true, &opts)?;
    buffer.set_lines(.., true, lines.iter().map(|s| format!("    {s}    ")))?;
    nvim::set_option_value("modifiable", false, &opts)?;

    Ok(())
}
