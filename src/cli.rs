use super::command::Command;
use super::input::Input;
use super::tcaps;

pub struct Cli<T: Clone> {
    prompt: String,
    commands: Vec<Command<T>>,
    stdout: tcaps::Tcaps,
}

impl<T: Clone> Cli<T> {
    pub fn new() -> Cli<T> {
        let default_prompt = String::from("$> ");
        let app = Cli {
            commands: Vec::new(),
            stdout: tcaps::Tcaps::new(default_prompt.clone()),
            prompt: default_prompt,
        };
        return app;
    }

    pub fn set_prompt(&mut self, content: &str) {
        self.prompt = format!("{} ", content);
        self.stdout.prompt = self.prompt.clone();
    }

    pub fn push_command(&mut self, key: T, pattern: &str, help: String) {
        self.commands.push(Command::new(key, pattern, help));
    }

    pub fn wait_input(&mut self) -> Result<Input<T>, std::io::Error> {
        let mut input = self.stdout.read_line()?;
        match self.handle_input(&mut input) {
            Some(cmd) => return Ok(Input::new(cmd.key.clone(), input)),
            None => {
                self.stdout.println("Invalid command")?;
                return self.wait_input();
            }
        }
    }

    fn handle_input(&self, raw_input: &String) -> Option<(&Command<T>)> {
        let input = raw_input.trim();
        for cmd in self.commands.iter() {
            if cmd.pattern.is_match(input) {
                return Some(cmd);
            }
        }
        return None;
    }
}
