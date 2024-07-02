use std::io::Write;

use crossterm::{
    cursor::{MoveTo, SetCursorStyle},
    execute,
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
        self.position.0 = 0;
        self.position.1 = 0;
        let content = self.content.clone();
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

    pub fn write_text(&mut self, text: &str) {
        for c in text.chars() {
            self.content[self.position.1 as usize].insert(self.position.0 as usize, c);
            self.position.0 += 1 as u16;
        }
        let x_position = self.position.0;
    
        self.position.0 = 0;
        let content = self.content.clone();
        for c in content[self.position.1 as usize].chars() {
            write!(self.terminal, "{}", c).unwrap();
            self.position.0 += 1 as u16;
            self.update_position();
        }
    
        self.position.0 = x_position;
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

    pub fn next_line(&mut self) {
        self.position.0 = 0;
        self.content.insert(self.position.1 as usize, String::from(""));
        self.position.1 += 1;
        self.update_position();
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

    #[test]
    fn test_write_text() {
        let content = create_content();
        let out = Cursor::new(vec![]);
        let mut cursor = super::TerminalCursor::new(out, content);
        cursor.initialize();
        cursor.write_text("a");
        assert_eq!(cursor.content[0], "aHello, World!");
    }

    #[test]
    fn test_write_text_new_line() {
        let content = create_content();
        let out = Cursor::new(vec![]);
        let mut cursor = super::TerminalCursor::new(out, content);
        cursor.initialize();
        cursor.write_text("a\n");
        assert_eq!(cursor.content[0], "a");
        assert_eq!(cursor.content[1], "Hello, World!");
    }

    #[test]
    fn test_write_text_new_line_middle() {
        let content = create_content();
        let out = Cursor::new(vec![]);
        let mut cursor = super::TerminalCursor::new(out, content);
        cursor.initialize();
        cursor.write_text("This is a simple text\nwith a new line");
        assert_eq!(cursor.content[1], "This is a simple text");
        assert_eq!(cursor.content[2], "with a new lineHello, World!");
    }
}
