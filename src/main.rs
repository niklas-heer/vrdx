//! Terminal lifecycle and the small command-line entry point.

use std::{
    env,
    ffi::OsString,
    io::{self, IsTerminal},
    path::{Path, PathBuf},
    process::ExitCode,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use color_eyre::eyre::{Result, eyre};
use crossterm::{
    event::{
        self, DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
        Event, KeyCode, KeyModifiers,
    },
    execute,
};
use vrdx::app::App;

const HELP: &str = "vrdx — engineering decisions in Markdown\n\nUsage: vrdx [DIRECTORY]\n\n  -h, --help       Show this help\n  -V, --version    Show the version\n\nLaunches the Ratatui editor in DIRECTORY (default: current directory).\nKeys: n new, Enter edit, 1–4 panes, r refresh, ? help, q quit.\nEditing: Tab next field, Ctrl+S save, Escape cancel, Ctrl+Q quit.\n";

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Help,
    Version,
    Run(PathBuf),
}

fn parse_args(args: impl IntoIterator<Item = OsString>) -> Result<Command> {
    let mut directory = None;
    let mut positional_only = false;
    for argument in args {
        if !positional_only {
            if argument == "--help" || argument == "-h" {
                return Ok(Command::Help);
            }
            if argument == "--version" || argument == "-V" {
                return Ok(Command::Version);
            }
            if argument == "--" {
                positional_only = true;
                continue;
            }
            if argument.to_string_lossy().starts_with('-') {
                return Err(eyre!(
                    "Unknown option: {}. Use --help.",
                    argument.to_string_lossy()
                ));
            }
        }
        if directory.replace(PathBuf::from(argument)).is_some() {
            return Err(eyre!("Expected at most one directory. Use --help."));
        }
    }
    Ok(Command::Run(
        directory.unwrap_or_else(|| PathBuf::from(".")),
    ))
}

/// The extra terminal modes must be restored even when rendering or input fails.
struct InputModes;

impl Drop for InputModes {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), DisableBracketedPaste, DisableMouseCapture);
    }
}

fn run(directory: &Path) -> Result<()> {
    let root = directory.canonicalize()?;
    if !root.is_dir() {
        return Err(eyre!("Not a directory: {}", root.display()));
    }
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err(eyre!(
            "vrdx needs an interactive terminal. Use --help for usage."
        ));
    }
    let mut app = App::new(root)?;
    let interrupted = Arc::new(AtomicBool::new(false));
    let signal_ids = [signal_hook::consts::SIGINT, signal_hook::consts::SIGTERM]
        .into_iter()
        .map(|signal| signal_hook::flag::register(signal, Arc::clone(&interrupted)))
        .collect::<io::Result<Vec<_>>>()?;
    let result = ratatui::run(|terminal| -> Result<()> {
        let _input_modes = InputModes;
        execute!(io::stdout(), EnableBracketedPaste, EnableMouseCapture)?;
        while !app.quit && !interrupted.load(Ordering::Relaxed) {
            terminal.draw(|frame| app.draw(frame))?;
            if event::poll(Duration::from_millis(100))? {
                let event = event::read()?;
                if matches!(event, Event::Key(key) if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL))
                {
                    break;
                }
                app.handle_event(event);
            }
        }
        Ok(())
    });
    for id in signal_ids {
        signal_hook::low_level::unregister(id);
    }
    result
}

fn start() -> Result<()> {
    color_eyre::install()?;
    match parse_args(env::args_os().skip(1))? {
        Command::Help => print!("{HELP}"),
        Command::Version => println!("vrdx {}", env!("CARGO_PKG_VERSION")),
        Command::Run(directory) => run(&directory)?,
    }
    Ok(())
}

fn main() -> ExitCode {
    match start() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:?}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_version_and_directory() {
        assert_eq!(
            parse_args([OsString::from("--help")]).unwrap(),
            Command::Help
        );
        assert_eq!(
            parse_args([OsString::from("--version")]).unwrap(),
            Command::Version
        );
        assert_eq!(parse_args([]).unwrap(), Command::Run(PathBuf::from(".")));
        assert_eq!(
            parse_args([OsString::from("--"), OsString::from("-repo")]).unwrap(),
            Command::Run(PathBuf::from("-repo"))
        );
        assert!(parse_args([OsString::from("--unknown")]).is_err());
        assert!(parse_args([OsString::from("a"), OsString::from("b")]).is_err());
    }
}
