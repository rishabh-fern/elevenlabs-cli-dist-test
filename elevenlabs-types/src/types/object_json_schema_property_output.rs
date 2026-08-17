pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ObjectJsonSchemaPropertyOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Box<ObjectJsonSchemaPropertyOutputPropertiesValue>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_constraints: Option<RequiredConstraints>,
}

impl ObjectJsonSchemaPropertyOutput {
    pub fn builder() -> ObjectJsonSchemaPropertyOutputBuilder {
        <ObjectJsonSchemaPropertyOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ObjectJsonSchemaPropertyOutputBuilder {
    r#type: Option<String>,
    required: Option<Vec<String>>,
    description: Option<String>,
    properties: Option<HashMap<String, Box<ObjectJsonSchemaPropertyOutputPropertiesValue>>>,
    required_constraints: Option<RequiredConstraints>,
}

impl ObjectJsonSchemaPropertyOutputBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn required(mut self, value: Vec<String>) -> Self {
        self.required = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn properties(mut self, value: HashMap<String, Box<ObjectJsonSchemaPropertyOutputPropertiesValue>>) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn required_constraints(mut self, value: RequiredConstraints) -> Self {
        self.required_constraints = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ObjectJsonSchemaPropertyOutput`].
    pub fn build(self) -> Result<ObjectJsonSchemaPropertyOutput, BuildError> {
        Ok(ObjectJsonSchemaPropertyOutput {
            r#type: self.r#type,
            required: self.required,
            description: self.description,
            properties: self.properties,
            required_constraints: self.required_constraints,
        })
    }
}
