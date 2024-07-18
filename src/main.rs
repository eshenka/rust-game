use crossterm::style;
use crossterm::terminal;
use crossterm::{QueueableCommand, queue, execute};
use std::io::{Write, stdout};

fn main() {
    let mut stdout = stdout();

    execute!(stdout, terminal::EnterAlternateScreen).unwrap();
    queue!(stdout, style::SetBackgroundColor(style::Color::Blue));

    stdout.flush().unwrap();
}
