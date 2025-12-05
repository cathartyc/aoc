use std::error::Error;
use utils::get_input;

fn part_1(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let mut input_lines = input.lines();

    // Ranges of good IDs
    let ranges: Vec<(u64, u64)> = input_lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|range_raw| {
            let mut extremes = range_raw.split('-').map(|el| el.parse().unwrap());
            (extremes.next().unwrap(), extremes.next().unwrap())
        })
        .collect();
    // IDs to check
    input_lines
        .map(|id_raw| id_raw.parse().unwrap())
        .for_each(|id| {
            if ranges.iter().any(|range| range.0 <= id && id <= range.1) {
                sum += 1;
            }
        });
    sum
}

fn part_2(input: &str) -> u64 {
    // Ranges of good IDs
    let mut ranges: Vec<(u64, u64)> = vec![];
    input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|range_raw| {
            let mut extremes = range_raw.split('-').map(|el| el.parse::<u64>().unwrap());
            (extremes.next().unwrap(), extremes.next().unwrap())
        })
        .for_each(|mut range| {
            let mut index = ranges.partition_point(|r| r.0 <= range.0);
            if index != 0 {
                let prev_range = ranges[index - 1];
                // the new range is totally contained into an already present one
                // -> discard
                if prev_range.1 >= range.1 {
                    return;
                }
                // the new range is overlapping (or totally adjacent) with the previous one
                // -> merge
                if prev_range.1 >= range.0 - 1 {
                    range.0 = prev_range.0;
                    ranges.remove(index - 1);
                    index -= 1;
                }
            }
            ranges.insert(index, range);
            while index + 1 < ranges.len() {
                let next_range = ranges[index + 1];
                // new range contains entirely the next one
                if range.1 >= next_range.1 {
                    ranges.remove(index + 1);
                    continue;
                }
                // new range overlaps with the next one
                if range.1 >= next_range.0 - 1 {
                    ranges[index].1 = next_range.1;
                    ranges.remove(index + 1);
                } else {
                    break;
                }
            }
        });
    ranges.iter().map(|range| range.1 - range.0 + 1).sum()
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
        assert_eq!(part_1(&get_input(PATH, true)), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(part_2(&get_input(PATH, true)), 14);
    }
}
