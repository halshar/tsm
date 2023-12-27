use std::process::{Command, Output, Stdio};

use crate::input::TsmErrors;

/// Fetch all tmux session names and return as a string.
///
/// # Errors
/// Returns an error if the command execution fails or if there are issues
/// with UTF-8 conversion of the command's output.
pub fn fetch_all_sessions() -> Result<String, TsmErrors> {
    // Command to list all the session name, the `-F` flag lists only name.
    let all_sessions_command = "tmux list-sessions -F #S";

    // execute the command and check it's execution status, if it is
    // success then convert the session names to string and return it
    // else return the error as tsm enum
    let output = execute_tmux_command(all_sessions_command)?;
    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|_| TsmErrors::NonUtf8Path)
    } else {
        String::from_utf8(output.stderr)
            .map_err(|_| TsmErrors::CommandExecutionFailed("Command Failed".to_string()))
    }
}

/// Switch to the existing session.
///
/// # Params
/// * session\_name(String) - session name to switch to
pub fn switch_session(session_name: String) -> Result<(), TsmErrors> {
    let switch_session_command = format!("tmux switch-client -t {}", session_name);
    execute_tmux_command(switch_session_command.as_str())?;
    Ok(())
}

/// Create new session and switch to it.
///
/// # Params
/// * session\_name(String) - session to create
pub fn create_session(session_name: String) -> Result<(), TsmErrors> {
    let create_session_command = format!("tmux new-session -ds {} -c ~", session_name);
    execute_tmux_command(create_session_command.as_str())?;
    switch_session(session_name)?;
    Ok(())
}

/// Takes tmux command as an input and returns the command's output.
///
/// # Params
/// * command(&str) - the entire tmux command to be executed
///
/// # Errors
/// * A result containing the command output and tsm enum
pub fn execute_tmux_command(command: &str) -> Result<Output, TsmErrors> {
    Command::new("tmux")
        // split on space and include only the arguments
        .args(command.split(' ').skip(1))
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| TsmErrors::CommandExecutionFailed(e.to_string()))
}
