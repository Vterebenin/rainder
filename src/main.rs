use std::io::{stdout, Write};
use rand::Rng;
mod user_input;
use user_input::user_input;

use crossterm::cursor::{Hide, Show, MoveTo};
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
    terminal,
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
    x: u16, 
    y: u16,
    symbol: char,
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
            x: x_pos,
            y: 0,
            symbol,
        };
        points.push(point);
        if i % 5 == 0 {
            let point = Point {
                x: 0,
                y: y_pos,
                symbol,
            };
            points.push(point);
        }
    }
    points
        .iter()
        .filter(|p| p.x + 1 <= width && p.y + 1 <= height)
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
        let Point { x, y, symbol } = point;
        stdout.execute(MoveTo(*x, *y))?;
        stdout
            .execute(SetForegroundColor(Color::White))?
            .execute(Print(symbol))?
            .execute(ResetColor)?;
        point.x += 1;
        point.y += 1;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    // using the macro
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    let mut is_running = true;
    stdout.execute(Hide)?;
    let (width, height) = terminal::size()?;
    let mut points = vec![];
    while is_running {
        is_running = user_input()?;
        points = generate_new_raindrops(&mut points, width, height);
        clear()?;
        draw_and_update(&mut points)?;
        stdout.flush()?;
    }
    stdout.execute(Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
