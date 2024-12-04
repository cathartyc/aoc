use std::error::Error;
use std::fs;
use std::path::PathBuf;

/// Removes an element from the given array.
fn exclude_level(vec: &[i8], index: usize) -> Vec<i8> {
    vec[..index]
        .iter()
        .chain(&vec[index + 1..])
        .cloned()
        .collect()
}

/// Checks if the given report is safe.
///
/// If a report is unsafe, applies the Problem dampener on the three
/// levels adjacent to the first bad event, that consists in
/// checking the same report after having removed one of those levels
/// at a time.
///
/// The recursion is tracked by the tolerate parameter, which is false in
/// a recursive call.
fn check_safety(report: &[i8], tolerate: bool) -> bool {
    let mut trend: bool = false;
    for i in 0..report.len() - 1 {
        let diff = report[i + 1] - report[i];
        if i == 0 {
            trend = report[i] < report[i + 1];
        }
        if diff.abs() > 3 || diff == 0 || (diff > 0 && !trend) || (diff < 0 && trend) {
            if !tolerate {
                return false;
            } else {
                return check_safety(&exclude_level(report, i), false)
                    || (i < report.len() - 1
                        && check_safety(&exclude_level(report, i + 1), false))
                    || (i > 0 && check_safety(&exclude_level(report, i - 1), false));
            }
        }
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
            .map(|lev| lev.parse())
            .collect::<Result<Vec<i8>, _>>()?;
        if check_safety(&levels, true) {
            safe_reports += 1;
        }
    }
    println!("{safe_reports}");
    Ok(())
}
