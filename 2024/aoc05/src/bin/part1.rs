use std::error::Error;
use std::fs;
use std::path::PathBuf;

/// Check if the order of the two values in the rule is respected.
fn order_is_respected(seq: &[u16], rule: &(u16, u16)) -> bool {
    if let Some(first_index) = seq.iter().position(|&x| x == rule.0) {
        if seq[..first_index].iter().any(|&x| x == rule.1) {
            return false;
        }
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input5.txt");
    let input = fs::read_to_string(path)?;
    // Code
    let mut orderings: Vec<(u16, u16)> = vec![];
    let mut sequences: Vec<Vec<u16>> = vec![];
    let mut sum_of_middles: u32 = 0;
    // Read ordering rules
    let mut lines = input.lines();
    for line in &mut lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let couple = line
            .split('|')
            .map(|num| num.parse())
            .collect::<Result<Vec<u16>, _>>()?;
        orderings.push((couple[0], couple[1]));
    }
    // Read page sequences
    for line in lines {
        let pages = line
            .split(',')
            .map(|num| num.parse())
            .collect::<Result<Vec<u16>, _>>()?;
        sequences.push(pages);
    }
    //
    // Check every sequence against the ordering rules
    'sequences: for seq in &sequences {
        for rule in &orderings {
            if !order_is_respected(seq, rule) {
                continue 'sequences;
            }
        }
        // If positive, take its middle value and sum it
        sum_of_middles += seq[seq.len() / 2] as u32;
    }
    println!("{sum_of_middles}");
    Ok(())
}
