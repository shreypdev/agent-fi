//! Normalize JSON (trim strings, stable key order).

use crate::error::ParseError;
use serde_json::{Map, Value};
use std::collections::BTreeMap;

/// Recursively sort object keys (deterministic serialization).
pub fn sort_json_value_keys(v: &Value) -> Value {
    match v {
        Value::Object(map) => {
            let sorted: BTreeMap<String, Value> = map
                .iter()
                .map(|(k, val)| (k.clone(), sort_json_value_keys(val)))
                .collect();
            Value::Object(sorted.into_iter().collect())
        }
        Value::Array(arr) => Value::Array(arr.iter().map(sort_json_value_keys).collect()),
        _ => v.clone(),
    }
}

fn trim_value(v: &Value) -> Result<Value, ParseError> {
    match v {
        Value::String(s) => Ok(Value::String(s.trim().to_string())),
        Value::Array(arr) => {
            let out: Result<Vec<_>, _> = arr.iter().map(trim_value).collect();
            Ok(Value::Array(out?))
        }
        Value::Object(map) => {
            let mut out = Map::new();
            for (k, val) in map.iter() {
                out.insert(k.clone(), trim_value(val)?);
            }
            Ok(Value::Object(out))
        }
        _ => Ok(v.clone()),
    }
}

/// Sort keys then trim all string leaves.
pub fn normalize_card_value(v: &Value) -> Result<Value, ParseError> {
    let sorted = sort_json_value_keys(v);
    trim_value(&sorted)
}
