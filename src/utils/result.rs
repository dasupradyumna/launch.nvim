/*------------------------------------- PLUGIN RESULT STRUCT -------------------------------------*/

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] ::serde_json::Error),
    #[error(transparent)]
    Nvim(#[from] ::nvim_oxi::Error),
    #[error("{0}")]
    Plugin(String),
}

impl Error {
    pub(crate) fn new<M, T>(message: M) -> Result<T>
    where
        M: std::fmt::Display,
    {
        Err(Self::Plugin(message.to_string()))
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
