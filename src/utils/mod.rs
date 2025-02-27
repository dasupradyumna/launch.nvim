/*----------------------------------------- UTILITY ITEMS ----------------------------------------*/

pub(crate) mod notify;
mod result;
pub(crate) mod serde;

pub(crate) use result::Result;

use ::nvim_oxi::api::opts::OptionOpts;
use ::nvim_oxi::api::{self as nvim, Buffer, Window};

macro_rules! setup_module_state {

    ( @state_macro $( $path:ident )::+, $state_struct:tt ) => {

        // Static state variable definition along with a macro for convenient access
        pub(crate) static _STATE: std::sync::LazyLock<std::sync::Mutex<$state_struct>> =
            std::sync::LazyLock::new(std::sync::Mutex::default);
        macro_rules! state {
            () => { crate::$( $path ::)+_STATE.lock().unwrap() };
        }
        pub(crate) use state;

    };

    ( $( $path:ident )::+ , {
        $( $pub:vis $field:ident: $field_type:ty = $field_default:expr ,)+
    } ) => {

        // Definition of `State` struct with its default initializer
        #[derive(Debug)]
        pub(crate) struct _State {
            $( $pub $field: $field_type ,)+
        }

        impl Default for _State {
            fn default() -> Self {
                Self { $( $field: $field_default ,)+ }
            }
        }

        crate::utils::setup_module_state!( @state_macro $( $path )::+, _State );

    };

    ( $( $path:ident )::+ , $struct:ty ) => {

        crate::utils::setup_module_state!( @state_macro $( $path )::+, $struct );

    };
}
pub(crate) use setup_module_state;

pub(crate) fn open_float(
    title: &str,
    buffer: &Buffer,
    width: u32,
    height: u32,
) -> ::nvim_oxi::Result<Window> {
    use ::nvim_oxi::api::types::*;

    // Compute top-left row and column for a centered floating window
    let screen_height: u32 = nvim::get_option_value("lines", &OptionOpts::default())?;
    let screen_width: u32 = nvim::get_option_value("columns", &OptionOpts::default())?;
    let row = (screen_height - height) / 2 - 2;
    let col = (screen_width - width) / 2 - 2;

    let mut config_builder = WindowConfig::builder();
    let win_config = config_builder
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
