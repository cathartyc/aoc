use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use utils::Loc;

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input14.txt");
    let input = fs::read_to_string(path)?;
    // Code
    let bounds: Loc<i32> = Loc {
        // Given from puzzle description
        x: 100,
        y: 102,
    };
    let elapsed_time: i32 = 100;
    let mut final_positions: Vec<Loc<i32>> = vec![];
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let px: i32 = caps[1].parse().unwrap();
        let py: i32 = caps[2].parse().unwrap();
        let vx: i32 = caps[3].parse().unwrap();
        let vy: i32 = caps[4].parse().unwrap();
        final_positions.push(Loc {
            x: (px + vx * elapsed_time).rem_euclid(bounds.x + 1),
            y: (py + vy * elapsed_time).rem_euclid(bounds.y + 1),
        });
    }
    let mut quadrants = [0; 4];
    let half_map = Loc {
        x: bounds.x / 2,
        y: bounds.y / 2,
    };
    for pos in final_positions {
        if pos.x < half_map.x && pos.y < half_map.y {
            quadrants[0] += 1;
        } else if pos.x < half_map.x && pos.y > half_map.y {
            quadrants[1] += 1;
        } else if pos.x > half_map.x && pos.y > half_map.y {
            quadrants[2] += 1;
        } else if pos.x > half_map.x && pos.y < half_map.y {
            quadrants[3] += 1;
        }
    }
    println!("{}", quadrants.iter().product::<i32>());
    // Return Ok
    Ok(())
}
