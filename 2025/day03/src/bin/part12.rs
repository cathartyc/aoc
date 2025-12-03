use std::{error::Error, fs};

fn part_1(input: &str) -> u32 {
    let mut sum: u32 = 0;
    input.lines().for_each(|bank| {
        let mut first: usize = usize::MAX;
        let mut second: usize = usize::MAX;
        for i in (1..=9).rev() {
            if let Some(attempt) = bank[..bank.len() - 1].find(&i.to_string()) {
                first = attempt;
                break;
            }
        }
        if first == usize::MAX {
            unreachable!("No value assigned for first");
        }
        // here first will surely have a value.
        for i in (1..=9).rev() {
            if let Some(attempt) = bank[first + 1..].find(&i.to_string()) {
                second = attempt + first + 1; // remember that indexes in slices are relative
                break;
            }
        }
        if second == usize::MAX {
            unreachable!("No value assigned for second");
        }
        sum += bank.chars().nth(first).unwrap().to_digit(10).unwrap() * 10
            + bank.chars().nth(second).unwrap().to_digit(10).unwrap();
    });
    sum
}

const PACK_SIZE: usize = 12;

fn part_2(input: &str) -> u64 {
    let mut sum: u64 = 0;
    input
        .lines()
        .for_each(|bank| {
        let mut batteries = vec![];
        let mut next_index = 0;
        while batteries.len() < PACK_SIZE {
            let mut i: i8 = 9;
            while i >= 0 {
                if let Some(attempt) = bank
                    [next_index..bank.len() - (PACK_SIZE - batteries.len() - 1)]
                    .find(&i.to_string())
                {
                    batteries.push(i as u64);
                    next_index += attempt + 1;
                    break;
                } else {
                    i -= 1;
                }
            }
        }
        sum += batteries
            .into_iter()
            .reduce(|acc, el| acc * 10 + el)
            .unwrap();
    });
    sum
}

/// Gets the input for the puzzle
fn get_input(is_example: bool) -> String {
    let project_dir = env!("CARGO_MANIFEST_DIR");
    let path = project_dir.to_string()
        + "/../inputs/input-"
        + &project_dir[project_dir.len() - 2..] // XX in /home/.../dayXX
        + (if is_example { "-ex" } else { "" })
        + ".txt";
    println!("{path}");
    fs::read_to_string(path).unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input(false);
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
        assert_eq!(part_1(&get_input(true)), 357);
    }
    #[test]
    fn test_2() {
        assert_eq!(part_2(&get_input(true)), 3121910778619);
    }
}
