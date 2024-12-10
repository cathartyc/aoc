use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    x: usize,
    y: usize,
}

fn count_new_stepped(
    dir: Direction,
    state: &mut State,
    rows: &HashMap<usize, Vec<usize>>,
    cols: &HashMap<usize, Vec<usize>>,
    bounds: &State,
) -> (HashSet<State>, bool) {
    let mut steps: HashSet<State> = HashSet::new();
    let mut quit = false;
    // code
    let next_obstacle_list = match dir {
        Direction::Up | Direction::Down => cols.get(&state.y),
        Direction::Left | Direction::Right => rows.get(&state.x),
    };
    let mut obs: Option<&usize> = None;
    if let Some(list) = next_obstacle_list {
        obs = match dir {
            Direction::Up => list.iter().rfind(|&&x| x < state.x),
            Direction::Down => list.iter().find(|&&x| x > state.x),
            Direction::Left => list.iter().rfind(|&&y| y < state.y),
            Direction::Right => list.iter().find(|&&y| y > state.y),
        };
    }
    let range = match obs {
        Some(obs_place) => match dir {
            Direction::Up => obs_place + 1..state.x,
            Direction::Down => state.x + 1..*obs_place,
            Direction::Left => obs_place + 1..state.y,
            Direction::Right => state.y + 1..*obs_place,
        },
        None => {
            quit = true;
            match dir {
                Direction::Up => 0..state.x,
                Direction::Down => state.x + 1..bounds.x + 1,
                Direction::Left => 0..state.y,
                Direction::Right => state.y + 1..bounds.y + 1,
            }
        }
    };
    for i in range {
        let next_step = match dir {
            Direction::Up | Direction::Down => State { x: i, y: state.y },
            Direction::Left | Direction::Right => State { x: state.x, y: i },
        };
        steps.insert(next_step);
    }
    *state = match obs {
        Some(obstacle) => match dir {
            Direction::Up => State {
                x: obstacle + 1,
                y: state.y,
            },
            Direction::Down => State {
                x: obstacle - 1,
                y: state.y,
            },
            Direction::Left => State {
                x: state.x,
                y: obstacle + 1,
            },
            Direction::Right => State {
                x: state.x,
                y: obstacle - 1,
            },
        },
        None => {
            // No obstacle, return the last position before going out of the map.
            quit = true;
            match dir {
                Direction::Up => State { x: 0, y: state.y },
                Direction::Down => State {
                    x: bounds.x,
                    y: state.y,
                },
                Direction::Left => State { x: state.x, y: 0 },
                Direction::Right => State {
                    x: state.x,
                    y: bounds.y,
                },
            }
        }
    };
    (steps, quit)
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
    let mut bounds = State { x: 0, y: 0 };
    // Start mutable because if not initialized could cause troubles
    let mut start: Option<State> = None;

    for (x, line) in input.lines().enumerate() {
        map.push(line.chars().collect());
        bounds.x += 1;
        if bounds.y == 0 {
            bounds.y = line.chars().count();
        }
        for (y, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    let obstacle = State { x, y };
                    obstacles.push(obstacle);
                    rows.entry(obstacle.x)
                        .and_modify(|vec| vec.push(obstacle.y))
                        .or_insert(vec![obstacle.y]);
                    cols.entry(obstacle.y)
                        .and_modify(|vec| vec.push(obstacle.x))
                        .or_insert(vec![obstacle.x]);
                }
                '^' => {
                    start = Some(State { x, y });
                    map[x][y] = 'X';
                }
                _ => continue,
            }
        }
    }
    bounds.x -= 1;
    bounds.y -= 1;
    let mut state = start.unwrap();
    let mut dir = Direction::Up;
    // Account for the starting position
    let mut total_steps: HashSet<State> = HashSet::new();
    total_steps.insert(start.unwrap());
    loop {
        let (steps, quit) = count_new_stepped(dir, &mut state, &rows, &cols, &bounds);
        total_steps.extend(steps);
        if quit {
            break;
        }
        dir = dir.rotate();
    }
    println!("{}", total_steps.len());
    Ok(())
}
