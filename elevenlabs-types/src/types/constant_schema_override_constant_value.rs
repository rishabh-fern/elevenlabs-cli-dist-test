pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConstantSchemaOverrideConstantValue {
        String(String),

        Integer(i64),

        Double(f64),

        Boolean(bool),

        ConstantSchemaOverrideConstantValueFourItemList(Vec<ConstantSchemaOverrideConstantValueFourItem>),
}

impl ConstantSchemaOverrideConstantValue {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_constant_schema_override_constant_value_four_item_list(&self) -> bool {
        matches!(self, Self::ConstantSchemaOverrideConstantValueFourItemList(_))
    }


    pub fn as_string(&self) -> Option<&str> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_integer(&self) -> Option<&i64> {
        match self {
                    Self::Integer(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_integer(self) -> Option<i64> {
        match self {
                    Self::Integer(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_double(&self) -> Option<&f64> {
        match self {
                    Self::Double(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_double(self) -> Option<f64> {
        match self {
                    Self::Double(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
                    Self::Boolean(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_boolean(self) -> Option<bool> {
        match self {
                    Self::Boolean(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_constant_schema_override_constant_value_four_item_list(&self) -> Option<&Vec<ConstantSchemaOverrideConstantValueFourItem>> {
        match self {
                    Self::ConstantSchemaOverrideConstantValueFourItemList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_constant_schema_override_constant_value_four_item_list(self) -> Option<Vec<ConstantSchemaOverrideConstantValueFourItem>> {
        match self {
                    Self::ConstantSchemaOverrideConstantValueFourItemList(value) => Some(value),
                    _ => None,
                }
    }
}
