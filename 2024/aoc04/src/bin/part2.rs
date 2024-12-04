use std::error::Error;
use std::fs;
use std::ops::Add;
use std::path::PathBuf;
use strum_macros::EnumIter;

#[derive(EnumIter)]
enum Direction {
    Ur,
    Ul,
    Dr,
    Dl,
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
            Direction::Ur => XY { x: 1, y: -1 },
            Direction::Ul => XY { x: -1, y: -1 },
            Direction::Dr => XY { x: 1, y: 1 },
            Direction::Dl => XY { x: -1, y: 1 },
        }
    }
}

fn find_word(word_matrix: &[Vec<char>], start: XY, dir: Direction) -> bool {
    let mut position = start;
    let direction = dir.incr();
    let chars = ['A', 'S'];
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

fn find_cross(word_matrix: &[Vec<char>], start: XY) -> u32 {
    let mut results = 0;
    if start.x + 2 < word_matrix[0].len().try_into().unwrap()
        && word_matrix[start.y as usize][start.x as usize + 2] == 'M'
    {
        let second = XY {
            x: start.x + 2,
            y: start.y,
        };
        if find_word(word_matrix, start, Direction::Dr)
            && find_word(word_matrix, second, Direction::Dl)
        {
            results += 1;
        }
        if find_word(word_matrix, start, Direction::Ur)
            && find_word(word_matrix, second, Direction::Ul)
        {
            results += 1;
        }
    }
    if start.y + 2 < word_matrix.len().try_into().unwrap()
        && word_matrix[start.y as usize + 2][start.x as usize] == 'M'
    {
        let second = XY {
            x: start.x,
            y: start.y + 2,
        };
        if find_word(word_matrix, start, Direction::Dr)
            && find_word(word_matrix, second, Direction::Ur)
        {
            results += 1;
        }
        if find_word(word_matrix, start, Direction::Dl)
            && find_word(word_matrix, second, Direction::Ul)
        {
            results += 1;
        }
    }
    results
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
    for (y, line) in word_matrix.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'M' {
                word_count += find_cross(
                    &word_matrix,
                    XY {
                        x: x as isize,
                        y: y as isize,
                    },
                )
            }
        }
    }
    println!("{word_count}");
    Ok(())
}
