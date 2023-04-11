#![warn(warnings)]

mod result;

pub use envir_derive::*;
pub use result::*;

use std::collections::HashMap;

pub trait Serialize {
    fn export(&self) {
        for (k, v) in self.into() {
            std::env::set_var(k, v);
        }
    }

    fn into(&self) -> HashMap<String, String>;
}

pub trait Deserialize {
    fn from_env() -> crate::Result<Self>
    where
        Self: Sized,
    {
        let env = std::env::vars().collect();

        Self::from(&env)
    }

    fn from(env: &HashMap<String, String>) -> crate::Result<Self>
    where
        Self: Sized;
}

pub fn from_env<T>() -> crate::Result<T>
where
    T: Deserialize,
{
    T::from_env()
}

pub fn from<T>(env: &HashMap<String, String>) -> crate::Result<T>
where
    T: Deserialize,
{
    T::from(env)
}

#[doc(hidden)]
pub fn load_optional_var<T: std::str::FromStr>(
    env: &HashMap<String, String>,
    var: &str,
    default: Option<String>,
) -> crate::Result<Option<T>>
where
    T::Err: ToString,
{
    #[cfg(feature = "extrapolation")]
    let default = default.map(|x| {
        let regex = regex::Regex::new(r#"\$\{ *(?P<name>.*?) *\}"#).unwrap();

        regex
            .replace(&x, |caps: &regex::Captures| {
                std::env::var(&caps["name"]).unwrap()
            })
            .to_string()
    });

    env.get(var)
        .or(default.as_ref())
        .map(|x| {
            x.parse::<T>()
                .map_err(|e| crate::result::Parse::new::<T, _>(var, e.to_string()))
        })
        .transpose()
}

#[must_use]
pub fn dump() -> HashMap<String, String> {
    std::env::vars().collect()
}

#[must_use]
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
            .map_err(|e| crate::result::Parse::new::<T, _>(key, e.to_string()))?,
        None => return Err(crate::result::Unicode::new(key, value)),
    };

    Ok(Some(value))
}

#[must_use]
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
    fn deserialize() {
        #[derive(Debug, PartialEq, crate::Deserialize)]
        #[envir(internal, prefix = "ENV_")]
        struct Test {
            #[envir(name = "FOO")]
            field1: String,
            #[envir(default)]
            field2: String,
            #[envir(default = "field3")]
            field3: String,
            field4: u8,
            #[envir(load_with = "load_field5")]
            field5: String,
            field6: Option<char>,
        }

        fn load_field5(_: &std::collections::HashMap<String, String>) -> crate::Result<String> {
            Ok("field5".to_string())
        }

        crate::set("ENV_FOO", "foo");
        crate::set("ENV_FIELD4", 4);

        let test = crate::from_env::<Test>().unwrap();
        assert_eq!(
            test,
            Test {
                field1: "foo".to_string(),
                field2: String::new(),
                field3: "field3".to_string(),
                field4: 4,
                field5: "field5".to_string(),
                field6: None,
            }
        );
    }

    #[test]
    fn serialize() {
        use crate::Serialize;

        #[derive(Debug, PartialEq, crate::Serialize)]
        #[envir(internal, prefix = "ENV2_")]
        struct Test2 {
            #[envir(name = "FOO")]
            field1: String,
            field2: String,
        }

        let test = Test2 {
            field1: "field1".to_string(),
            field2: "field2".to_string(),
        };

        assert!(std::env::var("ENV2_FOO").is_err());
        assert!(std::env::var("ENV2_FIELD2").is_err());

        test.export();

        assert_eq!(std::env::var("ENV2_FOO"), Ok("field1".to_string()));
        assert_eq!(std::env::var("ENV2_FIELD2"), Ok("field2".to_string()));
    }

    #[test]
    fn nested() {
        #[derive(Debug, PartialEq, crate::Deserialize, crate::Serialize)]
        #[envir(internal)]
        struct Test3 {
            #[envir(nested)]
            nested: Nested,
        }

        #[derive(Debug, PartialEq, crate::Deserialize, crate::Serialize)]
        #[envir(internal, prefix = "ENV3_")]
        struct Nested {
            foo: Option<String>,
        }

        let mut env = std::collections::HashMap::new();
        env.insert("ENV3_FOO".to_string(), "foo".to_string());

        let test = crate::from::<Test3>(&env).unwrap();
        assert_eq!(
            test,
            Test3 {
                nested: Nested {
                    foo: Some("foo".to_string()),
                }
            }
        );

        use crate::Serialize;

        assert!(std::env::var("ENV3_FOO").is_err());
        test.export();
        assert_eq!(std::env::var("ENV3_FOO"), Ok("foo".to_string()));
    }

    #[test]
    #[cfg(feature = "extrapolation")]
    fn env() {
        #[derive(crate::Deserialize)]
        #[envir(internal)]
        struct Test4 {
            #[envir(default = "${HOME}/.config")]
            config_dir: String,
        }

        let test = crate::from_env::<Test4>().unwrap();
        assert_eq!(
            Ok(test.config_dir),
            std::env::var("HOME").map(|x| format!("{x}/.config"))
        );
    }

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
