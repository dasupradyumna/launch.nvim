/*------------------------------------- PLUGIN RESULT STRUCT -------------------------------------*/
//!
//! This module contains result and error type used by the plugin

/// The result type used by the plugin
pub(crate) type Result<T> = std::result::Result<T, Error>;

// CHECK: nesting error sources from `source()` for detailed error messages

/// The error type used by the plugin
///
/// Disallows explicit creation ; primarily to be used via type coercion
#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    /// Wraps errors from std::io
    #[error("(std::io): {0}")]
    Io(#[from] std::io::Error),
    /// Wraps errors from serde_json
    #[error("(serde_json): {0}")]
    Json(#[from] ::serde_json::Error),
    /// Wraps errors from nvim_oxi
    #[error("(nvim_oxi): {0}")]
    Nvim(#[from] ::nvim_oxi::Error),
    /// An error internal to the plugin
    #[error("(internal): {0}")]
    Internal(String),
}

impl Error {
    /// Creates a new internal error instance
    pub(crate) fn new<M, T>(message: M) -> Result<T>
    where
        M: std::fmt::Display,
    {
        Err(Self::Internal(message.to_string()))
    }
}

/// Indicator trait that supports coercion from nvim_oxi nested error types
trait NvimErrorVariant: Into<::nvim_oxi::Error> {}
impl NvimErrorVariant for ::nvim_oxi::api::Error {}

/// Coerce nvim_oxi nested errors into plugin error variant
impl<E> From<E> for Error
where
    E: NvimErrorVariant,
{
    fn from(nvim_error_variant: E) -> Self {
        Self::Nvim(nvim_error_variant.into())
    }
}

/// Helper trait for indexing with error handling
pub(crate) trait IndexChecked<T> {
    /// Returns a Result-wrapped immutable reference to the element at the given index
    fn get_checked(&self, index: usize) -> Result<&T>;

    /// Returns a Result-wrapped mutable reference to the element at the given index
    fn get_mut_checked(&mut self, index: usize) -> Result<&mut T>;
}

/// IndexChecked implementation for Vec<T>
impl<T> IndexChecked<T> for Vec<T> {
    fn get_checked(&self, index: usize) -> Result<&T> {
        let caller = std::panic::Location::caller();
        self.get(index).ok_or_else(|| {
            Error::Internal(format!(
                "{}:{}:{}: Index out of bounds for vector",
                caller.file(),
                caller.line(),
                caller.column()
            ))
        })
    }

    fn get_mut_checked(&mut self, index: usize) -> Result<&mut T> {
        let caller = std::panic::Location::caller();
        self.get_mut(index).ok_or_else(|| {
            Error::Internal(format!(
                "{}:{}:{}: Index out of bounds for vector",
                caller.file(),
                caller.line(),
                caller.column()
            ))
        })
    }
}
