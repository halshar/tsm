use input::{run, TsmErrors};

mod input;
mod tmux;

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => match e {
            TsmErrors::NonUtf8Path => println!("Error: Command is not valid utf-8 string"),
            TsmErrors::FuzzyFindError(e) => println!("Error: Fuzzy finder failed {e}"),
            TsmErrors::CommandExecutionFailed(e) => println!("Error: Command failed {e}"),
            TsmErrors::OperationCancelled => println!("Operation cancelled"),
        },
    };
}
