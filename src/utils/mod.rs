/*----------------------------------------- UTILITY ITEMS ----------------------------------------*/

pub(crate) mod notify;
mod result;
pub(crate) mod serde;

pub(crate) use result::{Error, Result};

use ::nvim_oxi::api::opts::OptionOpts;
use ::nvim_oxi::api::{self as nvim, Buffer, Window};
use ::nvim_oxi::Function;

macro_rules! setup_module_state {

    ( @state_macro $pub:vis $( $path:ident )::+, $state_struct:tt ) => {

        // Static state variable definition along with a macro for convenient access
        $pub static _STATE: std::sync::LazyLock<std::sync::Mutex<$state_struct>> =
            std::sync::LazyLock::new(std::sync::Mutex::default);
        macro_rules! state {
            () => { crate::$( $path ::)+_STATE.lock().unwrap() };
        }
        $pub use state;

    };

    ( $( $path:ident )::+ , [$pub0:vis] {
        $( $pub1:vis $field:ident: $field_type:ty = $field_default:expr ,)+
    } ) => {

        // Definition of `State` struct with its default initializer
        #[derive(Debug)]
        $pub0 struct _State {
            $( $pub1 $field: $field_type ,)+
        }

        impl Default for _State {
            fn default() -> Self {
                Self { $( $field: $field_default ,)+ }
            }
        }

        crate::utils::setup_module_state!( @state_macro $pub0 $( $path )::+, _State );

    };

    ( $( $path:ident )::+ , [$pub:vis] $struct:ty ) => {

        crate::utils::setup_module_state!( @state_macro $pub $( $path )::+, $struct );

    };
}
pub(crate) use setup_module_state;

pub(crate) fn get_float_position(width: u32, height: u32) -> self::Result<(u32, u32)> {
    // Compute top-left row and column for a centered floating window
    let screen_height: u32 = nvim::get_option_value("lines", &OptionOpts::default())?;
    let screen_width: u32 = nvim::get_option_value("columns", &OptionOpts::default())?;
    let row = (screen_height - height) / 2 - 2;
    let col = (screen_width - width) / 2 - 2;

    Ok((row, col))
}

pub(crate) fn open_float(
    title: &str,
    buffer: &Buffer,
    width: u32,
    height: u32,
) -> self::Result<Window> {
    use ::nvim_oxi::api::types::*;

    let (row, col) = self::get_float_position(width, height)?;
    let win_config = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .row(row)
        .col(col)
        .width(width)
        .height(height)
        .title(WindowTitle::SimpleString(format!(" {title} ").into()))
        .title_pos(WindowTitlePosition::Center)
        .footer(WindowTitle::SimpleString(" launch.nvim ".into()))
        .footer_pos(WindowTitlePosition::Right)
        .border(WindowBorder::Rounded)
        .style(WindowStyle::Minimal)
        .zindex(49)
        .build();
    let window = nvim::open_win(buffer, true, &win_config)?;

    // Fix the target buffer
    let opts = OptionOpts::builder().win(window.clone()).build();
    nvim::set_option_value("winfixbuf", true, &opts)?;

    Ok(window)
}

pub(crate) fn open_popup(
    prompt: &str,
    default: &str,
    row: u32,
    col: u32,
    callback: Function<::nvim_oxi::String, ()>,
) -> self::Result<Window> {
    let mut buffer = nvim::create_buf(false, true)?;
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();
    nvim::set_option_value("filetype", "launch_nvim_popup_prompt", &opts)?;

    use ::nvim_oxi::api::types::*;
    let (width, height) = (40, 1);
    let win_config = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .row(row)
        .col(col)
        .width(width)
        .height(height)
        .border(WindowBorder::Rounded)
        .style(WindowStyle::Minimal)
        .zindex(49)
        .build();
    let window = nvim::open_win(&buffer, true, &win_config)?;

    // Fix the prompt buffer
    let opts = OptionOpts::builder().win(window.clone()).build();
    nvim::set_option_value("winfixbuf", true, &opts)?;

    // Update the prompt text
    buffer.set_var("prompt", prompt)?;
    buffer.set_var("default", default)?;
    buffer.set_var("callback", callback)?;
    nvim::command("call b:update_prompt()")?;

    Ok(window)
}
