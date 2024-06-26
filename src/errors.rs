pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "dotenv")]
    Dotenv(dotenvy::Error),
    #[cfg(feature = "logger")]
    Logger(String),
    Parse(Parse),
    Missing(String),
    Unicode(Unicode),
}

impl Error {
    pub(crate) fn parse<T, E: ToString>(key: &str, error: E) -> Self {
        Self::Parse(Parse {
            key: key.to_string(),
            ty: std::any::type_name::<T>().to_string(),
            error: error.to_string(),
        })
    }

    pub(crate) fn unicode(key: &str, value: std::ffi::OsString) -> Self {
        Self::Unicode(Unicode {
            key: key.to_string(),
            value,
        })
    }
}

#[cfg(feature = "dotenv")]
impl From<dotenvy::Error> for Error {
    fn from(value: dotenvy::Error) -> Self {
        Self::Dotenv(value)
    }
}

#[derive(Debug)]
pub struct Parse {
    key: String,
    ty: String,
    error: String,
}

#[derive(Debug)]
pub struct Unicode {
    key: String,
    value: std::ffi::OsString,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            #[cfg(feature = "dotenv")]
            Self::Dotenv(error) => error.to_string(),
            #[cfg(feature = "logger")]
            Self::Logger(error) => error.clone(),
            Self::Parse(Parse { key, ty, error }) => {
                format!("Enable to parse '{key}' variable to '{ty}': {error}")
            }
            Self::Missing(v) => format!("Missing '{v}' environment variable"),
            Self::Unicode(Unicode { key, value }) => {
                format!("environment variable '{key}' was not valid unicode: {value:?}")
            }
        };

        write!(f, "{s}")
    }
}

impl std::error::Error for Error {}
