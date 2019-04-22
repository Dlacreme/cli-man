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

    /**
     * update the text displayed before the user input
     */
    pub fn set_prompt(&mut self, content: &str) {
        self.prompt = format!("{} ", content);
        self.stdout.prompt = self.prompt.clone();
    }

    /**
     * Insert a new command
     * An event will be raised every time user input matches $pattern
     */
    pub fn push_command(&mut self, key: T, pattern: &str, help: String) {
        self.commands.push(Command::new(key, pattern, help));
    }

    /**
     * Will read user input and wait for an existing command (previously inserted via
     * `push_command`
     */
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

    /**
     * reset_term should be called just before the cli ends
     */
    pub fn reset_term(&mut self) -> std::io::Result<()> {
        self.stdout.print("\n")
    }

    /** OUTPUT
     */

    /**
     * Clear the terminal
     */
    pub fn clear(&mut self) -> std::io::Result<()> {
        self.stdout.clear()
    }

    /**
     * Display content
     */
    pub fn print(&mut self, content: &str) -> std::io::Result<()> {
        self.stdout.print(content)
    }

    /**
     * Display content and insert new like markup (\n)
     */
    pub fn println(&mut self, content: &str) -> std::io::Result<()> {
        self.stdout.println(content)
    }

    /**
     * Print $content on full screen mode
     */
    pub fn print_focus_mode(&mut self, content: &str) -> std::io::Result<()> {
        self.stdout.print_focus(content)
    }


    /**
     * Private
     */
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
