use skim::prelude::{SkimItemReader, SkimOptionsBuilder};
use skim::Skim;
use std::io::Cursor;

use crate::tmux::fetch_all_sessions;

/// Custom errors for command execution.
pub enum TsmErrors {
    /// The parsing of stdout into string failed.
    NonUtf8Path,
    /// The skim fuzzy finder command failed.
    FuzzyFindError(String),
    /// The tmux command failed.
    CommandExecutionFailed(String),
    /// The operation was cancelled.
    OperationCancelled,
}

/// Takes the session names as input and returns either the matched session
/// name or the input query.
///
/// # Params
/// * input(String) - all session names delimited by carriage return
///
/// # Errors
/// * TsmErrors enum
fn get_session_name(input: String) -> Result<String, TsmErrors> {
    // default skim options taken from readme page
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .color(Some("dark"))
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    // if there are any errors while running the fuzzy finder return the error
    let selected_item = Skim::run_with(&options, Some(items))
        .ok_or_else(|| TsmErrors::FuzzyFindError("Error: Cannot select the option".to_string()))?;

    // if the operation is cancelled then return the error
    if selected_item.is_abort {
        return Err(TsmErrors::OperationCancelled);
    }

    // return the selected session name and if the session name does not match
    // then return the input query which will be used to create a new session
    let val = selected_item.selected_items.get(0).map_or_else(
        || selected_item.query.to_string(),
        |item| item.text().to_string(),
    );

    Ok(val)
}

/// Execute all the commands and propogate errors
pub fn run() -> Result<(), TsmErrors> {
    let all_sessions = fetch_all_sessions()?;
    let session_name = get_session_name(all_sessions.clone())?;
    println!("{session_name}");
    Ok(())
}