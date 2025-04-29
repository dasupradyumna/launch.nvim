/*----------------------------------------- UTILITY ITEMS ----------------------------------------*/
//!
//! This module contains common utility functions and structs used by the plugin
//!
//! ## Submodules
//! - [`buffer`] - contains items related to the plugin buffer management
//! - [`float`] - contains items related to the floating window management
//! - [`result`] - contains items related to the result type used by the plugin
//! - [`serde`] - contains items related to the deserialization helpers

pub(crate) mod buffer;
pub(crate) mod float;
mod result;
pub(crate) mod serde;

pub(crate) use result::{Error, IndexChecked, Result};

macro_rules! setup_module_state {

    ( @state_macro $pub:vis $( $path:ident )::+, $state_struct:path ) => {

        // Static state variable definition along with a macro for convenient access
        $pub static _STATE: std::sync::LazyLock<std::sync::Mutex<$state_struct>> =
            std::sync::LazyLock::new(std::sync::Mutex::default);
        /// Module state accessor macro
        macro_rules! state {
            () => { crate::$( $path ::)+_STATE.lock().unwrap() };
        }
        $pub use state;

    };

    ( $( $path:ident )::+ , $([$pub0:vis])? {
        $( $pub1:vis $field:ident: $field_type:ty = $field_default:expr ,)+
    } ) => {

        // Definition of `_State` struct with its Default implementation
        #[derive(Debug)]
        $($pub0)? struct _State {
            $( $pub1 $field: $field_type ,)+
        }
        impl Default for _State {
            fn default() -> Self {
                Self { $( $field: $field_default ,)+ }
            }
        }

        crate::utils::setup_module_state!( @state_macro $($pub0)? $( $path )::+, _State );

    };

    ( $( $path:ident )::+ , $([$pub:vis])? $struct:path ) => {

        crate::utils::setup_module_state!( @state_macro $($pub)? $( $path )::+, $struct );

    };
}
pub(crate) use setup_module_state;

/// Helper function to send notifications to Neovim
pub(crate) fn _send<M: std::fmt::Display>(level: ::nvim_oxi::api::types::LogLevel, msg: M) {
    use ::nvim_oxi::api::notify;
    let lvl = format!("{level:?}").to_ascii_uppercase();
    _ = notify(&format!("[launch.nvim] {lvl}: {msg}"), level, &::nvim_oxi::Dictionary::new());
}
/// Send a notification of specified level to Neovim
macro_rules! notify(
    ($level:ident: $msg:expr) => {
        crate::utils::_send(::nvim_oxi::api::types::LogLevel::$level, $msg)
    };
);
pub(crate) use notify;

/// Helper trait for setting Neovim options using nvim-oxi
pub(crate) trait ScopeOpts {
    fn opts(&self) -> ::nvim_oxi::api::opts::OptionOpts;
}

/// Set Neovim option in the local scope - can be buffer or window
pub(crate) fn nvim_set_local<Scope, Value>(scope: &Scope, name: &str, value: Value) -> Result<()>
where
    Value: ::nvim_oxi::conversion::ToObject,
    Scope: ScopeOpts,
{
    Ok(::nvim_oxi::api::set_option_value(name, value, &scope.opts())?)
}
