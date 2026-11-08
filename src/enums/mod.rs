//! String and integer enums used by the panel API, one module per API area.

macro_rules! str_enum {
    ($(#[$meta:meta])* $name:ident { $($variant:ident => $value:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        pub enum $name {
            $(#[serde(rename = $value)] $variant),+
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $value),+
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value {
                    $($value => Ok(Self::$variant),)+
                    other => Err(format!("invalid {} value: {other:?}", stringify!($name))),
                }
            }
        }

        impl crate::query::QueryValue for $name {
            fn to_query(&self) -> String {
                self.as_str().to_owned()
            }
        }
    };
}

mod admin;
mod api_key;
mod client_template;
mod core;
mod general;
mod node;
mod proxy;
mod user;

pub use admin::*;
pub use api_key::*;
pub use client_template::*;
pub use core::*;
pub use general::*;
pub use node::*;
pub use proxy::*;
pub use user::*;
