use serde::{Deserialize, Serialize};

/// Options for parsing TSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOptions {
    /// Whether to allow comments in TSON
    pub allow_comments: Option<bool>,
    /// Custom options specific to implementation
    pub custom: Option<serde_json::Value>,
}

pub type Res<T> = std::result::Result<T, String>;
pub type Map<K, V> = std::collections::HashMap<K, V>;

pub enum TsonValue {
    Object(Map<String, TsonValue>),
    Array(Vec<TsonValue>),
    String(String),
    Int(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl From<serde_json::Value> for TsonValue {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Object(map) => {
                TsonValue::Object(map.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
            serde_json::Value::Array(arr) => {
                TsonValue::Array(arr.into_iter().map(|v| v.into()).collect())
            }
            serde_json::Value::String(s) => TsonValue::String(s),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    TsonValue::Int(i)
                } else {
                    TsonValue::Float(n.as_f64().unwrap())
                }
            }
            serde_json::Value::Bool(b) => TsonValue::Boolean(b),
            serde_json::Value::Null => TsonValue::Null,
        }
    }
}

impl From<TsonValue> for serde_json::Value {
    fn from(value: TsonValue) -> Self {
        match value {
            TsonValue::Object(map) => {
                serde_json::Value::Object(map.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
            TsonValue::Array(arr) => {
                serde_json::Value::Array(arr.into_iter().map(|v| v.into()).collect())
            }
            TsonValue::String(s) => serde_json::Value::String(s),
            TsonValue::Int(n) => serde_json::Value::Number(serde_json::Number::from(n)),
            TsonValue::Float(n) => {
                serde_json::Value::Number(serde_json::Number::from_f64(n).unwrap())
            }
            TsonValue::Boolean(b) => serde_json::Value::Bool(b),
            TsonValue::Null => serde_json::Value::Null,
        }
    }
}

impl From<&TsonValue> for serde_json::Value {
    fn from(value: &TsonValue) -> Self {
        match value {
            TsonValue::Object(map) => {
                let new_map = map.iter().map(|(k, v)| (k.clone(), v.into())).collect();
                serde_json::Value::Object(new_map)
            }
            TsonValue::Array(arr) => {
                let new_arr = arr.iter().map(|v| v.into()).collect();
                serde_json::Value::Array(new_arr)
            }
            TsonValue::String(s) => serde_json::Value::String(s.clone()),
            TsonValue::Int(n) => serde_json::Value::Number(serde_json::Number::from(*n)),
            TsonValue::Float(n) => {
                serde_json::Value::Number(serde_json::Number::from_f64(*n).unwrap())
            }
            TsonValue::Boolean(b) => serde_json::Value::Bool(*b),
            TsonValue::Null => serde_json::Value::Null,
        }
    }
}

impl std::fmt::Debug for TsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val: serde_json::Value = self.into();
        write!(f, "{:?}", val)
    }
}

impl std::fmt::Display for TsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val: serde_json::Value = self.into();
        write!(f, "{}", val)
    }
}
