use crossterm::cursor;
use crossterm::event;
use crossterm::event::read;
use crossterm::event::Event;
use crossterm::event::KeyEvent;
use crossterm::style;
use crossterm::style::Stylize;
use crossterm::terminal;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::{execute, queue, ExecutableCommand};
use std::io;
use std::io::{stdout, Write};

const UP_X_BOUND: u16 = 150 - 1;
const UP_Y_BOUND: u16 = 50 - 1;
const LOW_X_BOUND: u16 = 0;
const LOW_Y_BOUND: u16 = 0;

const MAZE_SIZE: u16 = 15;

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(PartialEq)]
enum Maze {
    Wall,
    Path,
}

struct Point {
    x: u16,
    y: u16,
    maze: Maze,
}

fn move_cursor(
    stdout: &mut io::Stdout,
    x: &mut u16,
    y: &mut u16,
    dir: Direction,
    maze: &Vec<Point>,
) -> std::io::Result<()> {
    match dir {
        Direction::Right => {
            if (*x + 1 < UP_X_BOUND)
                && maze[((*y - 11) * 25 + (*x - 61 + 1)) as usize].maze == Maze::Path
            {
                *x += 1;
                stdout.execute(cursor::MoveRight(1))?;
            }
        }

        Direction::Left => {
            if *x - 1 > LOW_X_BOUND {
                *x -= 1;
                stdout.execute(cursor::MoveLeft(1))?;
            }
        }

        Direction::Up => {
            if *y - 1 > LOW_Y_BOUND {
                *y -= 1;
                stdout.execute(cursor::MoveUp(1))?;
            }
        }

        Direction::Down => {
            if *y + 1 < UP_Y_BOUND {
                *y += 1;
                stdout.execute(cursor::MoveDown(1))?;
            }
        }
    }

    Ok(())
}

fn exiting(stdout: &mut io::Stdout) -> std::io::Result<()> {
    terminal::disable_raw_mode()?;

    stdout.execute(terminal::LeaveAlternateScreen)?;
    stdout.execute(style::SetForegroundColor(style::Color::Reset))?;

    Ok(())
}

fn main() -> io::Result<()> {
    std::panic::set_hook(Box::new(|info| {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);

        println!("thread {info}");
    }));

    let mut stdout = stdout();

    execute!(stdout, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    queue!(stdout, style::SetBackgroundColor(style::Color::Blue))?;

    for x in LOW_X_BOUND..=UP_X_BOUND {
        for y in LOW_Y_BOUND..=UP_Y_BOUND {
            if x == LOW_X_BOUND || y == LOW_Y_BOUND || x == UP_X_BOUND || y == UP_Y_BOUND {
                queue!(
                    stdout,
                    cursor::MoveTo(x, y),
                    style::PrintStyledContent("0".blue())
                )?;
            }
        }
    }

    let mut x_cursor = 60;
    let mut y_cursor = 10;

    stdout.flush()?;

    let mut maze: Vec<Point> = Vec::with_capacity(MAZE_SIZE as usize * MAZE_SIZE as usize);

    let enter = Point {
        x: 0,
        y: 1,
        maze: Maze::Path,
    };

    let exit = Point {
        x: 24,
        y: 7,
        maze: Maze::Path,
    };

    for x in 0..25 {
        for y in 0..9 {
            if (x == 0 || x == 24 || y == 0 || y == 8)
                && !(x == enter.x && y == enter.y)
                && !(x == exit.x && y == exit.y)
            {
                maze.push(Point {
                    x,
                    y,
                    maze: Maze::Wall,
                });

                queue!(
                    stdout,
                    cursor::MoveTo(60 + x, 10 + y),
                    style::PrintStyledContent("0".blue())
                )?;
            } else {
                maze.push(Point {
                    x,
                    y,
                    maze: Maze::Path,
                });
            }
        }
    }

    execute!(stdout, cursor::MoveTo(x_cursor, y_cursor + 1))?;

    loop {
        match read()? {
            Event::Key(KeyEvent { code, .. }) => match code {
                event::KeyCode::Left => {
                    move_cursor(
                        &mut stdout,
                        &mut x_cursor,
                        &mut y_cursor,
                        Direction::Left,
                        &maze,
                    )?;
                }

                event::KeyCode::Right => {
                    move_cursor(
                        &mut stdout,
                        &mut x_cursor,
                        &mut y_cursor,
                        Direction::Right,
                        &maze,
                    )?;
                }

                event::KeyCode::Up => {
                    move_cursor(
                        &mut stdout,
                        &mut x_cursor,
                        &mut y_cursor,
                        Direction::Up,
                        &maze,
                    )?;
                }

                event::KeyCode::Down => {
                    move_cursor(
                        &mut stdout,
                        &mut x_cursor,
                        &mut y_cursor,
                        Direction::Down,
                        &maze,
                    )?;
                }

                event::KeyCode::Esc => break,

                _ => continue,
            },

            _ => break,
        }
    }

    exiting(&mut stdout)
}
