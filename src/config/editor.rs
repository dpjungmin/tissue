use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields, default)]
pub struct EditorConfig {
    auto_pairs: bool,
    line_number: String,
    mouse: bool,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            auto_pairs: true,
            line_number: "absolute".into(),
            mouse: true,
        }
    }
}
