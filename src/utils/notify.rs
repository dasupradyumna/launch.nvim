/*------------------------------------- NOTIFICATION HELPERS -------------------------------------*/

use ::nvim_oxi::api;

pub(crate) trait NotifyInput {
    fn handle(&self) -> String;
}

impl NotifyInput for &str {
    fn handle(&self) -> String {
        self.to_string()
    }
}

impl NotifyInput for String {
    fn handle(&self) -> String {
        self.clone()
    }
}

impl<T> NotifyInput for &[T]
where
    T: AsRef<str>,
{
    fn handle(&self) -> String {
        let mut result = String::new();
        for string in self.iter() {
            result.push_str(string.as_ref());
            result.push('\n');
        }
        result.pop();
        result
    }
}

pub(crate) fn _send(level: api::types::LogLevel, msg: String) {
    let _ = api::notify(&format!("[launch.nvim] {msg}"), level, &api::opts::NotifyOpts::default());
}

macro_rules! input(
    ([$( $string:expr ,)*]) => {
        &[$( $string ,)*][..]
    };
    ($string:expr) => { $string };
);

macro_rules! send(
    ($level:ident: $msg:tt) => {{
        use crate::utils::notify::*;
        _send(::nvim_oxi::api::types::LogLevel::$level, input!($msg).handle());
    }};
);

pub(crate) use {input, send};
