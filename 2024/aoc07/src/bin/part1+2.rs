use itertools::{repeat_n, Itertools};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn generate_dispositions(len: usize, easy: bool) -> Vec<Vec<char>> {
    let operators = if easy {
        vec!['+', '*']
    } else {
        vec!['+', '*', '|']
    };
    let mut result: Vec<Vec<char>> = repeat_n(operators.iter().cloned(), len)
        .multi_cartesian_product()
        .collect();
    if !easy {
        result.retain(|vec| vec.contains(&'|'));
    }
    result
}

fn result_is_achievable(result: u64, operands: &[u64], operations: &Vec<Vec<char>>) -> bool {
    let ops_list = operations;
    for ops in ops_list {
        let mut partial = operands[0];
        for (i, op) in ops.iter().enumerate() {
            match op {
                '+' => partial += operands[i + 1],
                '*' => partial *= operands[i + 1],
                '|' => {
                    partial =
                            // Concatenation
                            partial * 10u64.pow(operands[i + 1].ilog10() + 1) + operands[i + 1]
                }
                _ => unreachable!(),
            }
            if partial > result {
                break;
            }
        }
        if partial == result {
            return true;
        }
    }
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input7.txt");
    let input = fs::read_to_string(path)?;
    // Code
    let mut ez_operations: HashMap<usize, Vec<Vec<char>>> = HashMap::new();
    let mut complex_operations: HashMap<usize, Vec<Vec<char>>> = HashMap::new();
    let mut sum_of_calibrations = 0;
    let mut calibrations: Vec<(u64, Vec<u64>)> = vec![];
    for line in input.lines() {
        let calibration: Vec<&str> = line.split(": ").collect();
        let result: u64 = calibration[0].parse()?;
        let operands: Vec<u64> = calibration[1]
            .split(' ')
            .map(|n| n.parse())
            .collect::<Result<Vec<u64>, _>>()?;
        calibrations.push((result, operands));
    }
    // Part 1: find calibrations solvable with sum and products
    for cal in calibrations.clone() {
        let ops = ez_operations
            .entry(cal.1.len() - 1)
            .or_insert(generate_dispositions(cal.1.len() - 1, true));

        if result_is_achievable(cal.0, &cal.1, ops) {
            sum_of_calibrations += cal.0;
            calibrations.remove(calibrations.iter().position(|x| *x == cal).unwrap());
        }
    }
    println!("Part 1: {sum_of_calibrations}");
    // Part 2: find calibrations solvable with sum, products and concatenation
    for cal in calibrations {
        let ops = complex_operations
            .entry(cal.1.len() - 1)
            .or_insert(generate_dispositions(cal.1.len() - 1, false));
        if result_is_achievable(cal.0, &cal.1, ops) {
            sum_of_calibrations += cal.0;
        }
    }
    println!("Part 2: {sum_of_calibrations}");
    Ok(())
}
