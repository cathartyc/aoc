use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn result_is_achievable(result: u64, operands: &Vec<u64>) -> bool {
    let mut partials: Vec<u64> = vec![];
    partials.push(operands[0]);
    for i in &operands[1..] {
        let l = partials.len();
        for _ in 0..l {
            let v = partials.remove(0);
            partials.push(v + i);
            partials.push(v * i);
        }
    }
    partials.contains(&result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input7.txt");
    let input = fs::read_to_string(path)?;
    // Code
    let mut sum_of_calibrations = 0;
    for line in input.lines() {
        let calibration: Vec<&str> = line.split(": ").collect();
        let result: u64 = calibration[0].parse()?;
        let operands: Vec<u64> = calibration[1]
            .split(' ')
            .map(|n| n.parse())
            .collect::<Result<Vec<u64>, _>>()?;
        if result_is_achievable(result, &operands) {
            sum_of_calibrations += result;
        }
    }
    println!("{sum_of_calibrations}");
    Ok(())
}
