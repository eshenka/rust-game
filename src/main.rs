use crossterm::style;
use crossterm::style::Stylize;
use crossterm::terminal;
use crossterm::cursor;
use crossterm::{QueueableCommand, queue, ExecutableCommand, execute};
use crossterm::event::read;
use crossterm::event::{Event, KeyEvent};
use crossterm::event;
use std::io::{Write, stdout};
use std::io;

const UP_X_BOUND: u16 = 150;
const UP_Y_BOUND: u16 = 50;
const LOW_X_BOUND: u16 = 0;
const LOW_Y_BOUND: u16 = 0;

enum Direction {
    Right, 
    Left, 
    Up, 
    Down,
}

fn move_cursor(stdout: &mut io::Stdout, x: &mut u16, y: &mut u16, dir: Direction) {
    match dir {
        Direction::Right => {
            if *x + 1 < UP_X_BOUND {
                *x += 1;
                stdout.execute(cursor::MoveRight(1));
            }
        }

        Direction::Left => {
            if *x - 1 > LOW_X_BOUND {
                *x -= 1;
                stdout.execute(cursor::MoveLeft(1));
            } 
        }

        Direction::Up => {
            if *y - 1 > LOW_Y_BOUND {
                *y -= 1;
                stdout.execute(cursor::MoveUp(1));
            }
        }

        Direction::Down => {
            if *y + 1 < UP_Y_BOUND {
                *y += 1;
                stdout.execute(cursor::MoveDown(1));
            }
        }
    }
}

fn exiting(stdout: &mut io::Stdout) {
    terminal::disable_raw_mode();

    stdout.execute(terminal::LeaveAlternateScreen);
    stdout.execute(style::SetForegroundColor(style::Color::Reset));

    std::process::exit(0);
}

fn main() -> io::Result<()> {
    let mut stdout = stdout();

    execute!(stdout, terminal::EnterAlternateScreen).unwrap();

    terminal::enable_raw_mode();
    
    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();

    queue!(stdout, style::SetForegroundColor(style::Color::Red)).unwrap();

    stdout.flush().unwrap();

    for x in 0..150 {
        for y in 0..40 {
            if x == 0 || y == 0 || x == 149 || y == 39 {
                queue!(stdout, cursor::MoveTo(x, y), style::PrintStyledContent("0".blue())).unwrap();
            }
        }
    }

    let mut x_cursor = 60;
    let mut y_cursor = 10;

    queue!(stdout, cursor::MoveTo(x_cursor, y_cursor)).unwrap();

    stdout.flush().unwrap();

    loop {
        match read()? {
            Event::Key(event) => {
                match event {
                    event::KeyEvent{code, ..} => {
                        match code {
                            event::KeyCode::Left => {
                                move_cursor(&mut stdout, &mut x_cursor, &mut y_cursor, Direction::Left);
                            }

                            event::KeyCode::Right => {
                                move_cursor(&mut stdout, &mut x_cursor, &mut y_cursor, Direction::Right);
                            }

                            event::KeyCode::Up => {
                                move_cursor(&mut stdout, &mut x_cursor, &mut y_cursor, Direction::Up);
                            }
                            
                            event::KeyCode::Down => {
                                move_cursor(&mut stdout, &mut x_cursor, &mut y_cursor, Direction::Down);
                            }

                            event::KeyCode::Esc => {
                                exiting(&mut stdout);
                            }
                            _ => continue,
                        }
                    }

                    _ => exiting(&mut stdout),
                }
            }
            _ => exiting(&mut stdout),
        }
    }
}

