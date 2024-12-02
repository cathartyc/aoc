use core::panic;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input2.txt");
    let input = fs::read_to_string(path)?;
    let mut trend = false;
    let mut safe_reports = 0;
    'outer: for line in input.lines() {
        let levels: Vec<i8> = line
            .split(' ')
            .map(|lev| lev.parse::<i8>().unwrap())
            .collect();
        for i in 0..levels.len() - 1 {
            let diff = levels[i + 1] - levels[i];
            if diff.abs() > 3 || diff == 0 {
                continue 'outer;
            }
            if i == 0 {
                trend = levels[i] < levels[i + 1];
            }
            if (diff > 0 && !trend) || (diff < 0 && trend) {
                continue 'outer;
            }
        }
        safe_reports += 1;
    }
    println!("{safe_reports}");
    Ok(())
}
