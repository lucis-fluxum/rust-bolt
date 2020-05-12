use std::collections::HashMap;

use bolt_proto_derive::*;

use crate::impl_try_from_value;
use crate::value::String;
use crate::Value;

pub(crate) const MARKER: u8 = 0xB3;
pub(crate) const SIGNATURE: u8 = 0x72;

#[derive(Debug, Clone, Eq, PartialEq, Signature, Marker, Serialize, Deserialize)]
pub struct UnboundRelationship {
    pub(crate) rel_identity: i64,
    pub(crate) rel_type: String,
    pub(crate) properties: HashMap<String, Value>,
}

impl UnboundRelationship {
    pub fn new(
        rel_identity: i64,
        rel_type: std::string::String,
        properties: HashMap<std::string::String, impl Into<Value>>,
    ) -> Self {
        Self {
            rel_identity,
            rel_type: rel_type.into(),
            properties: properties
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }

    pub fn rel_identity(&self) -> i64 {
        self.rel_identity
    }

    pub fn rel_type(&self) -> &str {
        &self.rel_type.value
    }

    pub fn properties(&self) -> &HashMap<String, Value> {
        &self.properties
    }
}

impl_try_from_value!(UnboundRelationship, UnboundRelationship);
