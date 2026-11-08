use crate::datetime::PanelDateTime;
use crate::encoding::urlencode;

pub trait QueryValue {
    fn to_query(&self) -> String;
}

impl QueryValue for bool {
    fn to_query(&self) -> String {
        if *self { "true".into() } else { "false".into() }
    }
}

impl QueryValue for i64 {
    fn to_query(&self) -> String {
        self.to_string()
    }
}

impl QueryValue for i32 {
    fn to_query(&self) -> String {
        self.to_string()
    }
}

impl QueryValue for u32 {
    fn to_query(&self) -> String {
        self.to_string()
    }
}

impl QueryValue for usize {
    fn to_query(&self) -> String {
        self.to_string()
    }
}

impl QueryValue for str {
    fn to_query(&self) -> String {
        self.to_owned()
    }
}

impl QueryValue for String {
    fn to_query(&self) -> String {
        self.clone()
    }
}

impl QueryValue for PanelDateTime {
    fn to_query(&self) -> String {
        self.isoformat()
    }
}

impl<T: QueryValue + ?Sized> QueryValue for &T {
    fn to_query(&self) -> String {
        (**self).to_query()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Query {
    pairs: Vec<(String, String)>,
}

impl Query {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn opt<V: QueryValue>(mut self, key: &str, value: Option<V>) -> Self {
        if let Some(value) = value {
            self.pairs.push((key.to_owned(), value.to_query()));
        }
        self
    }

    pub fn set<V: QueryValue>(self, key: &str, value: V) -> Self {
        self.opt(key, Some(value))
    }

    pub fn list<V: QueryValue>(mut self, key: &str, values: Option<&[V]>) -> Self {
        for value in values.unwrap_or_default() {
            self.pairs.push((key.to_owned(), value.to_query()));
        }
        self
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    pub fn pairs(&self) -> &[(String, String)] {
        &self.pairs
    }

    pub fn encode(&self) -> String {
        urlencode(self.pairs.iter().map(|(k, v)| (k.as_str(), v.as_str())))
    }
}
