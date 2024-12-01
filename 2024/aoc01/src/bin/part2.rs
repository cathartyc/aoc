use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut list1: Vec<u32> = vec![];
    let mut list2: Vec<u32> = vec![];
    let input = fs::read_to_string("../inputs/input1.txt")?;
    for line in input.lines() {
        let line_split: Vec<&str> = line.split("   ").collect();
        list1.push(line_split[0].parse::<u32>()?);
        list2.push(line_split[1].parse::<u32>()?);
    }
    let mut sum: u32 = 0;
    let mut map = HashMap::new();
    for el in list2.iter() {
        map.entry(el).and_modify(|num| *num += 1).or_insert(1);
    }
    for i in &list1 {
        if let Some(factor) = map.get(i) {
            sum += i * factor;
        }
    }
    println!("Total sum of products: {sum}");
    Ok(())
}
