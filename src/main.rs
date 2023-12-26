use std::io::Cursor;
use std::process::{Command, Output, Stdio};

use skim::prelude::{SkimItemReader, SkimOptionsBuilder};
use skim::Skim;

/// CustomErrors for command execution.
enum TsmErrors {
    /// The parsing of stdout into string failed.
    NonUtf8Path,
    /// The skim fuzzy finder command failed.
    FuzzyFindError(String),
    /// The tmux command failed.
    CommandExecutionFailed(String),
    /// The operation was cancelled.
    OperationCancelled,
}

/// Fetch all tmux session names and return as a string.
///
/// # Errors
/// Returns an error if the command execution fails or if there are issues
/// with UTF-8 conversion of the command's output.
fn all_sessions() -> Result<String, TsmErrors> {
    // Command to list all the session name, the `-F` flag lists only name.
    let all_sessions_command = "tmux list-sessions -F #S";

    let output = execute_tmux_command(all_sessions_command)?;
    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|_| TsmErrors::NonUtf8Path)
    } else {
        String::from_utf8(output.stderr)
            .map_err(|_| TsmErrors::CommandExecutionFailed("Command Failed".to_string()))
    }
}

/// Takes command as an input and returns the command's output.
fn execute_tmux_command(command: &str) -> Result<Output, TsmErrors> {
    Command::new("tmux")
        .args(command.split(' ').skip(1))
        .stdin(Stdio::inherit())
        .output()
        .map_err(|e| TsmErrors::CommandExecutionFailed(e.to_string()))
}

/// Takes the session names as input and returns either the matched session
/// name or the input query.
fn session_name(input: String) -> Result<String, TsmErrors> {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .color(Some("dark"))
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    let selected_item = Skim::run_with(&options, Some(items))
        .ok_or_else(|| TsmErrors::FuzzyFindError("Error: Cannot select the option".to_string()))?;

    if selected_item.is_abort {
        return Err(TsmErrors::OperationCancelled);
    }

    let val = selected_item.selected_items.get(0).map_or_else(
        || selected_item.query.to_string(),
        |item| item.text().to_string(),
    );

    Ok(val)
}

fn run() -> Result<(), TsmErrors> {
    let user_input = all_sessions()?;
    let user_session_name = session_name(user_input)?;
    println!("{user_session_name}");
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => match e {
            TsmErrors::NonUtf8Path => println!("Error: Command is not valid utf-8 string"),
            TsmErrors::FuzzyFindError(e) => println!("Error: {e}"),
            TsmErrors::CommandExecutionFailed(e) => println!("Error: {e}"),
            TsmErrors::OperationCancelled => println!("Operation cancelled"),
        },
    };
}
