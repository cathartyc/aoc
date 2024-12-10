use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Loc {
    x: usize,
    y: usize,
}

#[derive(EnumIter, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match *self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

/// Check if the next state is inside the bounds of the map.
fn in_bounds(point: &Loc, dir: Direction, bounds: &Loc) -> bool {
    let (dx, dy) = dir.delta();
    let new_x = point.x as isize + dx;
    let new_y = point.y as isize + dy;
    new_x >= 0 && new_y >= 0 && new_x <= bounds.x as isize && new_y <= bounds.y as isize
}

/// Finds a valid path from the start to any score in the map.
fn find_paths(start: &Loc, map: &[Vec<u8>], bounds: &Loc) -> usize {
    let mut frontier: HashSet<Loc> = HashSet::new();
    let mut scores: HashSet<Loc> = HashSet::new();
    frontier.insert(*start);
    loop {
        if frontier.is_empty() {
            break;
        }
        // Thanks to Rust for having such a clear way to pop an element
        // from a set :(
        let state = *frontier.iter().next().unwrap();
        frontier.remove(&state);

        if map[state.x][state.y] == 9 {
            scores.insert(state);
            continue;
        }
        // Add valid neighbors
        for dir in Direction::iter() {
            if in_bounds(&state, dir, bounds) {
                let (dx, dy) = dir.delta();
                let new_x = (state.x as isize + dx) as usize;
                let new_y = (state.y as isize + dy) as usize;
                if map[new_x][new_y] == map[state.x][state.y] + 1 {
                    frontier.insert(Loc {
                        x: (state.x as isize + dx) as usize,
                        y: (state.y as isize + dy) as usize,
                    });
                }
            }
        }
    }
    // Interested in unique trailhead-score paths
    scores.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input10.txt");
    let input = fs::read_to_string(path)?;
    // Code
    let mut map: Vec<Vec<u8>> = vec![];
    let mut trailheads: Vec<Loc> = vec![];
    let mut bounds = Loc { x: 0, y: 0 };
    for (x, line) in input.lines().enumerate() {
        bounds.x += 1;
        if bounds.y == 0 {
            bounds.y = line.chars().count();
        }
        map.push(vec![]);
        for (y, ch) in line.chars().enumerate() {
            let val = ch.to_digit(10).unwrap() as u8;
            if val == 0 {
                trailheads.push(Loc { x, y })
            }
            map[x].push(val);
        }
    }
    // Adjust bounds
    bounds.x -= 1;
    bounds.y -= 1;
    // For each trailhead, find a path to as much scores as possible.
    // The text says "a good trail is as long as possible", but each path
    // will always be 10 digits long, including the start.
    let mut trails = 0;
    for start in trailheads {
        trails += find_paths(&start, &map, &bounds);
    }
    println!("{trails}");
    Ok(())
}
