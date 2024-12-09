use std::error::Error;
use std::fs;
use std::path::PathBuf;

struct Residual {
    block: u32,
    amount: u32,
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
    let mut is_block_back = true;
    let mut residual = Residual {
        block: 0,
        amount: 0,
    };
    let mut index_back: usize = input.len() - 1;
    // In this puzzle, the input is a single line.
    let mut iter_1 = input.chars();
    let mut iter_2 = input.chars().rev();
    let mut forward = iter_1.by_ref().enumerate();
    let mut back = iter_2.by_ref().enumerate();
    'main: loop {
        let Some((i, ch)) = forward.next() else {
            println!("{block_step}: crashing here");
            panic!();
        };
        if i >= index_back {
            break;
        }
        let val: u32 = ch.to_digit(10).expect("not a digit");
        if is_block {
            for i in block_step..block_step + val {
                checksum += block_index * i as u128;
                //print!("{block_index}");
            }
            block_index += 1;
            block_step += val;
        } else {
            let mut space = val;
            'space: while space > 0 {
                while residual.amount > 0 {
                    let filling = u32::min(residual.amount, space);
                    space -= filling;
                    residual.amount -= filling;
                    for k in block_step..block_step + filling {
                        checksum += (residual.block * k) as u128;
                        //print!("{}", residual.block);
                    }
                    block_step += filling;
                    if space == 0 {
                        break 'space;
                    }
                }
                let (j, ch_back) = back.next().unwrap();
                index_back = input.len() - j - 1;
                if i >= index_back {
                    break 'main;
                }
                let val = ch_back.to_digit(10).unwrap();
                if is_block_back {
                    residual.block = index_back as u32 / 2;
                    residual.amount = val;
                }
                is_block_back = !is_block_back;
            }
        }
        is_block = !is_block;
    }
    if residual.amount > 0 {
        for i in block_step..block_step + residual.amount {
            checksum += (residual.block * i) as u128;
            //print!("{}", residual.block);
        }
    }
    //println!();
    println!("{checksum}");
    Ok(())
}
