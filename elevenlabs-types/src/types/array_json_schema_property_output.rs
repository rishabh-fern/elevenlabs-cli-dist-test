pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ArrayJsonSchemaPropertyOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Schema for array elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<ArrayJsonSchemaPropertyOutputItems>>,
    /// When set, the entire array is populated from this dynamic variable at runtime. Mutually exclusive with description (LLM-provided array), constant_value, and is_omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variable: Option<String>,
    /// When set, the entire array uses this constant value at runtime. Mutually exclusive with description (LLM-provided array), dynamic_variable, and is_omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_value: Option<Vec<ArrayJsonSchemaPropertyOutputConstantValueItem>>,
    /// If true, this array parameter will be completely omitted from the request. Only valid for optional parameters. Mutually exclusive with description, dynamic_variable, and constant_value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_omitted: Option<bool>,
}

impl ArrayJsonSchemaPropertyOutput {
    pub fn builder() -> ArrayJsonSchemaPropertyOutputBuilder {
        <ArrayJsonSchemaPropertyOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ArrayJsonSchemaPropertyOutputBuilder {
    r#type: Option<String>,
    description: Option<String>,
    items: Option<Box<ArrayJsonSchemaPropertyOutputItems>>,
    dynamic_variable: Option<String>,
    constant_value: Option<Vec<ArrayJsonSchemaPropertyOutputConstantValueItem>>,
    is_omitted: Option<bool>,
}

impl ArrayJsonSchemaPropertyOutputBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn items(mut self, value: Box<ArrayJsonSchemaPropertyOutputItems>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn dynamic_variable(mut self, value: impl Into<String>) -> Self {
        self.dynamic_variable = Some(value.into());
        self
    }

    pub fn constant_value(mut self, value: Vec<ArrayJsonSchemaPropertyOutputConstantValueItem>) -> Self {
        self.constant_value = Some(value);
        self
    }

    pub fn is_omitted(mut self, value: bool) -> Self {
        self.is_omitted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ArrayJsonSchemaPropertyOutput`].
    pub fn build(self) -> Result<ArrayJsonSchemaPropertyOutput, BuildError> {
        Ok(ArrayJsonSchemaPropertyOutput {
            r#type: self.r#type,
            description: self.description,
            items: self.items,
            dynamic_variable: self.dynamic_variable,
            constant_value: self.constant_value,
            is_omitted: self.is_omitted,
        })
    }
}
