use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Copy)]
struct Block {
    id: u32,
    start: u32,
    size: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input9.txt");
    let input = fs::read_to_string(path)?
        .strip_suffix("\n")
        .unwrap()
        .to_string();
    // Code
    let mut is_block = true;
    let mut block_index = 0;
    let mut block_step = 0;
    let mut checksum: u128 = 0;
    //
    let mut files: Vec<Block> = vec![];
    let mut free_spaces: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut max_space = 0;
    // In this puzzle, the input is a single line.
    for ch in input.chars() {
        let val: u32 = ch.to_digit(10).expect("not a digit");
        if is_block {
            files.push(Block {
                id: block_index,
                start: block_step,
                size: val,
            });
            block_index += 1;
        } else {
            free_spaces
                .entry(val)
                .and_modify(|vec| {
                    vec.push(block_step);
                    vec.sort();
                })
                .or_insert(vec![block_step]);
            if val > max_space {
                max_space = val;
            }
        }
        block_step += val;
        is_block = !is_block;
    }
    for file in files.iter_mut().rev() {
        // There surely is no valid candidate at 0
        let mut candidate = Block {
            id: 0, // unused
            start: 0,
            size: 0,
        };
        for size in file.size..=max_space {
            let indexes = free_spaces.get(&size);
            if indexes.is_none() {
                continue;
            }
            if indexes.unwrap().is_empty() {
                continue;
            }
            let i = indexes.unwrap().first().unwrap();
            if *i > file.start {
                continue;
            }
            if *i < candidate.start || candidate.size == 0 {
                candidate.start = *i;
                candidate.size = size;
                continue;
            }
        }
        if candidate.size == 0 {
            continue;
        }
        file.start = candidate.start;
        _ = free_spaces.get_mut(&candidate.size).unwrap().remove(0);
        if file.size < candidate.size {
            free_spaces
                .entry(candidate.size - file.size)
                .and_modify(|vec| {
                    vec.push(file.start + file.size);
                    vec.sort();
                })
                .or_insert(vec![file.start + file.size]);
        }
    }
    // Printing and evaluating
    let mut index = 0;
    files.sort_by(|a, b| a.start.cmp(&b.start));
    for file in files {
        if index < file.start {
            for _ in 0..file.start - index {
                // print!(".");
            }
        }
        index = file.start + file.size;
        for _ in 0..file.size {
            //print!("{}", file.id);
        }
        checksum += (file.start..file.start + file.size)
            .reduce(|a, b| a + b)
            .unwrap() as u128
            * file.id as u128;
    }
    // println!();
    assert_eq!(checksum, 6250605700557);
    // println!("{checksum}");
    Ok(())
}
