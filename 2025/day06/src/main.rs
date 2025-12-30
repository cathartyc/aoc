use std::collections::HashMap;
use std::error::Error;
use utils::get_input;

enum Op {
    SUM,
    PRODUCT,
}

fn part_1(input: &str) -> u64 {
    let mut sum: u64 = 0;
    // products are stored per operation ID
    let mut products: HashMap<usize, u64> = HashMap::default();
    let mut lines = input.lines().rev();
    let ops: Vec<Op> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(&|op| match op {
            "+" => Op::SUM,
            "*" => Op::PRODUCT,
            _ => unreachable!(),
        })
        .collect();
    for line in lines {
        let vals = line
            .split_whitespace()
            .map(|val| val.parse::<u64>().unwrap());
        for (i, val) in vals.enumerate() {
            match ops[i] {
                Op::SUM => sum += val,
                Op::PRODUCT => _ = products.entry(i).and_modify(|v| *v *= val).or_insert(val),
            }
        }
    }
    sum + products.values().sum::<u64>()
}

fn transpose_input(input: &str) -> Vec<String> {
    let line_len = input.lines().next().unwrap().len();
    let mut transposed_lines: Vec<String> = Vec::with_capacity(line_len);
    for _ in 0..line_len {
        transposed_lines.push(String::new());
    }
    for line in input.lines() {
        for (i, ch) in line.char_indices() {
            transposed_lines.get_mut(i).unwrap().push(ch);
        }
    }
    transposed_lines
}

fn part_2(input: &str) -> u64 {
    let mut sum: u64 = 0;
    let mut products: HashMap<usize, u64> = HashMap::default();
    // transpose
    let lines_transpose = transpose_input(input);
    let ops: Vec<Op> = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(&|op| match op {
            "+" => Op::SUM,
            "*" => Op::PRODUCT,
            _ => unreachable!(),
        })
        .collect();
    let mut current_op_index = 0;
    for line in lines_transpose.iter() {
        let val_raw = line.strip_suffix(&['+', '*']).unwrap_or(line);
        if let Ok(val) = val_raw.trim().parse::<u64>() {
            match ops[current_op_index] {
                Op::SUM => sum += val,
                Op::PRODUCT => {
                    _ = products
                        .entry(current_op_index)
                        .and_modify(|v| *v *= val)
                        .or_insert(val)
                }
            }
        } else {
            current_op_index += 1;
        }
    }
    sum + products.values().sum::<u64>()
}

const PATH: &str = env!("CARGO_MANIFEST_DIR");

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input(PATH, false);
    let result_pt1 = part_1(&input);
    println!("[Part 1] The result is {result_pt1}.");
    let result_pt2 = part_2(&input);
    println!("[Part 2] The result is {result_pt2}.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part_1(&get_input(PATH, true)), 4277556);
    }
    #[test]
    fn test_2() {
        assert_eq!(part_2(&get_input(PATH, true)), 3263827);
    }
}
