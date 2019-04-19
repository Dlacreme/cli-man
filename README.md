# CLI-MAN

Small library to create a basic CLI program.

# Usage

```rust

/// Create an enum which will contains your action
/// It must derive of Clone
#[derive(Copy, Clone)]
pub enum Command {
    Exit,
}

fn main() {

    // Init the application
    let mut app: Cli<Command> = Cli::new();
    // You can override the default prompt ('$>');
    app.set_prompt("Hello World >");

    // Push as many command as you want
    app.push_command(
        Command::Exit, // Set the enum matchin your command
        "^exit$", // Use a regex pattern to valid the input
        String::from("Properly exit this command line") // Error displayed if the arguments are not valid
    );

    // Listen for incoming input
    loop {
        let e = match app.listen() {
            Ok(ev) => ev,
            Err(_) => exit(-42), // something went wrong
        };

        match e.key {
            Command::Exit => return exit(42), // All good here
        }
    }

}

```
