use super::LanguageConfig;
use crate::metadata;
use anyhow::{anyhow, Result};
use serde::Deserialize;

/// Syntax configuration for all languages.
#[derive(Debug, Default, Deserialize)]
pub struct SyntaxConfig {
    #[serde(rename(deserialize = "language"))]
    pub languages: Vec<LanguageConfig>,
}

impl SyntaxConfig {
    pub fn load() -> Result<Self> {
        metadata::local_config_dirs()
            .into_iter()
            .chain([metadata::config_dir()].into_iter())
            .map(|path| path.join("syntax.toml"))
            .filter_map(|file| {
                std::fs::read_to_string(file)
                    .map(|s| toml::from_str(&s))
                    .ok()
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .chain([load_default_syntax_config()].into_iter())
            .fold(default_toml_table(), |acc, x| toml_merge(acc, x, 3))
            .try_into()
            .map_err(|e| anyhow!(e))
    }

    pub fn load_default() -> Self {
        load_default_syntax_config()
            .try_into()
            .expect("unable to parse built-in 'syntax.toml'")
    }
}

fn load_default_syntax_config() -> toml::Value {
    let bytes = include_bytes!("./syntax.toml");
    let s = unsafe { std::str::from_utf8_unchecked(bytes) };
    toml::from_str(s).expect("unable to parse built-in 'syntax.toml'")
}

fn default_toml_table() -> toml::Value {
    toml::Value::Table(toml::value::Table::default())
}

// Merge TOML values from `a` to `b`.
fn toml_merge(a: toml::Value, b: toml::Value, depth: usize) -> toml::Value {
    use toml::Value;

    match (a, b) {
        (Value::Array(left_items), Value::Array(mut right_items)) => {
            if depth > 0 {
                right_items.reserve(left_items.len());
                for lvalue in left_items {
                    let rvalue = lvalue
                        .get("name")
                        .and_then(Value::as_str)
                        .and_then(|lname| {
                            right_items
                                .iter()
                                .position(|v| v.get("name").and_then(Value::as_str) == Some(lname))
                        })
                        .map(|rpos| right_items.remove(rpos));
                    let mvalue = match rvalue {
                        Some(rvalue) => toml_merge(rvalue, lvalue, depth - 1),
                        None => lvalue,
                    };
                    right_items.push(mvalue);
                }

                Value::Array(right_items)
            } else {
                Value::Array(left_items)
            }
        }
        // The merged table consists of all keys unioned by `a` and `b`. The values in `a` will be
        // merged recursively onto values of `b`.
        (Value::Table(left_map), Value::Table(mut right_map)) => {
            if depth > 0 {
                for (lname, lvalue) in left_map {
                    match right_map.remove(&lname) {
                        Some(rvalue) => {
                            let mvalue = toml_merge(rvalue, lvalue, depth - 1);
                            right_map.insert(lname, mvalue);
                        }
                        None => {
                            right_map.insert(lname, lvalue);
                        }
                    }
                }
                Value::Table(right_map)
            } else {
                Value::Table(left_map)
            }
        }
        (_, x) => x,
    }
}
