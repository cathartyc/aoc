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

#[derive(Debug, Clone)]
struct State {
    loc: Loc,
    steps: Vec<Loc>,
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
        let mut new_state = self.clone();
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
        new_state.steps.push(new_state.loc);
        Ok(new_state)
    }
}

// Yeah, I know, it could be A*.
fn breadth_first_search(
    start: &Loc,
    goal: &Loc,
    bound: usize,
    map: &[Vec<char>],
) -> Result<State, &'static str> {
    let mut frontier: Vec<State> = vec![];
    let mut visited: HashSet<Loc> = HashSet::default();
    let mut current_state = State {
        loc: *start,
        steps: vec![*start],
        cost: 0,
    };
    loop {
        if current_state.loc == *goal {
            break;
        }
        [
            current_state.go(Direction::Up, bound, map),
            current_state.go(Direction::Down, bound, map),
            current_state.go(Direction::Left, bound, map),
            current_state.go(Direction::Right, bound, map),
        ]
        .into_iter()
        .filter_map(Result::ok)
        .filter(|ch: &State| !visited.contains(&ch.loc))
        .for_each(|ch| {
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
            None => return Err("No solution has been found"),
        }
    }
    Ok(current_state)
}

fn init_map(map: &mut [Vec<char>], bound: usize, input: &str, bytes: usize) {
    (0..=bound).for_each(|y| {
        map[y].clear();
        for _ in 0..=bound {
            map[y].push('.');
        }
    });
    for line in input.lines().take(bytes) {
        if let Some((x, y)) = line
            .split(",")
            .map(|d| d.parse::<usize>().unwrap())
            .collect_tuple()
        {
            map[y][x] = '#';
        }
    }
}

fn draw_map(map: &[Vec<char>], state: &State) {
    let mut map_to_print = map.to_vec();
    for step in state.steps.iter() {
        map_to_print[step.y][step.x] = 'O';
    }
    for line in map_to_print.iter() {
        for ch in line.iter() {
            print!("{ch}");
        }
        println!();
    }
    println!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input18.txt");
    let input = fs::read_to_string(path)?;
    // Bruteforce approach: draw a map
    let bound: usize = 70;
    let mut map: Vec<Vec<char>> = vec![];
    for _ in 0..=bound {
        map.push(vec![]);
    }

    // Define parameters
    let start = Loc { x: 0, y: 0 };
    let goal = Loc { x: bound, y: bound };

    // Bisection
    let mut good = 1024; // condition in previous input
    let mut bad = input.lines().count();
    while bad - good > 1 {
        let half: usize = (bad + good) / 2;
        init_map(&mut map, bound, &input, half);
        let result = breadth_first_search(&start, &goal, bound, &map);
        if result.is_ok() {
            good = half;
            // Print the path
            // draw_map(&map, &result.ok().unwrap());
        } else {
            bad = half;
        }
    }
    println!("{}", input.lines().nth(bad - 1).unwrap());
    Ok(())
}
