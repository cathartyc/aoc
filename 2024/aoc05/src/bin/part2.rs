use std::error::Error;
use std::fs;
use std::path::PathBuf;

/// Check if the order of the two values in the rule is respected.
fn order_is_respected(seq: &[u16], rule: &(u16, u16)) -> bool {
    if let Some(index) = seq.iter().position(|&x| x == rule.0) {
        if seq[..index].iter().any(|&x| x == rule.1) {
            return false;
        }
    }
    true
}

/// Fixes the sequence of pages, in order to comply with all the rules.
fn fix_rule(seq: &[u16], rules: &[(u16, u16)]) -> Vec<u16> {
    let mut fixed = true;
    let mut fixed_seq: Vec<u16> = seq.to_vec();
    let mut rules_iter = rules.iter();
    let mut rule: &(u16, u16);
    loop {
        let r = rules_iter.next();
        match r {
            Some(content) => {
                rule = content;
            }
            None => {
                if fixed {
                    break;
                } else {
                    fixed = true;
                    rules_iter = rules.iter();
                    rule = rules_iter.next().unwrap();
                }
            }
        }
        if !order_is_respected(&fixed_seq, rule) {
            // Swap them just once, since every element of the sequence is unique
            let first_index = fixed_seq.iter().position(|&x| x == rule.0).unwrap();
            let second_index = fixed_seq[..first_index]
                .iter()
                .position(|&x| x == rule.1)
                .unwrap();
            fixed_seq.swap(first_index, second_index);
            if fixed {
                fixed = false;
            }
        }
    }
    fixed_seq
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
    // Check every sequence against the ordering rules
    'sequences: for seq in &sequences {
        for rule in &orderings {
            if !order_is_respected(seq, rule) {
                // If the sequence is wrong, fix it
                let sorted_rule: Vec<u16> = fix_rule(seq, &orderings);
                // ...and read its middle value
                sum_of_middles += sorted_rule[seq.len() / 2] as u32;
                continue 'sequences;
            }
        }
    }
    println!("{sum_of_middles}");
    Ok(())
}
