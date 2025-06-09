use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn compose_towel<'a>(
    towel: &'a str,
    patterns: &HashSet<&str>,
    memoization: &mut HashMap<&'a str, u64>,
    start: usize,
) -> u64 {
    let mut count = 0;
    if memoization.contains_key(&towel[start..]) {
        return *memoization.get(&towel[start..]).unwrap();
    }
    if start == towel.len() {
        return 1;
    }
    for end in start + 1..=towel.len() {
        let subs = &towel[start..end];
        if patterns.contains(subs) {
            count += compose_towel(towel, patterns, memoization, end);
        }
    }
    memoization.insert(&towel[start..], count);
    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input19.txt");
    let input = fs::read_to_string(path)?;
    let mut lines = input.lines();
    // patterns
    let patterns: HashSet<&str> = lines.next().unwrap().split(',').map(|s| s.trim()).collect();
    // empty line
    lines.next();
    // main part
    let mut possible_towels: u64 = 0;
    // As always, the key behind heavy recursion is memoization...
    let mut memoization: HashMap<&str, u64> = HashMap::default();
    loop {
        let Some(towel) = lines.next() else { break };
        possible_towels += compose_towel(towel, &patterns, &mut memoization, 0);
    }
    println!("{possible_towels}");
    Ok(())
}
