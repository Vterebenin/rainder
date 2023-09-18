use std::io::{stdout, Write};
use rand::Rng;
mod user_input;
use user_input::user_input;

use crossterm::cursor::{Hide, Show, MoveTo};
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
    terminal,
    queue,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: Option<u16>, 
    y: Option<u16>,
    symbol: char,
    prev_x: Option<u16>,
    prev_y: Option<u16>,
}

const MAX_RAINDROPS: u16 = 30;
const MIN_RAINDROPS: u16 = 15;

pub fn generate_new_raindrops(points: &mut Vec<Point>, width: u16, height: u16) -> Vec<Point> {
    let raindrops_counts = rand::thread_rng().gen_range(MIN_RAINDROPS..MAX_RAINDROPS + 1);
    for i in 0..raindrops_counts {
        let x_pos = rand::thread_rng().gen_range(0..width);
        let y_pos = rand::thread_rng().gen_range(0..height);
        let symbol = if i % 3 != 0 {
            '\\'
        } else {
            '.'
        };
        let point = Point {
            x: Some(x_pos),
            y: Some(0),
            symbol,
            prev_x: None,
            prev_y: None,
        };
        points.push(point);
        if i % 5 == 0 {
            let point = Point {
                x: Some(0),
                y: Some(y_pos),
                symbol,
                prev_x: None,
                prev_y: None,
            };
            points.push(point);
        }
    }

    points
        .iter()
        .filter(|p| p.x.unwrap() != width && p.y.unwrap() != height)
        .map(|p| *p)
        .collect::<Vec<Point>>()
        .to_vec()
}

pub fn clear() -> std::io::Result<()> {
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    Ok(())
}

pub fn draw_and_update(points: &mut Vec<Point>) -> std::io::Result<()>  {
    let mut stdout = stdout();
    for point in points.iter_mut() {
        if let (Some(prev_x), Some(prev_y)) = (point.prev_x, point.prev_y) {
            queue!(stdout, MoveTo(prev_x, prev_y))?;
            queue!(stdout, Print(" "))?;
        }
        if let (Some(x), Some(y)) = (point.x, point.y) {
            queue!(stdout, MoveTo(x, y))?;
            queue!(stdout, SetForegroundColor(Color::White))?;
            queue!(stdout, Print(point.symbol))?;
            queue!(stdout, ResetColor)?;
            let (width, height) = terminal::size()?;
            point.prev_x = point.x;
            point.prev_y = point.y;
            if x + 1 >= width || y + 1 >= height {
                queue!(stdout, MoveTo(x, y))?;
                queue!(stdout, Print(" "))?;
                point.x = Some(width);
                point.y = Some(height);
            } else {
                point.x = Some(x + 1);
                point.y = Some(y + 1);
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    // using the macro
    let mut stdout = stdout();
    let mut is_running = true;
    terminal::enable_raw_mode()?;
    stdout.execute(Hide)?;
    let (width, height) = terminal::size()?;
    let mut points = vec![];
    clear()?;
    while is_running {
        points = generate_new_raindrops(&mut points, width, height);
        is_running = user_input()?;
        draw_and_update(&mut points)?;
        stdout.flush()?;
    }
    stdout.execute(Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
