#![warn(warnings)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

mod errors;
#[cfg(any(feature = "env_logger", feature = "tracing"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "env_logger", feature = "tracing"))))]
pub mod logger;
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
mod serde;

#[cfg(feature = "serde")]
pub use serde::*;

pub use errors::{Error, Result};

/**
 * Loads the *.env* file and initializes the logger.
 */
pub fn init() {
    #[cfg(all(feature = "dotenv", debug_assertions))]
    dotenv();

    #[cfg(feature = "logger")]
    logger::init();
}

/**
 * Attempts to load the *.env* file and to initialize the logger.
 */
pub fn try_init() -> Result {
    #[cfg(all(feature = "dotenv", debug_assertions))]
    dotenvy::dotenv()?;

    #[cfg(feature = "logger")]
    logger::try_init().map_err(|e| Error::Logger(e.to_string()))?;

    Ok(())
}

/**
 * Loads the *.env* files.
 */
#[cfg(feature = "dotenv")]
#[cfg_attr(docsrs, doc(cfg(feature = "dotenv")))]
pub fn dotenv() {
    dotenvy::dotenv().ok();
}

/**
 * Retreives all environment variables as an easy printable form.
 */
#[must_use]
pub fn collect() -> std::collections::HashMap<String, String> {
    std::env::vars().collect()
}

/**
 * Likes `try_get` but directly parses the variable value in desired `T` type.
 */
pub fn try_parse<T: std::str::FromStr>(key: &str) -> crate::Result<Option<T>>
where
    T::Err: ToString,
{
    let value = match crate::try_get(key)? {
        Some(v) => v
            .parse::<T>()
            .map_err(|e| crate::Error::parse::<T, _>(key, e.to_string()))?,
        None => return Ok(None),
    };

    Ok(Some(value))
}

/**
 * Likes `try_parse` but returns a `crate::Error::Missing` if the variable isn’t set.
 */
pub fn parse<T: std::str::FromStr>(key: &str) -> crate::Result<T>
where
    T::Err: ToString,
{
    crate::try_parse(key)?.ok_or_else(|| crate::Error::Missing(key.to_string()))
}

/**
 * Gets the environment variable `key`. This returns a `crate::Error::Unicode` if the variable
 * value isn’t valid unicode.
 */
pub fn try_get(key: &str) -> crate::Result<Option<String>> {
    let Some(value) = std::env::var_os(key) else {
        return Ok(None);
    };

    let value = match value.to_str() {
        Some(v) => v.to_string(),
        None => return Err(crate::Error::unicode(key, value)),
    };

    Ok(Some(value))
}

/**
 * Likes `try_get` but returns a `crate::Error::Missing` if the variable isn’t set.
 */
pub fn get(key: &str) -> crate::Result<String> {
    crate::try_get(key)?.ok_or_else(|| crate::Error::Missing(key.to_string()))
}

/**
 * Sets the environment variable `key` to the `value`.
 */
pub fn set<T: ToString>(key: &str, value: T) {
    std::env::set_var(key, value.to_string());
}

#[cfg(test)]
mod test {
    #[test]
    fn collect() {
        assert!(!crate::collect().is_empty());
    }

    #[test]
    fn try_parse() -> crate::Result {
        assert!(crate::try_parse::<String>("MISSING_ENV")?.is_none());

        Ok(())
    }

    #[test]
    fn parse() -> crate::Result {
        crate::set("TEST", 1);
        assert_eq!(crate::parse::<u8>("TEST")?, 1u8);

        Ok(())
    }

    #[test]
    fn try_get() -> crate::Result {
        assert!(crate::try_get("MISSING_ENV")?.is_none());

        Ok(())
    }

    #[test]
    fn get() -> crate::Result {
        crate::set("TEST", 1);
        assert_eq!(crate::get("TEST")?, "1");

        Ok(())
    }
}
