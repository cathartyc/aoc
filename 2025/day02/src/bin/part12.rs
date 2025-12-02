use ilog::*;
use std::{error::Error, fs};

/// Gets the number of digits of the given number.
fn len(num: u64) -> u32 {
    num.log10() as u32 + 1
}

fn part_1(input: &str) -> u64 {
    let mut sum = 0;
    input
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .map(|raw_range| {
            let mut vals = raw_range.split('-');
            (
                vals.next().unwrap().parse::<u64>().unwrap(),
                vals.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .for_each(|range| {
            let mut start = range.0;
            let end = range.1;
            // get smallest value with an even number of digits in the range
            // 1. count the digits
            let mut start_len = len(start);
            // 2. check if the number of digits is even
            if start_len % 2 != 0 {
                // if not, get the next power of 10 with an even number of digits
                start = 10_u64.pow(start_len); // NB: 10^n has n+1 digits)
                start_len += 1;
            }

            // extract the first half
            let mut half = start / 10_u64.pow(start_len / 2);
            // build the number as "halfhalf"
            let mut curr = half * (10_u64.pow(len(half)) + 1);
            if curr < start {
                // happens when the first half is greater than the second one
                half += 1;
                curr = half * (10_u64.pow(len(half)) + 1);
            }
            while curr <= end {
                sum += curr;
                half += 1;
                curr = half * (10_u64.pow(len(half)) + 1);
            }
        });
    sum
}

fn part_2(input: &str) -> u64 {
    let mut sum = 0;
    input
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .map(|raw_range| {
            let mut vals = raw_range.split('-');
            (
                vals.next().unwrap().parse::<u64>().unwrap(),
                vals.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .for_each(|range| {
            let start = range.0;
            let end = range.1;
            let mut id = start;
            while id <= end {
                let id_as_str = id.to_string();
                let id_len = id_as_str.len();
                // comparisons go from the most significant digit through the middle one
                for buf_len in 1..=id_len / 2 {
                    // don't count numbers if they don't fit multiples of the buffer
                    if id_len % buf_len == 0
                        // id == b.b.b.b.b...
                        && id_as_str == id_as_str[..buf_len].repeat(id_len / buf_len)
                    {
                        sum += id;
                        // the smallest pattern is sufficient
                        break;
                    }
                }
                // not proud of this: test every number
                id += 1;
            }
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
        assert_eq!(part_1(&get_input(true)), 1227775554);
    }
    #[test]
    fn test_2() {
        assert_eq!(part_2(&get_input(true)), 4174379265);
    }
}
