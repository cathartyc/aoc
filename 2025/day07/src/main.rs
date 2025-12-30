use memoize::memoize;
use std::{cell::Cell, error::Error};
use utils::{Loc, get_input};

const START: char = 'S';
const SPLITTER: char = '^';
const BEAM: char = '|';

fn part_1(input: &str) -> u64 {
    let mut sum: u64 = 0;

    let space: Vec<Vec<Cell<char>>> = input
        .lines()
        .map(|l| l.chars().map(|ch| Cell::new(ch)).collect())
        .collect();
    for (y, line) in space.iter().enumerate().skip(1) {
        for (x, ch) in line.iter().enumerate() {
            let top = space[y - 1][x].get();
            if top != BEAM && top != START {
                continue;
            }
            if ch.get() == SPLITTER {
                space[y][x - 1].set(BEAM); // there are no beams at the border of the input
                space[y][x + 1].set(BEAM);
                sum += 1;
            } else {
                space[y][x].set(BEAM);
            }
        }
    }
    sum
}

#[memoize(Ignore: input)]
fn possible_outcomes(input: &[Vec<char>], splitter: Loc<usize>) -> u64 {
    let mut sum: u64 = 0;
    let mut found_left = false;
    let mut found_right = false;
    for y in (splitter.y + 2..input.len()).step_by(2) {
        if !found_left && input[y][splitter.x - 1] == SPLITTER {
            sum += possible_outcomes(
                input,
                Loc {
                    x: splitter.x - 1,
                    y: y,
                },
            );
            found_left = true;
        }
        if !found_right && input[y][splitter.x + 1] == SPLITTER {
            sum += possible_outcomes(
                input,
                Loc {
                    x: splitter.x + 1,
                    y: y,
                },
            );
            found_right = true;
        }
        if found_left && found_right {
            break;
        }
    }
    if !found_left && !found_right {
        sum += 2;
    } else if !found_left || !found_right {
        sum += 1;
    }
    sum
}

fn part_2(input: &str) -> u64 {
    let space: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start_x = space[0].iter().position(|&ch| ch == START).unwrap(); // or space[0].len() / 2
    let start_splitter: Loc<usize> = Loc { x: start_x, y: 2 };
    let sum = possible_outcomes(&space, start_splitter);
    sum
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
        assert_eq!(part_1(&get_input(PATH, true)), 21);
    }
    #[test]
    fn test_2() {
        assert_eq!(part_2(&get_input(PATH, true)), 40);
    }
}
