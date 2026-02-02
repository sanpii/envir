use std::collections::HashMap;

pub fn from_env<T>() -> crate::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    serde_envfile::from_env().map_err(crate::Error::from)
}

pub fn from<T>(env: &HashMap<String, String>) -> crate::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let iter = env.iter().map(|(k, v)| (k.clone(), v.clone()));

    serde_envfile::de::from_iter(iter).map_err(crate::Error::from)
}

#[cfg(feature = "derive")]
impl crate::derive::Serialize for serde_envfile::Value {
    fn collect(&self) -> HashMap<String, String> {
        use std::ops::Deref as _;

        self.deref().clone()
    }
}
