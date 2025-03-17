/*------------------------------------- PLUGIN RESULT STRUCT -------------------------------------*/

pub(crate) type Result<T> = std::result::Result<T, Error>;

// CHECK: nesting error sources from `source()` for detailed error messages

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("(std::io): {0}")]
    Io(#[from] std::io::Error),
    #[error("(serde_json): {0}")]
    Json(#[from] ::serde_json::Error),
    #[error("(nvim_oxi): {0}")]
    Nvim(#[from] ::nvim_oxi::Error),
    #[error("(internal): {0}")]
    Internal(String),
}

impl Error {
    pub(crate) fn new<M, T>(message: M) -> Result<T>
    where
        M: std::fmt::Display,
    {
        Err(Self::Internal(message.to_string()))
    }
}

trait NvimErrorVariant: Into<::nvim_oxi::Error> {}
impl NvimErrorVariant for ::nvim_oxi::api::Error {}

impl<E> From<E> for Error
where
    E: NvimErrorVariant,
{
    fn from(nvim_error_variant: E) -> Self {
        Self::Nvim(nvim_error_variant.into())
    }
}
