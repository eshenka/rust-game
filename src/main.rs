#![allow(dead_code)]

use crossterm::cursor;
use crossterm::event;
use crossterm::event::read;
use crossterm::event::Event;
use crossterm::event::KeyEvent;
use crossterm::style;
use crossterm::style::Stylize;
use crossterm::terminal;
use crossterm::terminal::ClearType;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::{execute, queue, ExecutableCommand};

use std::io;
use std::io::{stdout, Write};

use rand::seq::SliceRandom;

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

#[derive(PartialEq, Debug)]
enum Maze {
    Wall,
    Path,
}

struct Point {
    x: u16,
    y: u16,
    maze: Maze,
}

struct Cell {
    connections: (bool, bool, bool, bool),
    visited: bool,
}

fn move_cursor(
    stdout: &mut io::Stdout,
    x_cursor: &mut u16,
    y_cursor: &mut u16,
    dir: Direction,
    maze: &Vec<Vec<Point>>,
) -> std::io::Result<()> {
    let x = (*x_cursor - 60) as usize;
    let y = (*y_cursor - 10) as usize;

    match dir {
        Direction::Right => {
            if x + 1 >= 25 {
                return Ok(());
            }

            if maze[y][x + 1].maze == Maze::Path {
                *x_cursor += 1;
                stdout.execute(cursor::MoveRight(1))?;
            }
        }

        Direction::Left => {
            if x - 1 <= 0 {
                return Ok(());
            }

            if maze[y][x - 1].maze == Maze::Path {
                *x_cursor -= 1;
                stdout.execute(cursor::MoveLeft(1))?;
            }
        }

        Direction::Up => {
            if y - 1 <= 0 {
                return Ok(());
            }

            if maze[y - 1][x].maze == Maze::Path {
                *y_cursor -= 1;
                stdout.execute(cursor::MoveUp(1))?;
            }
        }

        Direction::Down => {
            if y + 1 >= 9 {
                return Ok(());
            }

            if maze[y + 1][x].maze == Maze::Path {
                *y_cursor += 1;
                stdout.execute(cursor::MoveDown(1))?;
            }
        }
    }

    Ok(())
}

fn exiting(stdout: &mut io::Stdout, grid: &Vec<Vec<Cell>>) -> std::io::Result<()> {
    terminal::disable_raw_mode()?;

    execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        style::SetForegroundColor(style::Color::Reset)
    )?;

    for y in 0..4 {
        for x in 0..12 {
            println!("{}", grid[y][x].visited);
        }
        println!("####################");
    }

    Ok(())
}

fn get_heighbors((y, x): (usize, usize)) -> Vec<(usize, usize)> {
    let mut cells: Vec<(usize, usize)> = Vec::new();

    if x != 0 {
        cells.push((y, x - 1));
    }

    if y != 0 {
        cells.push((y - 1, x));
    }

    if x != 11 {
        cells.push((y, x + 1));
    }

    if y != 3 {
        cells.push((y + 1, 3));
    }

    cells
}

fn randomize(grid: &mut Vec<Vec<Cell>>, (y, x): (usize, usize)) {
    grid[y][x].visited = true;

    let neighbor_cells = {
        let mut cells = get_heighbors((y, x));
        cells.shuffle(&mut rand::thread_rng());
        cells
    };

    for (next_y, next_x) in neighbor_cells {
        if grid[next_y][next_x].visited {
            continue;
        }

        if x > next_x {
            grid[y][x].connections.0 = true;
            grid[y][next_x].connections.2 = true;
        } else if x < next_x {
            grid[y][x].connections.2 = true;
            grid[y][next_x].connections.0 = true;
        }

        if y > next_y {
            grid[y][x].connections.1 = true;
            grid[next_y][x].connections.3 = true;
        } else if y < next_y {
            grid[y][x].connections.3 = true;
            grid[next_y][x].connections.1 = true;
        }

        randomize(grid, (next_y, next_x))
    }
}

fn main() -> io::Result<()> {
    std::panic::set_hook(Box::new(|info| {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);

        println!("thread {info}");
    }));

    let mut stdout = stdout();

    terminal::enable_raw_mode()?;

    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        terminal::Clear(ClearType::All),
        style::SetBackgroundColor(style::Color::Blue)
    )?;

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

    let mut maze: Vec<Vec<Point>> = Vec::with_capacity(25 * 9);

    let enter = Point {
        x: 0,
        y: 1,
        maze: Maze::Path,
    };

    let exit = Point {
        x: 23,
        y: 7,
        maze: Maze::Path,
    };

    for y in 0..9 {
        let mut string: Vec<Point> = Vec::new();
        for x in 0..25 {
            if (x == 0 || x == 24 || y == 0 || y == 8)
                && !(x == enter.x && y == enter.y)
                && !(x == exit.x && y == exit.y)
            {
                string.push(Point {
                    x,
                    y,
                    maze: Maze::Wall,
                });

                queue!(
                    stdout,
                    cursor::MoveTo(x_cursor + x, y_cursor + y),
                    style::PrintStyledContent("0".blue())
                )?;
            } else {
                string.push(Point {
                    x,
                    y,
                    maze: Maze::Path,
                });
            }
        }
        maze.push(string);
    }

    let mut grid: Vec<Vec<Cell>> = Vec::new();

    for _ in 0..4 {
        let mut string: Vec<Cell> = Vec::new();

        for _ in 0..12 {
            string.push(Cell {
                connections: (false, false, false, false),
                visited: false,
            });
        }

        grid.push(string);
    }

    randomize(&mut grid, (0, 0));

    for y in 0..4 as usize {
        for x in 0..12 as usize {
            let x_global: u16 = (x * 2 + 1) as u16;
            let y_global: u16 = (y * 2 + 1) as u16;

            if !grid[y][x].connections.0 {
                maze[y_global as usize][x_global as usize - 1] = Point {
                    x: x_global - 1,
                    y: y_global,
                    maze: Maze::Wall,
                };

                queue!(
                    stdout,
                    cursor::MoveTo(x_cursor + x_global - 1, y_cursor + y_global),
                    style::PrintStyledContent("0".blue())
                )?;
            }

            if !grid[y][x].connections.1 {
                maze[y_global as usize - 1][x_global as usize] = Point {
                    x: x_global,
                    y: y_global - 1,
                    maze: Maze::Wall,
                };

                queue!(
                    stdout,
                    cursor::MoveTo(x_cursor + x_global, y_cursor + y_global - 1),
                    style::PrintStyledContent("0".blue())
                )?;
            }

            if !grid[y][x].connections.2 {
                maze[y_global as usize][x_global as usize + 1] = Point {
                    x: x_global + 1,
                    y: y_global,
                    maze: Maze::Wall,
                };

                queue!(
                    stdout,
                    cursor::MoveTo(x_cursor + x_global + 1, y_cursor + y_global),
                    style::PrintStyledContent("0".blue())
                )?;
            }

            if !grid[y][x].connections.3 {
                maze[y_global as usize + 1][x_global as usize] = Point {
                    x: x_global,
                    y: y_global + 1,
                    maze: Maze::Wall,
                };

                queue!(
                    stdout,
                    cursor::MoveTo(x_cursor + x_global, y_cursor + y_global + 1),
                    style::PrintStyledContent("0".blue())
                )?;
            }
        }
    }

    stdout.flush()?;

    x_cursor += 1;
    y_cursor += 1;
    execute!(stdout, cursor::MoveTo(x_cursor, y_cursor))?;

    loop {
        if x_cursor == exit.x + 60 && y_cursor == exit.y + 10 {
            println!("Hhhh");
            break;
        }

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

                event::KeyCode::Enter => {
                    println!("{x_cursor} {y_cursor}");
                }

                event::KeyCode::Esc => break,

                _ => continue,
            },

            _ => break,
        }
    }

    exiting(&mut stdout, &grid)
}
