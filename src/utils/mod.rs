/*----------------------------------------- UTILITY ITEMS ----------------------------------------*/

pub(crate) mod notify;
pub(crate) mod serde;

macro_rules! setup_module_state {
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

        // Static state variable definition along with a macro for convenient access
        pub(crate) static _STATE: std::sync::LazyLock<std::sync::Mutex<_State>> =
            std::sync::LazyLock::new(std::sync::Mutex::default);
        macro_rules! state {
            () => { crate::$( $path ::)+_STATE.lock().unwrap() };
        }
        pub(crate) use state;

    };
}
pub(crate) use setup_module_state;

pub(crate) fn get_float_position_size(size: u32, lines: u32, columns: u32) -> [u32; 4] {
    let width = columns * size / 100;
    let height = lines * size / 100;
    let col = (columns - width) / 2 - 2;
    let row = (lines - height) / 2 - 2;

    [row, col, width, height]
}
