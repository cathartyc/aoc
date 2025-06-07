use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    fn turn_cw(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn turn_ccw(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Loc {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Hash)]
struct State {
    loc: Loc,
    orientation: Direction,
    cost: u32,
}

impl State {
    fn go_forward(&mut self) {
        match self.orientation {
            Direction::Up => self.loc.x -= 1,
            Direction::Down => self.loc.x += 1,
            Direction::Left => self.loc.y -= 1,
            Direction::Right => self.loc.y += 1,
        }
        self.cost += 1;
    }
    fn go_backward(&mut self) {
        match self.orientation {
            Direction::Up => self.loc.x += 1,
            Direction::Down => self.loc.x -= 1,
            Direction::Left => self.loc.y += 1,
            Direction::Right => self.loc.y -= 1,
        }
        self.cost -= 1;
    }
}

/**
Get all the possible next states in a straight line.
A next state can be:
- the solution;
- any block in front of the reindeer where it can turn (without obstacles).
*/
fn get_next_states(map: &[Vec<char>], state: &State, location: Loc) -> Vec<State> {
    let mut states: Vec<State> = vec![];
    let mut temp_state = *state;
    loop {
        // Get valid turns
        let mut temp_state_left = temp_state;
        temp_state_left.orientation = temp_state_left.orientation.turn_ccw();
        temp_state_left.go_forward();
        if map[temp_state_left.loc.x][temp_state_left.loc.y] != '#' {
            temp_state_left.go_backward();
            temp_state_left.cost += 1000;
            states.push(temp_state_left);
        }
        let mut temp_state_right = temp_state;
        temp_state_right.orientation = temp_state_right.orientation.turn_cw();
        temp_state_right.go_forward();
        if map[temp_state_right.loc.x][temp_state_right.loc.y] != '#' {
            temp_state_right.go_backward();
            temp_state_right.cost += 1000;
            states.push(temp_state_right);
        }
        // Go forward
        temp_state.go_forward();
        if map[temp_state.loc.x][temp_state.loc.y] == '#' {
            break;
        } else if temp_state.loc == location {
            states.push(temp_state);
            break;
        }
    }
    states
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input16.txt");
    let input = fs::read_to_string(path)?;
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = State {
        loc: Loc {
            x: map.len() - 2,
            y: 1,
        },
        orientation: Direction::Right,
        cost: 0,
    };
    let target = Loc {
        x: 1,
        y: map[0].len() - 2,
    };
    let mut frontier: Vec<State> = Vec::default();
    let mut visited: Vec<State> = Vec::default();
    let mut current = start;
    loop {
        // check if this is a solution
        if current.loc == target {
            break;
        }
        visited.push(current);
        let next_states: Vec<State> = get_next_states(&map, &current, target);
        // Remove duplicates or worse results
        for st in next_states {
            match frontier
                .iter()
                .position(|f: &State| st.loc == f.loc && st.orientation == f.orientation)
            {
                Some(x) => {
                    if frontier[x].cost > st.cost {
                        frontier.remove(x);
                        frontier.push(st);
                    }
                }
                None => {
                    match visited
                        .iter()
                        .position(|v: &State| st.loc == v.loc && st.orientation == v.orientation)
                    {
                        Some(_) => (),
                        None => frontier.push(st),
                    }
                }
            }
        }
        frontier.sort_by_key(|s: &State| s.cost);
        frontier.reverse();
        match frontier.pop() {
            Some(next) => current = next,
            None => return Err("There is no solution".into()),
        }
        // Printing
        /*
        for (x, line) in map.iter().enumerate() {
            for (y, ch) in line.iter().enumerate() {
                if current.loc.x == x && current.loc.y == y {
                    print!("@");
                } else {
                    print!("{ch}");
                }
            }
            println!();
        }
        println!();
        */
        // end printing
    }
    println!("{}", current.cost);
    Ok(())
}
