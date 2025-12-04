use std::{cell::Cell, error::Error, fs};

const ROLL: char = '@';

fn part_1(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    // check if the map if a square
    assert_eq!(map.len(), map[0].len());
    let size = map.len();

    for (y, line) in map.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch != ROLL {
                continue;
            }

            let mut count = 0;
            if y > 0
                && let Some(top) = map.get(y - 1)
            {
                if let Some(three_top) = top.get(x.saturating_sub(1)..=usize::min(x + 1, size - 1))
                {
                    for c in three_top {
                        if *c == ROLL {
                            count += 1;
                        }
                    }
                }
            }

            let same_row = &map[y]; // this surely exists
            if x.saturating_sub(1) != x && same_row[x - 1] == ROLL {
                count += 1;
            }
            if usize::min(x + 1, size - 1) != x && same_row[x + 1] == ROLL {
                count += 1;
            }

            if let Some(bottom) = map.get(y + 1) {
                if let Some(three_bottom) =
                    bottom.get(x.saturating_sub(1)..=usize::min(x + 1, size - 1))
                {
                    for c in three_bottom {
                        if *c == ROLL {
                            count += 1;
                        }
                    }
                }
            }
            if count < 4 {
                sum += 1;
            }
        }
    }
    sum
}

fn part_2(input: &str) -> u64 {
    let mut sum: u64 = 0;
    let map: Vec<Vec<Cell<char>>> = input
        .lines()
        .map(|l| l.chars().map(|c| Cell::new(c)).collect())
        .collect();
    // check if the map if a square
    assert_eq!(map.len(), map[0].len());
    let size = map.len();
    let mut new_evictions: bool = true;
    while new_evictions {
        new_evictions = false;
        for (y, line) in map.iter().enumerate() {
            for (x, ch) in line.iter().enumerate() {
                if ch.get() != ROLL {
                    continue;
                }

                let mut count = 0;
                if y > 0
                    && let Some(top) = map.get(y - 1)
                {
                    if let Some(three_top) =
                        top.get(x.saturating_sub(1)..=usize::min(x + 1, size - 1))
                    {
                        for c in three_top {
                            if c.get() == ROLL {
                                count += 1;
                            }
                        }
                    }
                }

                let same_row = &map[y]; // this surely exists
                if x.saturating_sub(1) != x && same_row[x - 1].get() == ROLL {
                    count += 1;
                }
                if usize::min(x + 1, size - 1) != x && same_row[x + 1].get() == ROLL {
                    count += 1;
                }

                if let Some(bottom) = map.get(y + 1) {
                    if let Some(three_bottom) =
                        bottom.get(x.saturating_sub(1)..=usize::min(x + 1, size - 1))
                    {
                        for c in three_bottom {
                            if c.get() == ROLL {
                                count += 1;
                            }
                        }
                    }
                }
                if count < 4 {
                    sum += 1;
                    new_evictions = true;
                    map[y][x].set('x')
                }
            }
        }
    }
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
        assert_eq!(part_1(&get_input(true)), 13);
    }
    #[test]
    fn test_2() {
        assert_eq!(part_2(&get_input(true)), 43);
    }
}
