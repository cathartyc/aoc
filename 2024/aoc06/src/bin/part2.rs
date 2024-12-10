use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
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

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
struct State {
    x: usize,
    y: usize,
}

/// Finds the last position the guard will reach in its straight-lined movement,
/// which is the next obstacle she will reach, "minus one".
fn next_destination(
    state: &State,
    dir: &Direction,
    rows: &HashMap<usize, Vec<usize>>,
    cols: &HashMap<usize, Vec<usize>>,
    bounds: &State,
) -> (State, bool) {
    let mut quit = false;
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
    let destination: State = match obs {
        // This is the "minus one" I meant...
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
    (destination, quit)
}

/// Check if a path with an obstruction results in a loop, by temporarily
/// adding the obstruction to the list of obstacles.
fn is_loop(
    start: &State,
    rows: &HashMap<usize, Vec<usize>>,
    cols: &HashMap<usize, Vec<usize>>,
    bounds: &State,
    obstruction: &State,
) -> bool {
    let mut visited: HashSet<(State, Direction)> = HashSet::new();
    let mut state = *start;
    let mut rows = rows.clone();
    let mut cols = cols.clone();
    let mut dir = Direction::Up;
    rows.entry(obstruction.x)
        .and_modify(|vec| {
            vec.push(obstruction.y);
            vec.sort();
        })
        .or_insert(vec![obstruction.y]);
    cols.entry(obstruction.y)
        .and_modify(|vec| {
            vec.push(obstruction.x);
            vec.sort();
        })
        .or_insert(vec![obstruction.x]);
    loop {
        let (destination, quit) = next_destination(&state, &dir, &rows, &cols, bounds);
        if quit {
            return false;
        }
        if visited.contains(&(destination, dir)) {
            return true;
        }
        visited.insert((destination, dir));
        state = destination;
        dir = dir.rotate();
    }
}

/// Finds possible obstructions, avoiding to evaluate loops whenever
/// - going to the side would lead the guard out of the map;
/// - the obstruction has already been added to the list.
fn find_obstructions(
    state: &mut State,
    start: &State,
    dir: &Direction,
    bounds: &State,
    rows: &HashMap<usize, Vec<usize>>,
    cols: &HashMap<usize, Vec<usize>>,
) -> (HashSet<State>, bool) {
    let mut obstructions: HashSet<State> = HashSet::new();
    let (destination, quit) = next_destination(state, dir, rows, cols, bounds);
    let range = match dir {
        Direction::Up => destination.x + 1..=state.x,
        Direction::Down => state.x..=destination.x - 1,
        Direction::Left => destination.y + 1..=state.y,
        Direction::Right => state.y..=destination.y - 1,
    };
    for i in range {
        let next_position = match dir {
            Direction::Up | Direction::Down => State { x: i, y: state.y },
            Direction::Left | Direction::Right => State { x: state.x, y: i },
        };
        let obstruction = match dir {
            Direction::Up => State {
                x: next_position.x - 1,
                y: next_position.y,
            },
            Direction::Down => State {
                x: next_position.x + 1,
                y: next_position.y,
            },
            Direction::Left => State {
                x: next_position.x,
                y: next_position.y - 1,
            },
            Direction::Right => State {
                x: next_position.x,
                y: next_position.y + 1,
            },
        };
        if obstructions.contains(&obstruction) {
            continue;
        }
        let (_, no_obstacle) = next_destination(&next_position, &dir.rotate(), rows, cols, bounds);
        if !no_obstacle
            && is_loop(
                // A lot of time lost here because I was starting near the obstacle.
                // This is wrong because the obstruction could impact the previous path too.
                start,
                rows,
                cols,
                bounds,
                &obstruction,
            )
        {
            obstructions.insert(obstruction);
        }
    }
    *state = destination;
    (obstructions, quit)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input6.txt");
    let input = fs::read_to_string(path)?;
    // Code
    let mut rows: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut cols: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut bounds = State { x: 0, y: 0 };
    // Start mutable because if not initialized could cause troubles
    let mut start: Option<State> = None;
    for (i, line) in input.lines().enumerate() {
        if bounds.y == 0 {
            bounds.y = line.chars().count();
        }
        bounds.x += 1;
        for (j, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    rows.entry(i)
                        .and_modify(|vec| vec.push(j))
                        .or_insert(vec![j]);
                    cols.entry(j)
                        .and_modify(|vec| vec.push(i))
                        .or_insert(vec![i]);
                }
                '^' => {
                    start = Some(State { x: i, y: j });
                }
                _ => continue,
            }
        }
    }
    bounds.x -= 1;
    bounds.y -= 1;
    let mut state = start.unwrap();
    let mut dir = Direction::Up;

    let mut total_obstructions: HashSet<State> = HashSet::new();
    loop {
        let (obstructions, quit) =
            find_obstructions(&mut state, &start.unwrap(), &dir, &bounds, &rows, &cols);
        total_obstructions.extend(obstructions);
        if quit {
            break;
        }
        dir = dir.rotate();
    }
    // Remove guard's starting position if present
    total_obstructions.remove(&start.unwrap());
    println!("{}", total_obstructions.len());
    Ok(())
}
