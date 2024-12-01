use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut list1: Vec<u32> = vec![];
    let mut list2: Vec<u32> = vec![];
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input1.txt");
    let input = fs::read_to_string(path)?;
    for line in input.lines() {
        let line_split: Vec<&str> = line.split("   ").collect();
        list1.push(line_split[0].parse::<u32>()?);
        list2.push(line_split[1].parse::<u32>()?);
    }
    list1.sort();
    list2.sort();
    let mut sum: u32 = 0;
    for i in 0..list1.len() {
        sum += list1[i].abs_diff(list2[i]);
    }
    println!("Total sum: {sum}");
    Ok(())
}
