use anyhow::{anyhow, Context, Result};
use std::{path::PathBuf, process};
use tissue::{metadata, App, Args, Cli, Config, Health, HealthCategory, Metadata, Verbosity};

fn print_help_and_exit() -> ! {
    println!(
        "{}",
        format_args!(
            "\
{name}
{description}

USAGE:
    {bin} [OPTIONS] [files]...

ARGS:
    <files>...    The input files

OPTIONS:
    -h, --help                   Prints help information
    -V, --version                Prints version information
    --health [CATEGORY]          Performs health check and prints the result (default: 'all')
                                 ('all', 'clipboard', 'languages', or a language)
    -c, --config [PATH]          Specifies the configuration file
                                 (default: '{config_file}')
    -l, --log [PATH]             Specifies the log file
                                 (default: '{log_file}')
    -v [LEVEL]                   Specifies the logging verbosity (default: 0)
                                 (0: WARN, 1: INFO, 2: DEBUG, 3: TRACE)",
            name = env!("CARGO_PKG_NAME"),
            description = env!("CARGO_PKG_DESCRIPTION"),
            bin = env!("CARGO_BIN_NAME"),
            config_file = metadata::default_config_file().display(),
            log_file = metadata::default_log_file().display(),
        )
    );
    process::exit(0);
}

fn print_version_and_exit() -> ! {
    println!("{} {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
    process::exit(0);
}

fn print_health_and_exit(category: Option<String>) -> ! {
    let category = HealthCategory::from(category);

    if let Err(err) = Health::check(category) {
        eprintln!("{err}");
        process::exit(1);
    }

    process::exit(0);
}

fn args() -> Result<Args> {
    let cli = Cli::parse().context(format!(
        "could not parse arguments (run '{} --help' for more information)",
        env!("CARGO_BIN_NAME")
    ))?;

    let args = match cli {
        Cli::Help => print_help_and_exit(),
        Cli::Version => print_version_and_exit(),
        Cli::Health(arg) => print_health_and_exit(arg),
        Cli::Options(args) => args,
    };

    Ok(args)
}

fn setup_logger(log_file_path: &PathBuf, verbosity: Verbosity) -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S%.3f]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(verbosity.into())
        .chain(fern::log_file(log_file_path)?)
        .apply()
        .map_err(|err| anyhow!("{err}"))
}

type ExitCode = i32;

#[tokio::main]
pub async fn tissue_main() -> Result<ExitCode> {
    let args = args()?;
    let metadata = Metadata::load()?;

    if let Some(path) = &args.config_file {
        metadata.set_config_file(path).ok();
        debug_assert_eq!(metadata.config_file(), path);
    }

    if let Some(path) = &args.log_file {
        metadata.set_log_file(path).ok();
        debug_assert_eq!(metadata.log_file(), path);
    }

    setup_logger(metadata.log_file(), args.verbosity).context("failed to setup logger")?;

    let config = match Config::load(metadata.config_file()) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Bad config: {err}");
            eprintln!("Press <ENTER> to continue with default configuration");
            use std::io::Read;
            std::io::stdin().read(&mut []).ok();
            Config::default()
        }
    };

    let app = App::new(args, config).context("unable to create new application")?;

    app.run().await
}

fn main() -> anyhow::Result<()> {
    std::process::exit(tissue_main()?);
}
