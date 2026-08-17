pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AlertingWebhookNotifierResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<AlertingWebhookMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<AlertingWebhookHeaderResponse>>,
}

impl AlertingWebhookNotifierResponse {
    pub fn builder() -> AlertingWebhookNotifierResponseBuilder {
        <AlertingWebhookNotifierResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlertingWebhookNotifierResponseBuilder {
    r#type: Option<String>,
    url: Option<String>,
    method: Option<AlertingWebhookMethod>,
    headers: Option<Vec<AlertingWebhookHeaderResponse>>,
}

impl AlertingWebhookNotifierResponseBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn method(mut self, value: AlertingWebhookMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn headers(mut self, value: Vec<AlertingWebhookHeaderResponse>) -> Self {
        self.headers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AlertingWebhookNotifierResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](AlertingWebhookNotifierResponseBuilder::url)
    pub fn build(self) -> Result<AlertingWebhookNotifierResponse, BuildError> {
        Ok(AlertingWebhookNotifierResponse {
            r#type: self.r#type,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            method: self.method,
            headers: self.headers,
        })
    }
}
