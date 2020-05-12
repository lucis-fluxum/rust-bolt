use std::convert::TryFrom;

use crate::error::*;
use crate::value::String;
use crate::{impl_try_from_value, Value};

impl From<&str> for String {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl From<std::string::String> for String {
    fn from(value: std::string::String) -> Self {
        Self { value }
    }
}

impl_try_from_value!(String, String);

impl From<String> for std::string::String {
    fn from(string: String) -> Self {
        string.value
    }
}

impl TryFrom<Value> for std::string::String {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::String(string) => Ok(string.value),
            _ => Err(ConversionError::FromValue(value).into()),
        }
    }
}
