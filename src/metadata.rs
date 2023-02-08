use anyhow::{bail, Result};
use etcetera::base_strategy::{self, BaseStrategy, Xdg};
use std::{
    cell::OnceCell,
    path::{Path, PathBuf},
};

#[derive(Debug, Default)]
pub struct Metadata {
    config_file: OnceCell<PathBuf>,
    log_file: OnceCell<PathBuf>,
}

impl Metadata {
    pub fn load() -> Result<Self> {
        Ok(Self::default())
    }

    /// Returns the configuration file path. Creates the parent directory if is does not exist.
    pub fn config_file(&self) -> &PathBuf {
        self.config_file.get_or_init(|| {
            let file = default_config_file();
            // Unwrap OK since we are below the `configuration` directory
            self.create_dir_all(file.parent().unwrap());
            file
        })
    }

    /// Returns the log file path. Creates the parent directory if is does not exist.
    pub fn log_file(&self) -> &PathBuf {
        self.log_file.get_or_init(|| {
            let file = default_log_file();
            // Unwrap OK since we are below the `cache` directory
            self.create_dir_all(file.parent().unwrap());
            file
        })
    }

    /// Sets the configuration file path.
    ///
    /// # Errors
    ///
    /// Returns and error if configuration file path is inititialized.
    pub fn set_config_file(&self, path: &Path) -> Result<()> {
        match self.config_file.set(path.to_path_buf()) {
            Ok(_) => self.create_dir_all(path),
            Err(path) => bail!(
                "cannot override configuration file path: {}",
                path.display()
            ),
        }

        Ok(())
    }

    /// Sets the log file path.
    ///
    /// # Errors
    ///
    /// Returns and error if log file path is inititialized.
    pub fn set_log_file(&self, path: &Path) -> Result<()> {
        match self.log_file.set(path.to_path_buf()) {
            Ok(_) => self.create_dir_all(path),
            Err(path) => bail!("cannot override log file path: {}", path.display()),
        }

        Ok(())
    }

    // Recursively create a directory and all of its parent components if they are missing.
    fn create_dir_all(&self, path: &Path) {
        if !path.exists() {
            std::fs::create_dir_all(path).ok();
        }
    }
}

fn base_dir() -> Xdg {
    // We want to panic if we don't have a base strategy!
    base_strategy::choose_base_strategy().expect("Unable to find base directory")
}

// Returns the configuration directory.
pub fn config_dir() -> PathBuf {
    let base = base_dir();
    let mut path = base.config_dir();
    path.push(env!("CARGO_PKG_NAME"));
    path
}

// Returns the cache directory.
pub fn cache_dir() -> PathBuf {
    let base = base_dir();
    let mut path = base.cache_dir();
    path.push(env!("CARGO_PKG_NAME"));
    path
}

/// Returns the default configuration file path.
pub fn default_config_file() -> PathBuf {
    let config_dir = config_dir();
    config_dir.join("config.toml")
}

/// Returns the default log file path.
pub fn default_log_file() -> PathBuf {
    let cache_dir = cache_dir();
    cache_dir.join(concat!(env!("CARGO_PKG_NAME"), ".log"))
}

// Returns the local configuration directories.
pub fn local_config_dirs() -> Vec<PathBuf> {
    let config_dir_name = concat!('.', env!("CARGO_PKG_NAME"));
    let current_dir = std::env::current_dir().expect("unable to get the current working directory");
    let mut dirs = Vec::new();

    for ancestor in current_dir.ancestors() {
        // Stop at the root of a git repository.
        if ancestor.join(".git").exists() {
            dirs.push(ancestor.to_path_buf());
            break;
        } else if ancestor.join(config_dir_name).is_dir() {
            dirs.push(ancestor.to_path_buf());
        }
    }

    let dirs = dirs.iter().map(|path| path.join(config_dir_name)).collect();

    log::debug!("Located local configuration files: {:?}", dirs);

    dirs
}
