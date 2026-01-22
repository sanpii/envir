pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "extrapolation")]
    #[error("Command '{command}' failed: {output:?}")]
    Command {
        command: String,
        output: std::process::Output,
    },
    #[cfg(feature = "dotenv")]
    #[error("{0}")]
    Dotenv(#[from] dotenvy::Error),
    #[cfg(feature = "extrapolation")]
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[cfg(feature = "logger")]
    #[error("{0}")]
    Logger(String),
    #[error("Enable to parse '{key}' variable to '{ty}': {error}")]
    Parse {
        key: String,
        ty: String,
        error: String,
    },
    #[error("Missing '{0}' environment variable")]
    Missing(String),
    #[error("environment variable '{key}' was not valid unicode: {value:?}")]
    Unicode {
        key: String,
        value: std::ffi::OsString,
    },
}

impl Error {
    pub(crate) fn parse<T, E: ToString>(key: &str, error: E) -> Self {
        Self::Parse {
            key: key.to_string(),
            ty: std::any::type_name::<T>().to_string(),
            error: error.to_string(),
        }
    }

    pub(crate) fn unicode(key: &str, value: std::ffi::OsString) -> Self {
        Self::Unicode {
            key: key.to_string(),
            value,
        }
    }
}
