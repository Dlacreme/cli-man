use std::io::{Write, stdout};

use super::command::{Command};
use super::input::{Input};

pub struct Cli<T: Clone> {
    prompt: String,
    commands: Vec<Command<T>>,
}

impl<T: Clone> Cli<T> {

    pub fn new() -> Cli<T> {
        Cli {
            prompt: String::from("$> "),
            commands: Vec::new(),
        }
    }

    pub fn set_prompt(&mut self, content: &str) {
        self.prompt = format!("{} ", content);
    }

    pub fn push_command(&mut self, key: T, pattern: &str, help: String) {
        self.commands.push(Command::new(
            key,
            pattern,
            help,
        ));
    }

    pub fn listen(&self) -> Result<Input<T>, std::io::Error> {
        loop {
            print!("{}", self.prompt);
            stdout().flush().unwrap();
            let mut buffer = String::new();
            match std::io::stdin().read_line(&mut buffer) {
                Ok(_) => {
                    match self.handle_input(&buffer) {
                        Some(cmd) => return Ok(Input::new(cmd.key.clone(), buffer)),
                        None => { }
                    }
                }, Err(e) => {
                    return Err(e);
                }
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