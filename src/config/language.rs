use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields, default)]
pub struct LanguageConfig {
    pub name: String,                    // rust
    pub file_types: Vec<String>,         // [rs]
    pub roots: Vec<String>,              // [Cargo.toml, Cargo.lock]
    pub language_server: Option<String>, // rust-analyzer
    pub debugger: Option<String>,        // lldb-vscode
}
