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

fn free_to_move(side_box: Pos, dir: Direction, map: &[Vec<char>]) -> bool {
    // Pos is always referred to the left side, '['.
    match dir {
        Direction::Up => {
            if map[side_box.x - 1][side_box.y] == '.' && map[side_box.x - 1][side_box.y + 1] == '.'
            {
                return true;
            }
            if map[side_box.x - 1][side_box.y] == '#' || map[side_box.x - 1][side_box.y + 1] == '#'
            {
                return false;
            }
            //
            match map[side_box.x - 1][side_box.y] {
                '[' => free_to_move(
                    Pos {
                        x: side_box.x - 1,
                        y: side_box.y,
                    },
                    dir,
                    map,
                ),
                ']' => {
                    free_to_move(
                        Pos {
                            x: side_box.x - 1,
                            y: side_box.y - 1,
                        },
                        dir,
                        map,
                    ) && (map[side_box.x - 1][side_box.y + 1] == '.'
                        || free_to_move(
                            Pos {
                                x: side_box.x - 1,
                                y: side_box.y + 1,
                            },
                            dir,
                            map,
                        ))
                }
                _ => {
                    // Only remaining option is that there is a '[' on the right side.
                    free_to_move(
                        Pos {
                            x: side_box.x - 1,
                            y: side_box.y + 1,
                        },
                        dir,
                        map,
                    )
                }
            }
        }
        Direction::Down => {
            if map[side_box.x + 1][side_box.y] == '.' && map[side_box.x + 1][side_box.y + 1] == '.'
            {
                return true;
            }
            if map[side_box.x + 1][side_box.y] == '#' || map[side_box.x + 1][side_box.y + 1] == '#'
            {
                return false;
            }
            //
            match map[side_box.x + 1][side_box.y] {
                '[' => free_to_move(
                    Pos {
                        x: side_box.x + 1,
                        y: side_box.y,
                    },
                    dir,
                    map,
                ),
                ']' => {
                    free_to_move(
                        Pos {
                            x: side_box.x + 1,
                            y: side_box.y - 1,
                        },
                        dir,
                        map,
                    ) && (map[side_box.x + 1][side_box.y + 1] == '.'
                        || free_to_move(
                            Pos {
                                x: side_box.x + 1,
                                y: side_box.y + 1,
                            },
                            dir,
                            map,
                        ))
                }
                _ => free_to_move(
                    Pos {
                        x: side_box.x + 1,
                        y: side_box.y + 1,
                    },
                    dir,
                    map,
                ),
            }
        }
        // Just check if there is a '.' after an eventual series of boxes.
        // In this case, the side is necessarily the right one.
        Direction::Left => {
            *map[side_box.x][..side_box.y - 1]
                .iter()
                .rfind(|&&p| p == '.' || p == '#')
                .unwrap()
                == '.'
        }
        Direction::Right => {
            *map[side_box.x][side_box.y + 2..]
                .iter()
                .find(|&&p| p == '.' || p == '#')
                .unwrap()
                == '.'
        }
    }
}

fn move_all_boxes(side_box: Pos, dir: Direction, map: &mut [Vec<char>]) {
    match dir {
        Direction::Up => {
            match map[side_box.x - 1][side_box.y] {
                '[' => move_all_boxes(
                    Pos {
                        x: side_box.x - 1,
                        y: side_box.y,
                    },
                    dir,
                    map,
                ),
                ']' => move_all_boxes(
                    Pos {
                        x: side_box.x - 1,
                        y: side_box.y - 1,
                    },
                    dir,
                    map,
                ),
                _ => (),
            }
            if map[side_box.x - 1][side_box.y + 1] == '[' {
                move_all_boxes(
                    Pos {
                        x: side_box.x - 1,
                        y: side_box.y + 1,
                    },
                    dir,
                    map,
                )
            }
            assert_ne!(map[side_box.x - 1][side_box.y], '#');
            assert_ne!(map[side_box.x - 1][side_box.y + 1], '#');
            map[side_box.x - 1][side_box.y] = '[';
            map[side_box.x - 1][side_box.y + 1] = ']';
            map[side_box.x][side_box.y] = '.';
            map[side_box.x][side_box.y + 1] = '.';
        }
        Direction::Down => {
            match map[side_box.x + 1][side_box.y] {
                '[' => move_all_boxes(
                    Pos {
                        x: side_box.x + 1,
                        y: side_box.y,
                    },
                    dir,
                    map,
                ),
                ']' => move_all_boxes(
                    Pos {
                        x: side_box.x + 1,
                        y: side_box.y - 1,
                    },
                    dir,
                    map,
                ),
                _ => (),
            }
            if map[side_box.x + 1][side_box.y + 1] == '[' {
                move_all_boxes(
                    Pos {
                        x: side_box.x + 1,
                        y: side_box.y + 1,
                    },
                    dir,
                    map,
                )
            }
            assert_ne!(map[side_box.x + 1][side_box.y], '#');
            assert_ne!(map[side_box.x + 1][side_box.y + 1], '#');
            map[side_box.x + 1][side_box.y] = '[';
            map[side_box.x + 1][side_box.y + 1] = ']';
            map[side_box.x][side_box.y] = '.';
            map[side_box.x][side_box.y + 1] = '.';
        }
        Direction::Left => {
            let tail = map[side_box.x][..side_box.y - 1]
                .iter()
                .rposition(|&p| p == '.')
                .unwrap();
            for i in tail..side_box.y {
                map[side_box.x][i] = map[side_box.x][i + 1];
            }
        }
        Direction::Right => {
            let tail = map[side_box.x][side_box.y + 2..]
                .iter()
                .position(|&p| p == '.')
                .unwrap();
            for i in (side_box.y + 1..=tail + side_box.y + 2).rev() {
                map[side_box.x][i] = map[side_box.x][i - 1];
            }
        }
    }
}

fn move_boxes(state: &mut Pos, dir: Direction, map: &mut [Vec<char>]) {
    // Need to check ALL following boxes for obstacles.
    // This can only happen for vertical movements.
    let side_box = match dir {
        Direction::Up => {
            if map[state.x - 1][state.y] == '[' {
                Pos {
                    x: state.x - 1,
                    y: state.y,
                }
            } else {
                Pos {
                    x: state.x - 1,
                    y: state.y - 1,
                }
            }
        }
        Direction::Down => {
            if map[state.x + 1][state.y] == '[' {
                Pos {
                    x: state.x + 1,
                    y: state.y,
                }
            } else {
                Pos {
                    x: state.x + 1,
                    y: state.y - 1,
                }
            }
        }
        Direction::Left => Pos {
            x: state.x,
            y: state.y - 1,
        },
        Direction::Right => Pos {
            x: state.x,
            y: state.y + 1,
        },
    };
    if !free_to_move(side_box, dir, map) {
        return;
    }
    move_all_boxes(side_box, dir, map);
    map[state.x][state.y] = '.';
    *state += dir;
    map[state.x][state.y] = '@';
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
        let mut chars: Vec<char> = vec![];
        for (y, ch) in line.chars().enumerate() {
            chars.extend(
                match ch {
                    '#' => "##",
                    'O' => "[]",
                    '.' => "..",
                    '@' => "@.",
                    _ => unreachable!(),
                }
                .chars(),
            );
            if start.is_none() && ch == '@' {
                start = Some(Pos { x, y: y * 2 });
            }
        }
        map.push(chars);
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
                '[' | ']' => move_boxes(&mut state, dir, &mut map),
                _ => unreachable!(),
            }
        }
    }
    let mut gps_coordinates_sum = 0;
    for (x, line) in map.iter().enumerate() {
        for (y, ch) in line.iter().enumerate() {
            if *ch == '[' {
                gps_coordinates_sum += 100 * x + y;
            }
        }
    }
    println!("{gps_coordinates_sum}");
    Ok(())
}
