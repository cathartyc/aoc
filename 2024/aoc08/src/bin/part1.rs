use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Loc {
    x: isize,
    y: isize,
}

fn is_in_bound(val: &Loc, bound_x: usize, bound_y: usize) -> bool {
    val.x >= 0 && val.x <= bound_x as isize && val.y >= 0 && val.y <= bound_y as isize
}

fn find_antinodes(ant_1: &Loc, ant_2: &Loc, bound_x: usize, bound_y: usize) -> Vec<Loc> {
    let mut antinodes: Vec<Loc> = vec![];
    let dist_x = ant_2.x - ant_1.x;
    let dist_y = ant_2.y - ant_1.y;
    let antinode_1 = Loc {
        x: ant_1.x - dist_x,
        y: ant_1.y - dist_y,
    };
    if is_in_bound(&antinode_1, bound_x, bound_y) {
        antinodes.push(antinode_1);
    }
    let antinode_2 = Loc {
        x: ant_2.x + dist_x,
        y: ant_2.y + dist_y,
    };
    if is_in_bound(&antinode_2, bound_x, bound_y) {
        antinodes.push(antinode_2);
    }
    antinodes
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input8.txt");
    let input = fs::read_to_string(path)?;
    // Code
    let mut antennas: HashMap<char, Vec<Loc>> = HashMap::new();
    let mut antinodes: HashSet<Loc> = HashSet::new();
    let mut size_x: usize = 0;
    let mut size_y: usize = 0;
    for (x, line) in input.lines().enumerate() {
        size_x += 1;
        if size_y == 0 {
            size_y = line.chars().count();
        }
        for (y, ch) in line.chars().enumerate() {
            match ch {
                '.' => continue,
                _ => antennas
                    .entry(ch)
                    .and_modify(|vec| {
                        vec.push(Loc {
                            x: x as isize,
                            y: y as isize,
                        })
                    })
                    .or_insert(vec![Loc {
                        x: x as isize,
                        y: y as isize,
                    }]),
            };
        }
    }
    // make comparisons less error-prone
    size_x -= 1;
    size_y -= 1;
    for antennas in antennas.values() {
        for (i, ant_1) in antennas.iter().enumerate() {
            for ant_2 in &antennas[i + 1..] {
                antinodes.extend(find_antinodes(ant_1, ant_2, size_x, size_y));
            }
        }
    }
    println!("{}", antinodes.len());
    Ok(())
}
