use input::run;

mod input;
mod tmux;

fn main() {
    if let Err(e) = run() {
        println!("{e}");
    }
}
