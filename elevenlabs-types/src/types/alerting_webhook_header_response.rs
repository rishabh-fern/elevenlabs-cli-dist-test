pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AlertingWebhookHeaderResponse {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub is_secret: bool,
    /// The header value. Always null for secret headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl AlertingWebhookHeaderResponse {
    pub fn builder() -> AlertingWebhookHeaderResponseBuilder {
        <AlertingWebhookHeaderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlertingWebhookHeaderResponseBuilder {
    name: Option<String>,
    is_secret: Option<bool>,
    value: Option<String>,
}

impl AlertingWebhookHeaderResponseBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn is_secret(mut self, value: bool) -> Self {
        self.is_secret = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AlertingWebhookHeaderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AlertingWebhookHeaderResponseBuilder::name)
    /// - [`is_secret`](AlertingWebhookHeaderResponseBuilder::is_secret)
    pub fn build(self) -> Result<AlertingWebhookHeaderResponse, BuildError> {
        Ok(AlertingWebhookHeaderResponse {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_secret: self.is_secret.ok_or_else(|| BuildError::missing_field("is_secret"))?,
            value: self.value,
        })
    }
}
