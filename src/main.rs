mod terminal;
use std::{
    io,
    time::{Duration, Instant}, vec,
};

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
struct Debouncer {
    last_call: Instant,
    delay: Duration,
}

impl Debouncer {
    fn new(delay: Duration) -> Self {
        Debouncer {
            last_call: Instant::now() - delay,
            delay,
        }
    }

    fn should_call(&mut self) -> bool {
        let now = Instant::now();
        if now - self.last_call > self.delay {
            self.last_call = now;
            true
        } else {
            false
        }
    }
}

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let initial_content: Vec<String> = vec![
        String::from("Hello, world!"),
        String::from("This is a Rust program!"),
        String::from("Third line of code!"),
        String::from("Fourth line of code!"),
        String::from("Fifth line of code!"),
        String::from("Sixth line of code!"),
    ];

    let mut cursor = terminal::TerminalCursor::new(&mut stdout, initial_content);
    cursor.initialize();

    let mut debouncer = Debouncer::new(Duration::from_millis(20));
    let mut running = true;

    while running {
        while event::poll(std::time::Duration::from_millis(20)).unwrap() {
            if cursor.is_editing() {
                if let Event::Key(event) = event::read()? {
                    if !debouncer.should_call() {
                        continue;
                    }
                    match event.code {
                        event::KeyCode::Esc => {
                            cursor.set_editing(false);
                        }
                        event::KeyCode::Backspace => {
                            cursor.delete_char();
                        }
                        event::KeyCode::Enter => {
                            cursor.write_char('\n');
                        }
                        event::KeyCode::Char(c) => {
                            cursor.write_char(c);
                        }
                        _ => (),
                    }
                }
            } else {
                if let Event::Key(event) = event::read()? {
                    if !debouncer.should_call() {
                        continue;
                    }
                    match event.code {
                        event::KeyCode::Char('q') => {
                            running = false;
                        }
                        event::KeyCode::Char('k') => cursor.move_up(1),
                        event::KeyCode::Char('h') => cursor.move_left(1),
                        event::KeyCode::Char('j') => cursor.move_down(1),
                        event::KeyCode::Char('l') => cursor.move_right(1),
                        event::KeyCode::Char('i') => cursor.set_editing(true),
                        _ => (),
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(std::io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
