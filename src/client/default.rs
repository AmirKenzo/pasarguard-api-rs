use super::*;
use serde_json::{Map, Value};

impl PanelClient {
    /// Fetch the panel root page as text.
    pub async fn base(&self) -> PanelResult<String> {
        let raw = self
            .send(Method::GET, "/", Query::new(), Body::None, false)
            .await?;
        Ok(String::from_utf8_lossy(&raw.body).into_owned())
    }

    /// Fetch the panel health status.
    pub async fn health(&self) -> PanelResult<Map<String, Value>> {
        let raw = self
            .send(Method::GET, "/health", Query::new(), Body::None, false)
            .await?;
        decode(raw)
    }
}
