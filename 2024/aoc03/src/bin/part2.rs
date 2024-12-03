use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

/// Finds all patterns in the line and returns them in a vector.
fn find_patterns<'a>(line: &'a str, pattern: &Regex) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for m in pattern.find_iter(line) {
        result.push(m.as_str());
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input3.txt");
    let input = fs::read_to_string(path)?;
    //    Code
    let mut result = 0;
    let pattern = Regex::new(r"do\(\)|don't\(\)|mul\(\d+,\d+\)")?;
    let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let mut flip_flop = true;
    for line in input.lines() {
        let scan = find_patterns(line, &pattern);
        for pattern in scan.into_iter() {
            match pattern {
                "do()" => flip_flop = true,
                "don't()" => flip_flop = false,
                _ => {
                    if flip_flop {
                        result += mul_pattern
                            .captures_iter(pattern)
                            .map(|m| {
                                let (_, [fact_a, fact_b]) = m.extract();
                                fact_a.parse::<u32>().unwrap() * fact_b.parse::<u32>().unwrap()
                            })
                            .reduce(|a: u32, b: u32| a + b)
                            .unwrap();
                    }
                }
            }
        }
    }
    println!("{result:?}");
    Ok(())
}
