use serde::{Deserialize, Deserializer, Serialize, Serializer};

str_enum!(
    AdminAccountStatus {
        Active => "active",
        Disabled => "disabled",
    }
);

str_enum!(
    AdminStatus {
        Active => "active",
        Disabled => "disabled",
        Limited => "limited",
    }
);

/// How far an admin role permission reaches.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PermissionScope {
    #[default]
    None = 0,
    Own = 1,
    All = 2,
}

impl PermissionScope {
    pub fn as_i64(self) -> i64 {
        self as i64
    }

    pub fn from_i64(value: i64) -> Option<Self> {
        match value {
            0 => Some(Self::None),
            1 => Some(Self::Own),
            2 => Some(Self::All),
            _ => None,
        }
    }
}

impl Serialize for PermissionScope {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(self.as_i64())
    }
}

impl<'de> Deserialize<'de> for PermissionScope {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i64::deserialize(deserializer)?;
        Self::from_i64(value).ok_or_else(|| {
            serde::de::Error::custom(format!("invalid PermissionScope value: {value}"))
        })
    }
}

impl crate::query::QueryValue for PermissionScope {
    fn to_query(&self) -> String {
        self.as_i64().to_string()
    }
}
