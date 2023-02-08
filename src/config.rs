mod editor;
mod language;
mod syntax;

pub use self::{editor::EditorConfig, language::LanguageConfig, syntax::SyntaxConfig};

use crate::{KeyBinding, Mode};
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

type KeyBindings = HashMap<Mode, KeyBinding>;

/// Global configuration.
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub theme: Option<String>,
    // TODO: Load default keybindings.
    #[serde(skip)]
    pub keys: KeyBindings,
    #[serde(default)]
    pub editor: EditorConfig,
    #[serde(skip)]
    pub syntax: SyntaxConfig,
}

impl Config {
    pub fn load(config_file_path: &PathBuf) -> Result<Self> {
        let mut config = match std::fs::read_to_string(config_file_path) {
            Ok(s) => toml::from_str(&s).map_err(|err| anyhow!(err))?,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Self::default(),
            Err(err) => return Err(err.into()),
        };

        config.syntax = SyntaxConfig::load()?;

        Ok(config)
    }
}
