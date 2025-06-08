use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Loc {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct State {
    loc: Loc,
    cost: u16,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl State {
    fn go(
        &self,
        direction: Direction,
        bound: usize,
        map: &[Vec<char>],
    ) -> Result<Self, &'static str> {
        let mut new_state = *self;
        let mut underflow = false;
        match direction {
            Direction::Up => (new_state.loc.y, underflow) = new_state.loc.y.overflowing_sub(1),
            Direction::Down => new_state.loc.y += 1,
            Direction::Left => (new_state.loc.x, underflow) = new_state.loc.x.overflowing_sub(1),
            Direction::Right => new_state.loc.x += 1,
        }
        if underflow
            || new_state.loc.x > bound
            || new_state.loc.y > bound
            || map[new_state.loc.y][new_state.loc.x] == '#'
        {
            return Err("Cannot go there.");
        }
        new_state.cost += 1;
        Ok(new_state)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input18.txt");
    let input = fs::read_to_string(path)?;
    // Bruteforce approach: draw a map
    let bound: usize = 70;
    let size = 1024;
    let mut map: Vec<Vec<char>> = vec![];
    for y in 0..=bound {
        map.push(vec![]);
        for _ in 0..=bound {
            map[y].push('.');
        }
    }
    for line in input.lines().take(size) {
        if let Some((x, y)) = line
            .split(",")
            .map(|d| d.parse::<usize>().unwrap())
            .collect_tuple()
        {
            map[y][x] = '#';
        }
    }
    // Define parameters
    let start = Loc { x: 0, y: 0 };
    let goal = Loc { x: bound, y: bound };
    // solve a optimal path problem
    let mut current_state = State {
        loc: start,
        cost: 0,
    };
    let mut frontier: Vec<State> = vec![];
    let mut visited: HashSet<Loc> = HashSet::default();
    loop {
        if current_state.loc == goal {
            break;
        }
        [
            current_state.go(Direction::Up, bound, &map),
            current_state.go(Direction::Down, bound, &map),
            current_state.go(Direction::Left, bound, &map),
            current_state.go(Direction::Right, bound, &map),
        ]
        .iter_mut()
        .filter_map(|ch| ch.ok())
        .filter(|ch: &State| !visited.contains(&ch.loc))
        .for_each(|ch| {
            //print!("{ch:?},");
            if let Some(i) = frontier.iter().position(|f| f.loc == ch.loc) {
                if frontier[i].cost > ch.cost {
                    frontier[i] = ch;
                }
            } else {
                frontier.push(ch);
            }
        });
        frontier.sort_by_key(|ch| ch.cost);
        frontier.reverse();
        visited.insert(current_state.loc);
        match frontier.pop() {
            Some(new_state) => current_state = new_state,
            None => return Err("No solution has been found".into()),
        }
    }
    println!("{}", current_state.cost);
    Ok(())
}
