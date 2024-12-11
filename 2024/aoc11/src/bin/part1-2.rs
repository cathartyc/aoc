use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input11.txt");
    let input = fs::read_to_string(path)?;
    // Code
    const FACTOR: u64 = 2024;
    // Using an hashmap to turn an otherwise huge vector into a sparse histogram
    let mut stones: HashMap<u64, u64> = input
        .strip_suffix('\n')
        .unwrap()
        .split(' ')
        .map(|n| (n.parse::<u64>().unwrap(), 1))
        .collect();
    // Part 1
    let part1 = blink(&mut stones.clone(), 25, FACTOR);
    println!("Part 1: {part1}");
    // Part 2, the same but with 3x the iterations of part 1
    let part2 = blink(&mut stones, 75, FACTOR);
    println!("Part 2: {part2}");
    // Return Ok
    Ok(())
}

/// Compute a blink, that is an iteration through the stones.
fn blink(stones: &mut HashMap<u64, u64>, steps: usize, factor: u64) -> u64 {
    // The cache is used to store the results of the stone mutations
    let mut cache: HashMap<u64, Vec<u64>> = HashMap::default();
    for _ in 0..steps {
        // Argued with Rust a lot on this one
        let stones_iter: Vec<(u64, u64)> =
            stones.clone().into_iter().filter(|&(_, v)| v > 0).collect();
        for (stone, amount) in stones_iter.clone() {
            let new_stones = mutate_stone(stone, &mut cache, factor);
            for new_stone in new_stones {
                stones
                    .entry(new_stone)
                    .and_modify(|v| *v += amount)
                    .or_insert(amount);
            }
        }
        // Do this later to avoid update concurrency
        for (stone, amount) in stones_iter {
            stones.entry(stone).and_modify(|v| *v -= amount);
        }
    }
    stones.values().sum::<u64>()
}

/// Return an evolution of the stones, with the following rules:
/// - if the stone is 0, return 1;
/// - if the stone has an even number of digits, return its two halves;
/// - otherwise, return the stone multiplied by factor.
fn mutate_stone(stone: u64, cache: &mut HashMap<u64, Vec<u64>>, factor: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }
    cache
        // if already computed, the call stops here
        .entry(stone)
        .or_insert({
            let num_len: u32 = stone.ilog10() + 1;
            if num_len % 2 == 0 {
                let pow_ten = 10u64.pow(num_len / 2);
                vec![stone / pow_ten, stone % pow_ten]
            } else {
                vec![stone * factor]
            }
        })
        .to_vec()
}
