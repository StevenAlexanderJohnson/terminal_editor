use std::io::Write;

use crossterm::{
    cursor::{MoveTo, SetCursorStyle},
    execute,
    terminal::{Clear, ClearType},
};
pub struct TerminalCursor<W: Write> {
    content: Vec<String>,
    terminal: W,
    editing: bool,
    position: (u16, u16), // (x, y)
}

impl<W: Write> TerminalCursor<W> {
    pub fn new(stdout: W, content: Vec<String>) -> TerminalCursor<W> {
        TerminalCursor {
            content: content,
            terminal: stdout,
            editing: false,
            position: (0, 0),
        }
    }

    fn write_content(&mut self) {
        let initial_position = self.position;
        self.position = (0, 0);
        self.set_position(0, 0);
        let content = self.content.clone();
        execute!(self.terminal, Clear(ClearType::All)).unwrap();
        for line in &content {
            write!(self.terminal, "{}", line).unwrap();
            self.position.1 += 1;
            self.position.0 = 0;
            self.update_position();
        }

        self.position = initial_position;
        self.update_position();
    }

    pub fn initialize(&mut self) {
        self.write_content();
        self.set_position(0, 0);
    }

    pub fn is_editing(&self) -> bool {
        self.editing
    }

    pub fn move_up(&mut self, amount: u16) {
        if self.position.1 < amount {
            self.position.1 = 0;
        } else {
            self.position.1 -= amount;
        }
        self.update_position();
    }

    pub fn move_down(&mut self, amount: u16) {
        if self.position.1 + amount >= self.content.len() as u16 {
            self.position.1 = self.content.len() as u16 - 1;
        } else {
            self.position.1 += amount;
        }
        self.update_position();
    }

    pub fn move_left(&mut self, amount: u16) {
        if self.position.0 < amount {
            self.position.0 = 0;
        } else {
            self.position.0 -= amount;
        }
        self.update_position();
    }

    pub fn move_right(&mut self, amount: u16) {
        if self.position.0 + amount >= self.content[self.position.1 as usize].len() as u16 {
            self.position.0 = self.content[self.position.1 as usize].len() as u16 - 1;
        } else {
            self.position.0 += amount;
        }
        self.update_position();
    }

    pub fn write_char(&mut self, letter: char) {
        match letter {
            '\n' => {
                // Split the current line into two lines at the cursor
                let content = self.content.clone();
                let (left, right) =
                    content[self.position.1 as usize].split_at(self.position.0 as usize);
                self.content[self.position.1 as usize] = left.to_string();
                self.content
                    .insert(self.position.1 as usize + 1, right.to_string());
                self.position = (0, self.position.1 + 1);
            }
            _ => {
                self.content[self.position.1 as usize].insert(self.position.0 as usize, letter);
                self.position.0 += 1;
            }
        }
        self.write_content();
    }

    pub fn delete_char(&mut self) {
        // if the cursor is at the beginning of the line, join the current line with the previous line
        // unless it's the first line, then do nothing
        if self.position.0 == 0 {
            if self.position.1 == 0 {
                return;
            }
            let current_line = self.content.remove(self.position.1 as usize);
            self.position.1 -= 1;
            self.position.0 = self.content[self.position.1 as usize].len() as u16;
            self.content[self.position.1 as usize] += &current_line;
        } else {
            self.content[self.position.1 as usize].remove(self.position.0 as usize - 1);
            self.position.0 -= 1;
        }
        self.write_content();
    }

    pub fn set_position(&mut self, x: u16, y: u16) {
        self.position = (x, y);
        self.update_position();
    }

    pub fn set_editing(&mut self, editing: bool) {
        if editing {
            execute!(self.terminal, SetCursorStyle::SteadyUnderScore).unwrap();
        } else {
            execute!(self.terminal, SetCursorStyle::SteadyBlock).unwrap();
        }
        self.editing = editing;
    }

    pub fn update_position(&mut self) {
        match execute!(self.terminal, MoveTo(self.position.0, self.position.1)) {
            Ok(_) => (),
            Err(e) => eprintln!("Error moving cursor: {}", e),
        };
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    fn create_content() -> Vec<String> {
        vec![
            String::from("Hello, World!"),
            String::from("This is a Rust program!"),
            String::from("Third line of code!"),
            String::from("Fourth line of code!"),
            String::from("Fifth line of code!"),
            String::from("Sixth line of code!"),
        ]
    }

    #[test]
    fn test_move_up() {
        let content = create_content();
        let out = Cursor::new(vec![]);
        let mut cursor = super::TerminalCursor::new(out, content);
        cursor.initialize();
        cursor.move_up(1);
        assert_eq!(cursor.position, (0, 0));
    }

    #[test]
    fn test_move_left() {
        let content = create_content();
        let out = Cursor::new(vec![]);
        let mut cursor = super::TerminalCursor::new(out, content);
        cursor.initialize();
        cursor.move_left(1);
        assert_eq!(cursor.position, (0, 0));
    }

    #[test]
    fn test_move_right() {
        let content = create_content();
        let out = Cursor::new(vec![]);
        let mut cursor = super::TerminalCursor::new(out, content);
        cursor.initialize();
        cursor.move_right(1);
        assert_eq!(cursor.position, (1, 0));
    }

    #[test]
    fn test_move_down() {
        let content = create_content();
        let out = Cursor::new(vec![]);
        let mut cursor = super::TerminalCursor::new(out, content);
        cursor.initialize();
        cursor.move_down(1);
        assert_eq!(cursor.position, (0, 1));
    }
}
