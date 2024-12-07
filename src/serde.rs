pub use envir_derive::*;

use std::collections::HashMap;

pub trait Serialize {
    fn export(&self) {
        for (k, v) in self.collect() {
            crate::set(&k, v);
        }
    }

    fn collect(&self) -> HashMap<String, String>;
}

pub trait Deserialize {
    fn from_env() -> crate::Result<Self>
    where
        Self: Sized,
    {
        let env = crate::collect();

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
    fn try_replace<'t, F: FnMut(&regex::Captures) -> crate::Result<String>>(
        regex: &regex::Regex,
        text: &'t str,
        mut rep: F,
    ) -> crate::Result<std::borrow::Cow<'t, str>> {
        let mut it = regex.captures_iter(text).peekable();

        if it.peek().is_none() {
            return Ok(std::borrow::Cow::Borrowed(text));
        }

        let mut new = String::with_capacity(text.len());
        let mut last_match = 0;
        for cap in it {
            // unwrap on 0 is OK because captures only reports matches
            let m = cap.get(0).unwrap();
            new.push_str(&text[last_match..m.start()]);
            new.push_str(&rep(&cap)?);
            last_match = m.end();
        }
        new.push_str(&text[last_match..]);
        Ok(std::borrow::Cow::Owned(new))
    }

    #[cfg(feature = "extrapolation")]
    let default = default
        .map(|x| {
            let regex = regex::Regex::new(r#"\$\{ *(?P<name>.*?) *\}"#).unwrap();

            try_replace(&regex, &x, |caps: &regex::Captures| {
                crate::get(&caps["name"])
            })
            .map(|x| x.to_string())
        })
        .transpose()?;

    env.get(var)
        .or(default.as_ref())
        .map(|x| {
            x.parse::<T>()
                .map_err(|e| crate::Error::parse::<T, _>(var, e.to_string()))
        })
        .transpose()
}

#[cfg(test)]
mod test {
    #[test]
    fn deserialize() {
        #[derive(Debug, PartialEq, crate::Deserialize)]
        #[envir(prefix = "ENV_")]
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
        #[envir(prefix = "ENV2_")]
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
        struct Test3 {
            #[envir(nested)]
            nested: Nested,
        }

        #[derive(Debug, PartialEq, crate::Deserialize, crate::Serialize)]
        #[envir(prefix = "ENV3_")]
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
    #[cfg(feature = "extrapolation")]
    fn extrapolation_error() {
        #[derive(crate::Deserialize)]
        struct Test {
            #[envir(default = "${MISSING_ENV}/.config")]
            _config_dir: String,
        }

        assert!(crate::from_env::<Test>().is_err());
    }

    #[test]
    fn skip_export() {
        use crate::Serialize as _;

        #[derive(crate::Serialize)]
        struct Test {
            #[envir(skip_export)]
            #[allow(dead_code)]
            skip_export: String,
        }

        let test = Test {
            skip_export: "skip".to_string(),
        };

        test.export();

        assert!(std::env::var("SKIP_EXPORT").is_err());
    }

    #[test]
    fn skip_load() -> crate::Result {
        #[derive(crate::Deserialize)]
        struct Test {
            #[envir(skip_load)]
            home: String,
        }

        let test = crate::from_env::<Test>()?;

        assert!(test.home.is_empty());

        Ok(())
    }
}
