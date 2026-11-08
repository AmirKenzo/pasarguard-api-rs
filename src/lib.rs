#![doc = include_str!("../README.md")]

pub mod client;
pub mod credential;
pub mod datetime;
pub mod encoding;
pub mod enums;
pub mod error;
pub mod jwt;
pub mod models;
pub mod query;
mod secret;

pub use client::{
    HttpOptions, PanelClient, build_http_client, build_http_client_with, shared_http_client,
};
pub use credential::Credential;
pub use datetime::{DateTimeOrInt, PanelDateTime};
pub use error::{HttpStatusError, PanelError, PanelResult};
pub use reqwest::{self, Method};
pub use secret::Secret;
