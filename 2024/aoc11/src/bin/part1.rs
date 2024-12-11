use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input11.txt");
    let input = fs::read_to_string(path)?;
    // Code
    const STEPS: usize = 25;
    const FACTOR: u64 = 2024;
    let mut stones: Vec<u64> = input
        .strip_suffix('\n')
        .unwrap()
        .split(' ')
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    for _ in 0..STEPS {
        let mut next_iteration = vec![];
        for stone in stones {
            match stone {
                0 => next_iteration.push(1),
                _ => {
                    let num_len: u32 = stone.ilog10() + 1;
                    if num_len % 2 == 0 {
                        let pow_ten = 10u64.pow(num_len / 2);
                        next_iteration.push(stone / pow_ten);
                        next_iteration.push(stone % pow_ten);
                    } else {
                        next_iteration.push(stone * FACTOR);
                    }
                }
            }
        }
        stones = next_iteration;
    }
    println!("{}", stones.len());
    // Return Ok
    Ok(())
}
