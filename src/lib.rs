#![warn(
    clippy::todo,
    nonstandard_style,
    missing_debug_implementations,
    missing_docs
)]
#![feature(once_cell)]

mod app;
mod args;
mod config;
mod health;
pub mod metadata;

pub use self::{
    app::App,
    args::{Args, Cli, Verbosity},
    config::{Config, EditorConfig, LanguageConfig, SyntaxConfig},
    health::{Health, HealthCategory},
    metadata::Metadata,
};

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Normal = 0,
    Insert = 1,
    Visual = 2,
}

#[derive(Debug, Default)]
pub struct KeyBinding {}
