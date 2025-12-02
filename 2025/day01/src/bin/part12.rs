use std::{error::Error, fs};

const DIAL_SIZE: i16 = 100;
const INITIAL_POSITION: i16 = 50;

fn part_1(input: &str) -> u32 {
    let mut state: i16 = INITIAL_POSITION;
    let mut zero_counts: u32 = 0;
    input
        .lines()
        .map(|line| {
            let mut clicks: i16 = line[1..].parse().unwrap();
            // Rotations are here expressed as signed
            // integers, with right rotations being
            // considered positive.
            if line.chars().nth(0).unwrap() == 'L' {
                clicks *= -1;
            }
            clicks
        })
        .for_each(|rotation| {
            // rotations should wrap at DIAL_SIZE and 0
            state = (state + rotation).rem_euclid(DIAL_SIZE);
            if state == 0 {
                zero_counts += 1;
            }
        });
    zero_counts
}

fn part_2(input: &str) -> u32 {
    let mut state: i16 = INITIAL_POSITION;
    let mut zero_counts: u32 = 0;
    input
        .lines()
        .map(|line| {
            let mut clicks: i16 = line[1..].parse().unwrap();
            // Rotations are here expressed as signed
            // integers, with right rotations being
            // considered positive.
            if line.chars().nth(0).unwrap() == 'L' {
                clicks *= -1;
            }
            clicks
        })
        .for_each(|rotation| {
            // count how many times the rotation will surely go through 0
            zero_counts += (rotation.abs() / DIAL_SIZE) as u32;
            // remove the maximum multiple of DIAL_SIZE
            let reduced_rotation = rotation % DIAL_SIZE;
            state += reduced_rotation;
            if (state < 0 && state > reduced_rotation) || state > DIAL_SIZE {
                zero_counts += 1;
            }
            else if (state == 0 || state == DIAL_SIZE) && reduced_rotation != 0 {
                zero_counts += 1;
                if state == DIAL_SIZE {
                    state = 0;
                }
            }
            state = state.rem_euclid(100);
        });
    zero_counts
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
        assert_eq!(part_1(&get_input(true)), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(&get_input(true)), 6);
    }
}
