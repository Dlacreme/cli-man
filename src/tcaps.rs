use termion::raw::{RawTerminal, IntoRawMode};
use termion::event::Key;
use termion::input::TermRead;
use std::io::{Write, stdout, stdin};

pub struct Tcaps {
    x: u16,
    y: u16,
    pub prompt: String,
    stdout: RawTerminal<std::io::Stdout>,
}

impl Tcaps {

    pub fn new(prompt: String) -> Tcaps {
        let mut tcp = Tcaps {
            prompt: prompt,
            y: 1,
            x: 1,
            stdout: stdout().into_raw_mode().unwrap()
        };
        tcp.clear().unwrap();
        tcp.set_cursor(tcp.x, tcp.y).unwrap();
        return tcp;
    }

    pub fn read_line(&mut self) -> Result<String, std::io::Error> {
        self.set_cursor(1, self.y)?;
        self.print_prompt()?;
        let mut buffer: String = String::from("");
        for raw_key in stdin().lock().keys() {
            let k = raw_key?;
            match k {
                    Key::Char('\n') => {
                    self.y += 1;
                    return Ok(buffer);
                },
                Key::Char(c)    => buffer.push(c),
                Key::Alt(c)     => self.handle_alt(c)?,
                Key::Ctrl(c)    => self.handle_ctrl(c)?,
                Key::Backspace  => {
                    buffer.pop();
                }
                Key::Left       => println!("<left>"),
                Key::Right      => println!("<right>"),
                Key::Up         => println!("<up>"),
                Key::Down       => println!("<down>"),
                _ => (),
            }
            write!(self.stdout, "{}{}", termion::cursor::Goto(self.x, self.y), termion::clear::CurrentLine).unwrap();
            self.print_with_prompt(buffer.as_str())?;
        }
        unreachable!();
    }

    /* Handlers */

    pub fn handle_alt(&mut self, c: char) -> std::io::Result<()> {
        match c {
            _ => {}
        }
        Ok(())
    }

    pub fn handle_ctrl(&mut self, c: char) -> std::io::Result<()> {
        match c {
            'l' => self.clear()?,
            _   => {},
        }
        Ok(())
    }

    /* TERM Actions */

    pub fn clear(&mut self) -> std::io::Result<()> {
        self.y = 1;
        write!(self.stdout, "{}", termion::clear::All)?;
        Ok(())
    }

    pub fn set_cursor(&mut self, x: u16, y: u16) -> std::io::Result<()> {
        write!(self.stdout, "{}", termion::cursor::Goto(x, y))
    }

    /* PRINTER */
    pub fn print_focus(&mut self, content: &str) -> std::io::Result<()> {
        self.clear()?;
        self.println("(Press any key to quit)\n")?;
        self.println("")?;
        self.println(content)?;
        self.set_cursor(1, 1)?;
        for _ in stdin().lock().keys() {
            self.clear()?;
            return Ok(());
        }
        unreachable!();
    }

    pub fn print_prompt(&mut self) -> std::io::Result<()> {
        write!(self.stdout, "{}", self.prompt.as_str())?;
        self.stdout.flush().unwrap();
        Ok(())
    }

    pub fn print(&mut self, content: &str) -> std::io::Result<()> {
        // self.y += final_content.matches("\n").count() as u16;
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
        self.set_cursor(1, self.y)?;
        self.y += 1;
        write!(self.stdout, "{}\n", content)?;
        self.stdout.flush()?;
        Ok(())
    }

}
