use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn compose_towel(towel: &str, patterns: &[&str]) -> Option<()> {
    let valid_prefixes = patterns.iter().filter(|&&tow| towel.starts_with(tow));
    for &t in valid_prefixes {
        if towel.len() == t.len() || compose_towel(&towel[t.len()..], patterns).is_some() {
            return Some(());
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input19.txt");
    let input = fs::read_to_string(path)?;
    let mut lines = input.lines();
    // patterns
    let mut patterns: Vec<&str> = lines.next().unwrap().split(',').map(|s| s.trim()).collect();
    patterns.sort_by_key(|t| t.len());
    patterns.reverse();
    println!("{patterns:?}");
    // empty line
    lines.next();
    // duty
    let mut possible_towels: u64 = 0;
    loop {
        let Some(towel) = lines.next() else { break };
        if compose_towel(towel, &patterns).is_some() {
            possible_towels += 1;
        }
    }
    println!("{possible_towels}");
    Ok(())
}
