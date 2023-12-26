use std::process::{Command, Output, Stdio};

/// Fetch all tmux session names and return as a string.
///
/// # Errors
/// Returns an error if the command execution fails or if there are issues
/// with UTF-8 conversion of the command's output.
fn all_sessions() -> Result<String, String> {
    // Command to list all the session name, the `-F` flag lists only name.
    let all_sessions_command = "tmux list-sessions -F #S";

    match execute_tmux_command(all_sessions_command) {
        Ok(output) => {
            // if the command execution was successful return the list of session names
            // else return the error
            if output.status.success() {
                match String::from_utf8(output.stdout) {
                    Ok(all_sessions) => Ok(all_sessions),
                    Err(e) => Err(format!(
                        "Error: Command is not valid utf-8 string: {}",
                        e.utf8_error()
                    )),
                }
            } else {
                match String::from_utf8(output.stderr) {
                    Ok(e) => Err(format!("Error:- {}", e)),
                    Err(e) => Err(format!(
                        "Error: Command is not valid utf-8 string: {}",
                        e.utf8_error()
                    )),
                }
            }
        }
        Err(e) => Err(format!("Command execution failed: {}", e)),
    }
}

/// Takes command as an input and returns the command's output.
fn execute_tmux_command(command: &str) -> Result<Output, std::io::Error> {
    let args: Vec<&str> = command.split(' ').skip(1).collect();
    Command::new("tmux")
        .args(args)
        .stdin(Stdio::inherit())
        .output()
}

fn main() {
    match all_sessions() {
        Ok(all_sessions) => println!("all sessions: {}", all_sessions),
        Err(err) => println!("{}", err),
    };
}
