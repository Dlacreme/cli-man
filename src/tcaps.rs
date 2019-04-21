use termion::raw::{RawTerminal, IntoRawMode};
use termion::event::Key;
use termion::input::TermRead;
use std::io::{Write, stdout, stdin};

pub struct Tcaps {
    line_index: u16,
    pub prompt: String,
    stdout: RawTerminal<std::io::Stdout>,
}

impl Tcaps {

    pub fn new(prompt: String) -> Tcaps {
        let mut tcp = Tcaps {
            prompt: prompt,
            line_index: 1,
            stdout: stdout().into_raw_mode().unwrap()
        };
        tcp.clear().unwrap();
        tcp.set_cursor(1, 1).unwrap();
        return tcp;
    }

    pub fn read_line(&mut self) -> Result<String, std::io::Error> {
        self.set_cursor(1, self.line_index)?;
        self.print_prompt()?;
        let mut buffer: String = String::from("");
        for raw_key in stdin().lock().keys() {
            let k = raw_key?;
            match k {
                Key::Char('\n') => {

                    self.line_index += 1;
                    return Ok(buffer);
                },
                Key::Char(c) => buffer.push(c),
                _ => (),
            }
            write!(self.stdout, "{}{}", termion::cursor::Goto(1, self.line_index), termion::clear::CurrentLine).unwrap();
            self.print_with_prompt(buffer.as_str())?;
        }
        unreachable!();
    }

    /* TERM Actions */

    pub fn clear(&mut self) -> std::io::Result<()> {
        self.line_index = 1;
        write!(self.stdout, "{}", termion::clear::All)?;
        Ok(())
    }

    pub fn set_cursor(&mut self, x: u16, y: u16) -> std::io::Result<()> {
        write!(self.stdout, "{}", termion::cursor::Goto(x, y))
    }

    /* PRINTER */

    pub fn print_prompt(&mut self) -> std::io::Result<()> {
        write!(self.stdout, "{}", self.prompt.as_str())?;
        self.stdout.flush().unwrap();
        Ok(())
    }

    pub fn print(&mut self, content: &str) -> std::io::Result<()> {
        write!(self.stdout, "{}", content)?;
        self.stdout.flush().unwrap();
        Ok(())
    }

    pub fn print_with_prompt(&mut self, content: &str) -> std::io::Result<()> {
        write!(self.stdout, "{}{}", self.prompt, content)?;
        self.stdout.flush().unwrap();
        Ok(())
    }

    pub fn println(&mut self, content: &str) -> std::io::Result<()> {
        self.set_cursor(1, self.line_index);
        self.line_index += 1;
        write!(self.stdout, "{}\n", content)?;
        self.stdout.flush().unwrap();
        Ok(())
    }

}
