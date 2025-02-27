/*------------------------------------- PLUGIN RESULT STRUCT -------------------------------------*/

#[derive(Debug)]
pub(crate) enum Error {
    Io(std::io::Error),
    Json(::serde_json::Error),
    Nvim(::nvim_oxi::Error),
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Self::Io(io_error)
    }
}

impl From<::serde_json::Error> for Error {
    fn from(json_error: ::serde_json::Error) -> Self {
        Self::Json(json_error)
    }
}

impl From<::nvim_oxi::Error> for Error {
    fn from(nvim_error: ::nvim_oxi::Error) -> Self {
        Self::Nvim(nvim_error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(io_error) => std::fmt::Display::fmt(io_error, f),
            Self::Json(json_error) => std::fmt::Display::fmt(json_error, f),
            Self::Nvim(nvim_error) => std::fmt::Display::fmt(nvim_error, f),
        }
    }
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
