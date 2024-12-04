use std::error::Error;
use std::fs;
use std::ops::Add;
use std::path::PathBuf;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    DiagUr,
    DiagUl,
    DiagDr,
    DiagDl,
}

#[derive(Clone, Copy)]
struct XY {
    x: isize,
    y: isize,
}

impl Add for XY {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Direction {
    fn incr(&self) -> XY {
        match self {
            Direction::Up => XY { x: -1, y: 0 },
            Direction::Down => XY { x: 1, y: 0 },
            Direction::Left => XY { x: 0, y: -1 },
            Direction::Right => XY { x: 0, y: 1 },
            Direction::DiagUr => XY { x: -1, y: 1 },
            Direction::DiagUl => XY { x: -1, y: -1 },
            Direction::DiagDr => XY { x: 1, y: 1 },
            Direction::DiagDl => XY { x: 1, y: -1 },
        }
    }
}

fn find_word(word_matrix: &[Vec<char>], start: XY, dir: Direction) -> bool {
    let mut position = start;
    let direction = dir.incr();
    let chars = ['M', 'A', 'S'];
    for c in chars.iter() {
        position = position + direction;
        if position.x < 0
            || position.y < 0
            || position.x >= word_matrix[0].len().try_into().unwrap()
            || position.y >= word_matrix.len().try_into().unwrap()
            || &word_matrix[position.y as usize][position.x as usize] != c
        {
            return false;
        }
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input4.txt");
    let input = fs::read_to_string(path)?;
    // Code
    let mut word_matrix: Vec<&str> = vec![];
    let mut word_count: u32 = 0;
    for line in input.lines() {
        word_matrix.push(line);
    }
    let word_matrix: Vec<Vec<char>> = word_matrix.iter().map(|s| s.chars().collect()).collect();
    for (j, line) in word_matrix.iter().enumerate() {
        for (i, c) in line.iter().enumerate() {
            if *c == 'X' {
                for direction in Direction::iter() {
                    if find_word(
                        &word_matrix,
                        XY {
                            x: i as isize,
                            y: j as isize,
                        },
                        direction,
                    ) {
                        word_count += 1;
                    }
                }
            }
        }
    }
    println!("{word_count}");
    Ok(())
}
