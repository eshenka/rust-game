use crossterm::style;
use crossterm::style::Stylize;
use crossterm::terminal;
use crossterm::cursor;
use crossterm::{QueueableCommand, queue, ExecutableCommand, execute};
use std::io::{Write, stdout};

fn main() {
    let mut stdout = stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();

    execute!(stdout, terminal::EnterAlternateScreen).unwrap();
    queue!(stdout, style::SetBackgroundColor(style::Color::Blue), style::SetForegroundColor(style::Color::Red));

    stdout.flush().unwrap();

    for x in 0..150 {
        for y in 0..40 {
            if x == 0 || y == 0 || x == 149 || y == 39 {
                queue!(stdout, cursor::MoveTo(x, y), style::PrintStyledContent("0".blue())).unwrap();
            }
        }
    }

    queue!(stdout, cursor::MoveTo(60, 10));

    stdout.flush().unwrap();
}

