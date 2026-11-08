use std::collections::BTreeMap;
use std::fmt;

use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Extra(pub Map<String, Value>);

impl Extra {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.0.insert(key.into(), value.into());
        self
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<Value>) {
        self.0.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Serialize for Extra {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        for (key, value) in &self.0 {
            if !value.is_null() {
                map.serialize_entry(key, value)?;
            }
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Extra {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Map::deserialize(deserializer).map(Self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AnyPayload {
    Empty,
    Json(Value),
    Text(String),
}

impl AnyPayload {
    pub(crate) fn from_body(body: &[u8]) -> Self {
        if body.is_empty() {
            return Self::Empty;
        }
        match serde_json::from_slice(body) {
            Ok(value) => Self::Json(value),
            Err(_) => Self::Text(String::from_utf8_lossy(body).into_owned()),
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
            _ => None,
        }
    }
}

pub mod lax_int {
    use super::*;

    pub fn from_value(value: &Value) -> Option<i64> {
        match value {
            Value::Bool(flag) => Some(i64::from(*flag)),
            Value::Number(number) => number.as_i64().or_else(|| {
                number
                    .as_f64()
                    .filter(|f| f.is_finite() && f.fract() == 0.0 && f.abs() < 9.2e18)
                    .map(|f| f as i64)
            }),
            Value::String(text) => {
                let text = text.trim();
                text.parse::<i64>().ok().or_else(|| {
                    text.parse::<f64>()
                        .ok()
                        .filter(|f| f.is_finite() && f.fract() == 0.0 && f.abs() < 9.2e18)
                        .map(|f| f as i64)
                })
            }
            _ => None,
        }
    }

    pub fn option<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<i64>, D::Error> {
        let value = Value::deserialize(deserializer)?;
        if value.is_null() {
            return Ok(None);
        }
        from_value(&value).map(Some).ok_or_else(|| {
            de::Error::custom(format!("input should be a valid integer, got {value}"))
        })
    }
}

pub(crate) fn ordered_entries<'de, D, T>(deserializer: D) -> Result<Vec<(String, T)>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct EntriesVisitor<T>(std::marker::PhantomData<T>);

    impl<'de, T: Deserialize<'de>> Visitor<'de> for EntriesVisitor<T> {
        type Value = Vec<(String, T)>;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a JSON object")
        }

        fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
            let mut entries = Vec::with_capacity(access.size_hint().unwrap_or(0));
            while let Some((key, value)) = access.next_entry::<String, T>()? {
                entries.push((key, value));
            }
            Ok(entries)
        }
    }

    deserializer.deserialize_map(EntriesVisitor(std::marker::PhantomData))
}

pub(crate) fn default_some_zero() -> Option<i64> {
    Some(0)
}

pub(crate) fn default_some_true() -> Option<bool> {
    Some(true)
}

pub(crate) fn default_some_false() -> Option<bool> {
    Some(false)
}

pub(crate) fn default_some_empty_string() -> Option<String> {
    Some(String::new())
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CustomVariable {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BaseNotificationEnable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Conflict {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Forbidden {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HTTPException {
    pub detail: String,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HTTPRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, Vec<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HTTPResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, Vec<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HTTPValidationError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<Vec<ValidationError>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NotFound {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Unauthorized {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValidationError {
    pub loc: Vec<Value>,
    pub msg: String,
    pub r#type: String,
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub input: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctx: Option<Map<String, Value>>,
    #[serde(flatten)]
    pub extra: Extra,
}
