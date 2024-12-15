use std::error::Error;
use std::fs;
use std::ops::{Add, AddAssign};
use std::path::PathBuf;
use utils::Loc;

type Pos = Loc<usize>;

impl AddAssign<Direction> for Pos {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Up => self.x -= 1,
            Direction::Down => self.x += 1,
            Direction::Left => self.y -= 1,
            Direction::Right => self.y += 1,
        }
    }
}

impl Add<Direction> for Pos {
    type Output = Pos;
    fn add(self, _rhs: Direction) -> Pos {
        match _rhs {
            Direction::Up => Pos {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Down => Pos {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Left => Pos {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Pos {
                x: self.x,
                y: self.y + 1,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from(ch: char) -> Self {
        match ch {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => {
                println!("Found {ch} instead of a direction");
                unreachable!();
            }
        }
    }
}

fn move_boxes(state: &mut Pos, dir: Direction, map: &mut [Vec<char>]) {
    let tail = match dir {
        Direction::Up => Pos {
            x: map[..state.x]
                .iter()
                .rposition(|r| r[state.y] == '.' || r[state.y] == '#')
                .unwrap(),
            y: state.y,
        },
        Direction::Down => Pos {
            x: map[state.x + 1..]
                .iter()
                .position(|r| r[state.y] == '.' || r[state.y] == '#')
                .map(|p| p + state.x + 1)
                .unwrap(),
            y: state.y,
        },
        Direction::Left => Pos {
            x: state.x,
            y: map[state.x][..state.y]
                .iter()
                .rposition(|&r| r == '.' || r == '#')
                .unwrap(),
        },
        Direction::Right => Pos {
            x: state.x,
            y: map[state.x][state.y + 1..]
                .iter()
                .position(|&r| r == '.' || r == '#')
                .map(|p| p + state.y + 1)
                .unwrap(),
        },
    };
    if map[tail.x][tail.y] == '#' {
        return;
    }
    map[state.x][state.y] = '.';
    *state += dir;
    map[state.x][state.y] = '@';
    map[tail.x][tail.y] = 'O';
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input15.txt");
    let input = fs::read_to_string(path)?;
    // Code
    // Parse input
    let mut map: Vec<Vec<char>> = vec![];
    // Start mutable because if not initialized could cause troubles
    let mut start: Option<Pos> = None;
    let input_iter = &mut input.lines();
    for (x, line) in input_iter.by_ref().enumerate() {
        if line.is_empty() {
            break;
        }
        map.push(line.chars().collect());
        for (y, ch) in line.chars().enumerate() {
            if start.is_none() && ch == '@' {
                start = Some(Pos { x, y });
            }
        }
    }
    //
    let mut state = start.unwrap();
    // Account for the starting position
    for line in input_iter {
        for ch in line.chars() {
            let dir = Direction::from(ch);

            let next_place = match dir {
                Direction::Up => map[state.x - 1][state.y],
                Direction::Down => map[state.x + 1][state.y],
                Direction::Left => map[state.x][state.y - 1],
                Direction::Right => map[state.x][state.y + 1],
            };
            let next_pos = state + dir;
            match next_place {
                '#' => continue,
                '.' => {
                    map[next_pos.x][next_pos.y] = '@';
                    map[state.x][state.y] = '.';
                    state = next_pos;
                }
                'O' => move_boxes(&mut state, dir, &mut map),
                _ => unreachable!(),
            }
            // for line in map.clone() {
            //     println!("{}", line.iter().collect::<String>());
            // }
            // println!();
        }
    }
    let mut gps_coordinates_sum = 0;
    for (x, line) in map.iter().enumerate() {
        for (y, ch) in line.iter().enumerate() {
            if *ch == 'O' {
                gps_coordinates_sum += 100 * x + y;
            }
        }
    }
    println!("{gps_coordinates_sum}");
    Ok(())
}
