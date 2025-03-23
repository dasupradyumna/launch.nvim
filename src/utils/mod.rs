/*----------------------------------------- UTILITY ITEMS ----------------------------------------*/

pub(crate) mod buffer;
pub(crate) mod float;
mod result;
pub(crate) mod serde;

pub(crate) use result::{Error, Result};

macro_rules! setup_module_state {

    ( @state_macro $pub:vis $( $path:ident )::+, $state_struct:path ) => {

        // Static state variable definition along with a macro for convenient access
        $pub static _STATE: std::sync::LazyLock<std::sync::Mutex<$state_struct>> =
            std::sync::LazyLock::new(std::sync::Mutex::default);
        macro_rules! state {
            () => { crate::$( $path ::)+_STATE.lock().unwrap() };
        }
        $pub use state;

    };

    ( $( $path:ident )::+ , $([$pub0:vis])? {
        $( $pub1:vis $field:ident: $field_type:ty = $field_default:expr ,)+
    } ) => {

        // Definition of `State` struct with its default initializer
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

pub(crate) fn _send<M: std::fmt::Display>(level: ::nvim_oxi::api::types::LogLevel, msg: M) {
    use ::nvim_oxi::api::notify;
    let lvl = format!("{level:?}").to_ascii_uppercase();
    _ = notify(&format!("[launch.nvim] {lvl}: {msg}"), level, &::nvim_oxi::Dictionary::new());
}
macro_rules! notify(
    ($level:ident: $msg:expr) => {
        crate::utils::_send(::nvim_oxi::api::types::LogLevel::$level, $msg)
    };
);
pub(crate) use notify;
