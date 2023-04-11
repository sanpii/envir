pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Parse(String),
    Missing(String),
    Unicode(std::ffi::OsString),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Parse(s) => s.clone(),
            Self::Missing(v) => format!("Missing '{v}' environment variable"),
            Self::Unicode(s) => format!("environment variable was not valid unicode: {s:?}"),
        };

        write!(f, "{s}")
    }
}

impl std::error::Error for Error {}
