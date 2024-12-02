use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn check_safety(levels: &Vec<i8>, tolerate: bool) -> bool {
    let mut trend: bool = false;
    for i in 0..levels.len() - 1 {
        let diff = levels[i + 1] - levels[i];
        if diff.abs() > 3 || diff == 0 {
            if !tolerate {
                return false;
            } else {
                let mut levels_a = levels.clone();
                levels_a.remove(i);
                let mut levels_b = levels.clone();
                levels_b.remove(i + 1);
                if check_safety(&levels_a, false) {
                    println!("{levels:?} is safe by removing {}", levels[i]);
                    return true
                } else if check_safety(&levels_b, false) {
                    println!("{levels:?} is safe by removing {}", levels[i + 1]);
                    return true
                } else {
                    return false;
                }
            }
        }
        if i == 0 {
            trend = levels[i] < levels[i + 1];
        } else if (diff > 0 && !trend) || (diff < 0 && trend) {
            if !tolerate {
                return false;
            } else {
                let mut levels_a = levels.clone();
                levels_a.remove(i);
                let mut levels_b = levels.clone();
                levels_b.remove(i + 1);
                let mut levels_c = levels.clone();
                levels_c.remove(i - 1);
                if check_safety(&levels_a, false) {
                    println!("{levels:?} is safe by removing {}", levels[i]);
                    return true
                } else if check_safety(&levels_b, false) {
                    println!("{levels:?} is safe by removing {}", levels[i + 1]);
                    return true
                } else if check_safety(&levels_c, false) {
                    println!("{levels:?} is safe by removing {}", levels[i - 1]);
                    return true
                } else {
                    return false;
                }
            }
        }
    }
    if tolerate {
        println!("{levels:?} is safe");
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input2.txt");
    let input = fs::read_to_string(path)?;
    let mut safe_reports = 0;
    for line in input.lines() {
        let levels: Vec<i8> = line
            .split(' ')
            .map(|lev| lev.parse::<i8>().unwrap())
            .collect();
        if check_safety(&levels, true) {
            safe_reports += 1;
        } else {
            println!("{levels:?} is not safe");
        }
    }
    println!("{safe_reports}");
    Ok(())
}
