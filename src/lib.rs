#![warn(warnings)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod errors;
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
mod serde;

#[cfg(feature = "serde")]
pub use envir_derive::*;
#[cfg(feature = "serde")]
pub use serde::*;

pub use errors::{Result, Error};

pub fn dotenv() {
    dotenvy::dotenv().ok();
}

#[must_use]
pub fn dump() -> std::collections::HashMap<String, String> {
    std::env::vars().collect()
}

pub fn try_get<T: std::str::FromStr>(key: &str) -> crate::Result<Option<T>>
where
    T::Err: ToString,
{
    let Some(value) = std::env::var_os(key) else {
        return Ok(None);
    };

    let value = match value.to_str() {
        Some(v) => v
            .parse::<T>()
            .map_err(|e| crate::Error::parse::<T, _>(key, e.to_string()))?,
        None => return Err(crate::Error::unicode(key, value)),
    };

    Ok(Some(value))
}

pub fn get<T: std::str::FromStr>(key: &str) -> crate::Result<T>
where
    T::Err: ToString,
{
    crate::try_get(key)?.ok_or_else(|| crate::Error::Missing(key.to_string()))
}

pub fn set<T: ToString>(key: &str, value: T) {
    std::env::set_var(key, value.to_string());
}

#[cfg(test)]
mod test {
    #[test]
    fn dump() {
        assert!(!crate::dump().is_empty());
    }

    #[test]
    fn try_get() -> crate::Result {
        assert!(crate::try_get::<String>("MISSING_ENV")?.is_none());

        Ok(())
    }

    #[test]
    fn get() -> crate::Result {
        crate::set("TEST", 1);
        assert_eq!(crate::get::<u8>("TEST")?, 1u8);

        Ok(())
    }
}
