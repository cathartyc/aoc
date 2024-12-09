use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Copy)]
struct Block {
    id: u32,
    start: u32,
    size: u32,
    moved: bool,
}

struct Space {
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
    let mut moved_files: Vec<Block> = vec![];
    let mut free_spaces: Vec<Space> = vec![];
    // In this puzzle, the input is a single line.
    for ch in input.chars() {
        let val: u32 = ch.to_digit(10).expect("not a digit");
        if is_block {
            files.push(Block {
                id: block_index,
                start: block_step,
                size: val,
                moved: false,
            });
            block_index += 1;
        } else {
            free_spaces.push(Space {
                start: block_step,
                size: val,
            });
        }
        block_step += val;
        is_block = !is_block;
    }
    for file in files.iter_mut().rev() {
        match free_spaces
            .iter_mut()
            .find(|s| file.size <= s.size && file.start > s.start)
        {
            Some(space) => {
                file.start = space.start;
                space.size -= file.size;
                space.start += file.size;
            }
            None => {
                file.moved = true;
            }
        }
        free_spaces.retain(|x| x.size > 0);
    }
    // Printing and evaluating
    files.extend(moved_files);
    let mut index = 0;
    files.sort_by(|a, b| a.start.cmp(&b.start));
    for file in files {
        if index < file.start {
            for _ in 0..file.start - index {
                //print!(".");
            }
        }
        index = file.start + file.size;
        for _ in 0..file.size {
            //print!("{}", file.id);
        }
        checksum += ((file.start..file.start + file.size)
            .reduce(|a, b| a + b)
            .unwrap()
            * file.id) as u128;
    }
    //println!();
    println!("{checksum}");
    Ok(())
}
