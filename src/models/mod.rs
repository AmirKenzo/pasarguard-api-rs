//! Request and response models, one module per API area.

mod admin;
mod admin_role;
mod api_key;
mod client_template;
mod common;
mod core;
mod groups;
mod host;
mod node;
mod proxy;
mod settings;
mod subscription;
mod system;
mod user;
mod user_hwid;
mod user_template;

pub use crate::enums::*;
pub use admin::*;
pub use admin_role::*;
pub use api_key::*;
pub use client_template::*;
pub use common::*;
pub use core::*;
pub use groups::*;
pub use host::*;
pub use node::*;
pub use proxy::*;
pub use settings::*;
pub use subscription::*;
pub use system::*;
pub use user::*;
pub use user_hwid::*;
pub use user_template::*;
