use anyhow::{bail, Result};
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Copy)]
pub enum Verbosity {
    #[default]
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<Verbosity> for log::LevelFilter {
    fn from(v: Verbosity) -> Self {
        match v {
            Verbosity::Warn => Self::Warn,
            Verbosity::Info => Self::Info,
            Verbosity::Debug => Self::Debug,
            Verbosity::Trace => Self::Trace,
        }
    }
}

#[derive(Debug, Default)]
pub struct Args {
    pub config_file: Option<PathBuf>,
    pub log_file: Option<PathBuf>,
    pub verbosity: Verbosity,
    pub files: Vec<PathBuf>,
}

#[derive(Debug)]
pub enum Cli {
    Help,
    Version,
    Health(Option<String>),
    Options(Args),
}

impl Default for Cli {
    fn default() -> Self {
        Self::Options(Args::default())
    }
}

impl Cli {
    /// Parses the command line arguments.
    pub fn parse() -> Result<Self> {
        let mut args = Args::default();
        let mut argv = std::env::args().skip(1).peekable();

        while let Some(arg) = argv.next() {
            match arg.as_str() {
                "-h" | "--help" => return Ok(Self::Help),
                "-V" | "--version" => return Ok(Self::Version),
                "--health" => {
                    let arg = argv.next_if(|v| !v.starts_with('-'));
                    return Ok(Self::Health(arg));
                }
                "-c" | "--config" => match argv.next().as_deref() {
                    Some(path) => args.config_file = Some(path.into()),
                    None => bail!("missing [PATH] argument for '{}' option", arg),
                },
                "-l" | "--log" => match argv.next().as_deref() {
                    Some(path) => args.log_file = Some(path.into()),
                    None => bail!("missing [PATH] argument for '{}' option", arg),
                },
                "-v" => match argv.next().as_deref() {
                    Some(level) => {
                        args.verbosity = match level {
                            "0" => Verbosity::Warn,
                            "1" => Verbosity::Info,
                            "2" => Verbosity::Debug,
                            "3" => Verbosity::Trace,
                            _ => bail!("invalid [LEVEL] for '{}' option", arg),
                        };
                    }
                    None => bail!("missing [LEVEL] argument for '{}' option", arg),
                },
                arg if arg.starts_with("--") => bail!("unexpected argument: {}", arg),
                arg => args.files.push(PathBuf::from(arg)),
            }
        }

        argv.for_each(|arg| args.files.push(PathBuf::from(arg)));

        Ok(Self::Options(args))
    }
}
