use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input3.txt");
    let input = fs::read_to_string(path)?;
    // Code
    let mut result = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for line in input.lines() {
        result += re
            .captures_iter(line)
            .map(|m| {
                let (_, [fact_a, fact_b]) = m.extract();
                fact_a.parse::<u32>().unwrap() * fact_b.parse::<u32>().unwrap()
            })
            .reduce(|a, b| a + b)
            .unwrap();
    }
    println!("{result:?}");
    Ok(())
}
