use crate::{LanguageConfig, SyntaxConfig};
use anyhow::Result;
use crossterm::style::Stylize;
use std::io::Write;
use which::which;

#[derive(Debug)]
pub enum HealthCategory {
    All,
    Clipboard,
    Languages,
    Language(String),
}

impl From<Option<String>> for HealthCategory {
    fn from(s: Option<String>) -> Self {
        match s.as_deref() {
            None | Some("all") => Self::All,
            Some("clipboard") => Self::Clipboard,
            Some("languages") => Self::Languages,
            Some(lang) => Self::Language(lang.into()),
        }
    }
}

pub struct Health {}

impl Health {
    pub fn check(category: HealthCategory) -> Result<()> {
        match category {
            HealthCategory::All => check_all()?,
            HealthCategory::Clipboard => check_clipboard()?,
            HealthCategory::Languages => check_languages()?,
            HealthCategory::Language(lang) => check_language(lang)?,
        }

        Ok(())
    }
}

// TODO: Add additional information.
fn check_all() -> Result<()> {
    check_languages()
}

fn check_clipboard() -> Result<()> {
    todo!()
}

fn check_languages() -> Result<()> {
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    // TODO: Add tree-sitter features.
    let headings = vec!["Language", "LSP", "DAP"];

    let columns = crossterm::terminal::size().map(|(c, _)| c).unwrap_or(80);
    let column_width = columns as usize / headings.len();

    let format = |v: &str| {
        format!(
            "{:width$}",
            v.get(..column_width - 2)
                .map(|v| format!("{v}…"))
                .unwrap_or_else(|| v.to_string()),
            width = column_width,
        )
    };

    let check_program = |name: Option<String>| match name {
        Some(name) => match which(&name) {
            Ok(_) => format(&format!("✓ {name}")).green(),
            Err(_) => format(&format!("✘ {name}")).red(),
        },
        None => format("None").yellow(),
    };

    for heading in headings {
        write!(stdout, "{}", format(heading).blue().bold())?;
    }

    writeln!(stdout)?;

    let mut syntax_config = match SyntaxConfig::load() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            eprintln!("{}", "Using default syntax configuration".yellow());
            SyntaxConfig::load_default()
        }
    };

    syntax_config.languages.sort_by(|a, b| a.name.cmp(&b.name));

    for LanguageConfig {
        name,
        language_server,
        debugger,
        ..
    } in syntax_config.languages
    {
        write!(stdout, "{}", format(&name).reset())?;
        write!(stdout, "{}", check_program(language_server))?;
        write!(stdout, "{}", check_program(debugger))?;
        writeln!(stdout)?;
    }

    Ok(())
}

fn check_language(language: String) -> Result<()> {
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    let languages = match SyntaxConfig::load() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            eprintln!("{}", "Using default syntax configuration".yellow());
            SyntaxConfig::load_default()
        }
    }
    .languages;

    let Some(language)= languages.iter().find(|l| l.name == language) else {
        writeln!(
            stdout,
            "{}",
            format!("Language '{language}' not found").red()
        )?;

        let suggestions: Vec<_> = languages
            .into_iter()
            .filter(|lang| lang.name.starts_with(language.chars().next().unwrap()))
            .map(|lang| lang.name)
            .collect();

        match suggestions.len() {
            0 => return Ok(()),
            1 => writeln!(stdout, "Did you mean {} ?", suggestions[0].clone().yellow())?,
            _ => writeln!(
                stdout,
                "Did you mean one of these: {} ?",
                suggestions.join(", ").yellow()
            )?,
        };

        return Ok(());
    };

    let language_server = match language.language_server {
        Some(ref name) => match which(name) {
            Ok(_) => name.to_string().green(),
            Err(_) => format!("{name} (Not Found)").red(),
        },
        None => "None".to_string().yellow(),
    };

    let debug_adapter = match language.debugger {
        Some(ref name) => match which(name) {
            Ok(_) => name.to_string().green(),
            Err(_) => format!("{name} (Not Found)").red(),
        },
        None => "None".to_string().yellow(),
    };

    writeln!(stdout, "Language-server : {language_server}")?;
    writeln!(stdout, "Debug-adapter   : {debug_adapter}")?;

    Ok(())
}
