pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AlertingSettingsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_configs: Option<HashMap<String, AlertingMonitorConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_resolve_after_inactive_minutes: Option<i64>,
}

impl AlertingSettingsResponse {
    pub fn builder() -> AlertingSettingsResponseBuilder {
        <AlertingSettingsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlertingSettingsResponseBuilder {
    monitor_configs: Option<HashMap<String, AlertingMonitorConfig>>,
    auto_resolve_after_inactive_minutes: Option<i64>,
}

impl AlertingSettingsResponseBuilder {
    pub fn monitor_configs(mut self, value: HashMap<String, AlertingMonitorConfig>) -> Self {
        self.monitor_configs = Some(value);
        self
    }

    pub fn auto_resolve_after_inactive_minutes(mut self, value: i64) -> Self {
        self.auto_resolve_after_inactive_minutes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AlertingSettingsResponse`].
    pub fn build(self) -> Result<AlertingSettingsResponse, BuildError> {
        Ok(AlertingSettingsResponse {
            monitor_configs: self.monitor_configs,
            auto_resolve_after_inactive_minutes: self.auto_resolve_after_inactive_minutes,
        })
    }
}
