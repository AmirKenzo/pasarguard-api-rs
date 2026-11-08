use super::*;

/// Optional device headers sent with subscription requests.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SubscriptionDeviceHeaders {
    pub x_hwid: Option<String>,
    pub x_device_os: Option<String>,
    pub x_ver_os: Option<String>,
    pub x_device_model: Option<String>,
}

impl SubscriptionDeviceHeaders {
    fn pairs(&self) -> impl Iterator<Item = (&'static str, &str)> {
        [
            ("x-hwid", self.x_hwid.as_deref()),
            ("x-device-os", self.x_device_os.as_deref()),
            ("x-ver-os", self.x_ver_os.as_deref()),
            ("x-device-model", self.x_device_model.as_deref()),
        ]
        .into_iter()
        .filter_map(|(name, value)| value.map(|value| (name, value)))
    }
}

impl PanelClient {
    /// Send a HEAD request for a subscription and return the response headers.
    ///
    /// `user_agent` defaults to `""` in Python; `None` keeps the client's own user agent.
    pub async fn user_subscription_head(
        &self,
        token: &str,
        user_agent: Option<&str>,
    ) -> PanelResult<HeaderMap> {
        let (headers, _) = self
            .send_with_headers(
                Method::HEAD,
                &format!("/sub/{token}/"),
                Query::new(),
                Body::None,
                false,
                &user_agent
                    .map(|ua| ("user-agent", ua))
                    .into_iter()
                    .collect::<Vec<_>>(),
            )
            .await?;
        Ok(headers)
    }

    /// Get a user's subscription content as the given user agent would see it.
    ///
    /// `user_agent` defaults to `""` in Python; `None` keeps the client's own user agent.
    pub async fn user_subscription(
        &self,
        token: &str,
        user_agent: Option<&str>,
        device: &SubscriptionDeviceHeaders,
    ) -> PanelResult<AnyPayload> {
        let headers: Vec<_> = user_agent
            .map(|ua| ("user-agent", ua))
            .into_iter()
            .chain(device.pairs())
            .collect();
        let (_, raw) = self
            .send_with_headers(
                Method::GET,
                &format!("/sub/{token}/"),
                Query::new(),
                Body::None,
                false,
                &headers,
            )
            .await?;
        Ok(AnyPayload::from_body(&raw.body))
    }

    /// Get the user information behind a subscription token.
    pub async fn user_subscription_info(
        &self,
        token: &str,
    ) -> PanelResult<SubscriptionUserResponse> {
        let raw = self
            .send(
                Method::GET,
                &format!("/sub/{token}/info"),
                Query::new(),
                Body::None,
                false,
            )
            .await?;
        decode(raw)
    }

    /// Get the raw subscription content for the given subscription user agent.
    ///
    /// `x_subscription_user_agent` defaults to `""` in Python; `None` omits the header.
    pub async fn user_subscription_raw(
        &self,
        token: &str,
        x_subscription_user_agent: Option<&str>,
        device: &SubscriptionDeviceHeaders,
    ) -> PanelResult<AnyPayload> {
        let headers: Vec<_> = x_subscription_user_agent
            .map(|ua| ("x-subscription-user-agent", ua))
            .into_iter()
            .chain(device.pairs())
            .collect();
        let (_, raw) = self
            .send_with_headers(
                Method::GET,
                &format!("/sub/{token}/raw"),
                Query::new(),
                Body::None,
                false,
                &headers,
            )
            .await?;
        Ok(AnyPayload::from_body(&raw.body))
    }

    /// List the client applications offered for a subscription.
    pub async fn user_subscription_apps(&self, token: &str) -> PanelResult<Vec<Application>> {
        let raw = self
            .send(
                Method::GET,
                &format!("/sub/{token}/apps"),
                Query::new(),
                Body::None,
                false,
            )
            .await?;
        decode(raw)
    }

    /// Get the traffic usage of a subscription.
    pub async fn get_sub_user_usage(
        &self,
        token: &str,
        params: &SubUsageParams,
    ) -> PanelResult<UserUsageStatsList> {
        let raw = self
            .send(
                Method::GET,
                &format!("/sub/{token}/usage"),
                sub_usage_query(params),
                Body::None,
                false,
            )
            .await?;
        decode(raw)
    }

    /// Get a subscription rendered in a specific client format.
    pub async fn user_subscription_with_client_type(
        &self,
        token: &str,
        client_type: ConfigFormat,
        device: &SubscriptionDeviceHeaders,
    ) -> PanelResult<AnyPayload> {
        let (_, raw) = self
            .send_with_headers(
                Method::GET,
                &format!("/sub/{token}/{client_type}"),
                Query::new(),
                Body::None,
                false,
                &device.pairs().collect::<Vec<_>>(),
            )
            .await?;
        Ok(AnyPayload::from_body(&raw.body))
    }
}

pub(super) fn sub_usage_query(params: &SubUsageParams) -> Query {
    Query::new()
        .opt("period", params.period)
        .opt("start", params.start.as_ref())
        .opt("end", params.end.as_ref())
}
