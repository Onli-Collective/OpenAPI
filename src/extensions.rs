use std::{marker::PhantomData, ops::Deref};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(bound = "")]
pub struct Extensions<T> {
    /// Allows extensions to the OpenAPI Schema. The field name MUST begin with x-, for example, x-internal-id. The value can be null, a primitive, an array or an object. Can have any valid JSON format value.
    #[serde(flatten)]
    inner: IndexMap<ExtensionField, serde_json::Value>,

    #[serde(default, skip)]
    _type: PhantomData<T>,
}

impl<T> Default for Extensions<T> {
    fn default() -> Self {
        Self {
            inner: IndexMap::new(),
            _type: PhantomData,
        }
    }
}

impl<T> std::ops::DerefMut for Extensions<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> std::ops::Deref for Extensions<T> {
    type Target = IndexMap<ExtensionField, serde_json::Value>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[serde(try_from = "String")]
pub struct ExtensionField(pub(crate) String);

impl std::ops::Deref for ExtensionField {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

pub struct ParseExtensionFieldError(String);
impl std::fmt::Display for ParseExtensionFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Extension] field [names] MUST begin with `x-` (C4.8), got `{}`",
            self.0
        )
    }
}

impl TryFrom<String> for ExtensionField {
    type Error = ParseExtensionFieldError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("x-") {
            Ok(Self(value))
        } else {
            Err(ParseExtensionFieldError(value))
        }
    }
}

pub(crate) struct MixedPatternVisitor<'de, T, K, V, F>
where
    F: for<'a> FnMut(K, Box<dyn FnOnce() -> Result<V, String> + 'a>) -> Result<(), String>,
{
    name: &'static str,
    func: F,
    __type: PhantomData<(T, K, V, F, &'de ())>,
}

impl<'de, T, K, V, F> MixedPatternVisitor<'de, T, K, V, F>
where
    F: for<'a> FnMut(K, Box<dyn FnOnce() -> Result<V, String> + 'a>) -> Result<(), String>,
{
    pub(crate) fn new(name: &'static str, func: F) -> Self {
        Self {
            name,
            func,
            __type: PhantomData,
        }
    }
}

impl<'de, T, K, V, F> serde::de::Visitor<'de> for &mut MixedPatternVisitor<'de, T, K, V, F>
where
    F: for<'a> FnMut(K, Box<dyn FnOnce() -> Result<V, String> + 'a>) -> Result<(), String>,
    K: DeserializeOwned + Clone + Deref<Target = str>,
    V: DeserializeOwned,
{
    type Value = Extensions<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.name)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use serde::de::Error;
        let mut extensions = Extensions::default();

        while let Some(key) = map.next_key::<K>()? {
            match key {
                ext if ext.starts_with("x-") => extensions
                    .insert(ExtensionField(ext.to_string()), map.next_value()?)
                    .map_or(Ok(()), |_| {
                        Err(A::Error::custom(format!(
                            "duplicate extension key: {}",
                            ext.deref()
                        )))
                    })?,
                key => {
                    let tmp = &mut map;
                    let next_value =
                        Box::new(|| tmp.next_value::<V>().map_err(|err| err.to_string()));
                    (self.func)(key, next_value).map_err(A::Error::custom)?
                }
            }
        }

        Ok(extensions)
    }
}
