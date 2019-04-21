extern crate regex;
extern crate termion;

pub mod cli;
pub mod input;

mod tcaps;
mod command;

#[cfg(test)]
mod tests {

    // cargo test -- --nocapture

    use super::cli::{Cli};

    #[derive(Copy, Clone)]
    pub enum Command {
        Exit,
    }

    #[test]
    fn it_works() {
        let mut app: Cli<Command> = super::cli::Cli::new();
        app.set_prompt("Cli Man $>");
        app.push_command(
            Command::Exit,
            "^exit$",
            String::from("Properly exit this command line")
        );
        loop {
            let e = match app.wait_input() {
                Ok(ev) => ev,
                Err(_) => return assert!(false),
            };

            match e.key {
                Command::Exit => return assert!(true),
            }
        }
    }
}

