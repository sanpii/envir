#[warn(warnings)]

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Parse(String),
    Missing(String),
    Var(std::env::VarError),
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Self::Var(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Parse(s) => s.clone(),
            Self::Missing(v) => format!("Missing '{v}' environment variable"),
            Self::Var(e) => e.to_string(),
        };

        write!(f, "{s}")
    }
}
