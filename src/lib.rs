pub mod cli;
pub mod command;
pub mod input;

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
        app.set_prompt("HELLO");
        app.push_command(
            Command::Exit,
            "^exit$",
            String::from("Properly exit this command line")
        );
        loop {
            let e = match app.listen() {
                Ok(ev) => ev,
                Err(_) => return assert!(false),
            };

            match e.key {
                Command::Exit => return assert!(true),
            }
        }
    }
}

