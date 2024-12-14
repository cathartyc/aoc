use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Copy)]
struct Loc {
    x: f64,
    y: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input13.txt");
    let input = fs::read_to_string(path)?;
    // Code
    let re = Regex::new(r"X[\+=](?<x>\d+), Y[\+=](?<y>\d+)")?;
    let mut sum = 0;
    let mut lines = input.lines();
    while let (Some(a), Some(b), Some(prize), _) =
        (lines.next(), lines.next(), lines.next(), lines.next())
    {
        let caps = re.captures(a).unwrap();
        let a = Loc {
            x: caps["x"].parse()?,
            y: caps["y"].parse()?,
        };
        let caps = re.captures(b).unwrap();
        let b = Loc {
            x: caps["x"].parse()?,
            y: caps["y"].parse()?,
        };
        let caps = re.captures(prize).unwrap();
        let mut prize = Loc {
            x: caps["x"].parse()?,
            y: caps["y"].parse()?,
        };
        prize.x += 10000000000000.0;
        prize.y += 10000000000000.0;
        // Logic
        let n_1 = (prize.x - (b.x / b.y) * prize.y) / (a.x * (1.0 - (a.y * b.x) / (a.x * b.y)));
        let n_2 = (prize.y - a.y * n_1) / b.y;
        if n_1 < 0.0 || n_2 < 0.0 {
            continue;
        }
        let mantissa_1 = n_1 - (n_1 as u64) as f64;
        let mantissa_2 = n_2 - (n_2 as u64) as f64;
        if 0.1 < mantissa_1 && mantissa_1 < 0.9 {
            continue;
        }
        if 0.1 < mantissa_2 && mantissa_2 < 0.9 {
            continue;
        }
        if a.x * n_1.round() + b.x * n_2.round() == prize.x {
            sum += n_1.round() as u64 * 3 + n_2.round() as u64;
        }
    }
    println!("{sum}");
    // Return Ok
    Ok(())
}
