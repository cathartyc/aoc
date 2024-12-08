use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
struct State {
    x: usize,
    y: usize,
}

fn count_new_stepped(
    dir: Direction,
    state: &mut State,
    rows: &HashMap<usize, Vec<usize>>,
    cols: &HashMap<usize, Vec<usize>>,
    map: &mut [Vec<char>],
) -> (usize, bool) {
    let mut steps = 0;
    // code
    match dir {
        Direction::Up => {
            let pos_x = cols.get(&state.y).unwrap().iter().rfind(|&&x| x < state.x);
            match pos_x {
                Some(x) => {
                    for i in *x + 1..state.x {
                        if map[i][state.y] == '.' {
                            map[i][state.y] = 'X';
                            steps += 1;
                        }
                    }
                    state.x = *x + 1;
                    (steps, true)
                }
                None => {
                    for i in 0..state.x {
                        if map[i][state.y] == '.' {
                            steps += 1;
                        }
                    }
                    (steps, false)
                }
            }
        }
        Direction::Right => {
            let pos_y = rows.get(&state.x).unwrap().iter().find(|&&y| y > state.y);
            match pos_y {
                Some(y) => {
                    for i in state.y + 1..*y {
                        if map[state.x][i] == '.' {
                            map[state.x][i] = 'X';
                            steps += 1;
                        }
                    }
                    state.y = *y - 1;
                    (steps, true)
                }
                None => {
                    for i in state.y + 1..map[0].len() {
                        if map[state.x][i] == '.' {
                            steps += 1;
                        }
                    }
                    (steps, false)
                }
            }
        }
        Direction::Down => {
            let pos_x = cols.get(&state.y).unwrap().iter().find(|&&x| x > state.x);
            match pos_x {
                Some(x) => {
                    for i in state.x + 1..*x {
                        if map[i][state.y] == '.' {
                            map[i][state.y] = 'X';
                            steps += 1;
                        }
                    }
                    state.x = *x - 1;
                    (steps, true)
                }
                None => {
                    for i in state.x + 1..map.len() {
                        if map[i][state.y] == '.' {
                            steps += 1;
                        }
                    }
                    (steps, false)
                }
            }
        }
        Direction::Left => {
            let pos_y = rows.get(&state.x).unwrap().iter().rfind(|&&y| y < state.y);
            match pos_y {
                Some(y) => {
                    for i in *y + 1..state.y {
                        if map[state.x][i] == '.' {
                            map[state.x][i] = 'X';
                            steps += 1;
                        }
                    }
                    state.y = *y + 1;
                    (steps, true)
                }
                None => {
                    for i in 0..state.y {
                        if map[state.x][i] == '.' {
                            steps += 1;
                        }
                    }
                    (steps, false)
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input6.txt");
    let input = fs::read_to_string(path)?;
    // Code
    // Parse input
    let mut map: Vec<Vec<char>> = vec![];
    let mut obstacles: Vec<State> = vec![];
    let mut rows: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut cols: HashMap<usize, Vec<usize>> = HashMap::new();
    // Start mutable because if not initialized could cause troubles
    let mut start: Option<State> = None;

    for (i, line) in input.lines().enumerate() {
        map.push(line.chars().collect());
        for (j, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    let obstacle = State { x: i, y: j };
                    obstacles.push(obstacle);
                    rows.entry(obstacle.x)
                        .and_modify(|vec| vec.push(obstacle.y))
                        .or_insert(vec![obstacle.y]);
                    cols.entry(obstacle.y)
                        .and_modify(|vec| vec.push(obstacle.x))
                        .or_insert(vec![obstacle.x]);
                }
                '^' => {
                    start = Some(State { x: i, y: j });
                    map[i][j] = 'X';
                }
                _ => continue,
            }
        }
    }
    let mut state = start.unwrap();
    let mut dirs = Direction::iter();
    // Account for the starting position
    let mut total_positions: usize = 1;
    loop {
        let dir = match dirs.next() {
            Some(val) => val,
            None => {
                dirs = Direction::iter();
                dirs.next().unwrap()
            }
        };
        let (steps, inside_bounds) = count_new_stepped(dir, &mut state, &rows, &cols, &mut map);
        total_positions += steps;
        if !inside_bounds {
            break;
        }
    }
    println!("{total_positions}");
    Ok(())
}
