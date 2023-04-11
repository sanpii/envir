pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Parse(Parse),
    Missing(String),
    Unicode(Unicode),
}

#[derive(Debug)]
pub struct Parse {
    key: String,
    ty: String,
    error: String,
}

impl Parse {
    pub(crate) fn new<T, E: ToString>(key: &str, error: E) -> crate::Error {
        crate::Error::Parse(Self {
            key: key.to_string(),
            ty: std::any::type_name::<T>().to_string(),
            error: error.to_string(),
        })
    }
}

#[derive(Debug)]
pub struct Unicode {
    key: String,
    value: std::ffi::OsString,
}

impl Unicode {
    pub(crate) fn new(key: &str, value: std::ffi::OsString) -> crate::Error {
        crate::Error::Unicode(Self {
            key: key.to_string(),
            value,
        })
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Parse(Parse { key, ty, error }) => format!("Enable to parse '{key}' variable to '{ty}': {error}"),
            Self::Missing(v) => format!("Missing '{v}' environment variable"),
            Self::Unicode(Unicode { key, value }) => format!("environment variable '{key}' was not valid unicode: {value:?}"),
        };

        write!(f, "{s}")
    }
}

impl std::error::Error for Error {}
