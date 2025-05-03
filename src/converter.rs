

use crate::types::TsonValue;

/// TSON Stringifier
pub struct TSONStringifier;

impl TSONStringifier {
    /// Stringify Rust value to TSON string
    pub fn stringify(value: &TsonValue, pretty: bool) -> String {
        if pretty {
            Self::stringify_pretty(value, 0)
        } else {
            Self::stringify_value(value)
        }
    }

    fn stringify_value(value: &TsonValue) -> String {
        match value {
            TsonValue::Null => {
                "~".to_string()
            },
            TsonValue::Boolean(b) => {
                if *b {
                    "?true".to_string()
                } else {
                    "?false".to_string()
                }
            }
            TsonValue::Int(n) => {
                format!("#{}", n)
            }
            TsonValue::Float(n) => {
                format!("={}", n)
            }
            TsonValue::String(s) => format!("\"{}\"", Self::escape_string(s)),
            TsonValue::Array(arr) => {
                let mut result = String::from("[");
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        result.push_str(" ");
                    }
                    result.push_str(&Self::stringify_value(item));
                }
                result.push(']');
                result
            }
            TsonValue::Object(obj) => {
                let mut result = String::from("{");
                for (i, (key, value)) in obj.iter().enumerate() {
                    if i > 0 {
                        result.push_str(" ");
                    }
                    result.push_str(key);
                    result.push_str(&Self::stringify_value(value));
                }
                result.push('}');
                result
            }
        }
    }

    fn stringify_pretty(value: &TsonValue, depth: usize) -> String {
        let indent = "  ".repeat(depth);
        let indent_inner = "  ".repeat(depth + 1);

        match value {
            TsonValue::Object(obj) => {
                if obj.is_empty() {
                    return "{}".to_string();
                }
                
                let mut result = String::from("{\n");
                
                for (i, (key, val)) in obj.iter().enumerate() {
                    if i > 0 {
                        result.push_str("\n");
                    }
                    result.push_str(&indent_inner);
                    result.push_str(key);
                    result.push_str(&Self::stringify_pretty(val, depth + 1));
                }
                
                result.push_str("\n");
                result.push_str(&indent);
                result.push('}');
                result
            }
            TsonValue::Array(arr) => {
                if arr.is_empty() {
                    return "[]".to_string();
                }
                
                let mut result = String::from("[\n");
                
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        result.push_str("\n");
                    }
                    result.push_str(&indent_inner);
                    result.push_str(&Self::stringify_pretty(item, depth + 1));
                }
                
                result.push_str("\n");
                result.push_str(&indent);
                result.push(']');
                result
            }
            _ => Self::stringify_value(value),
        }
    }

    fn escape_string(s: &str) -> String {
        s.replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("\n", "\\n")
            .replace("\r", "\\r")
            .replace("\t", "\\t")
    }
} 