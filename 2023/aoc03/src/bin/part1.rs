use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Clone)]
struct Component {
    value: u32,
    start: usize,
    end: usize,
}
#[derive(Clone)]
struct Symbol {
    start: usize,
    end: usize,
}

fn make_serial(val: String, last_pos: usize) -> Component {
    Component {
        value: val.parse().expect("Should be a number here"),
        start: last_pos - val.len(),
        end: last_pos - 1,
    }
}

fn sum_and_reduce(symbols: &Vec<Symbol>, components: &mut Vec<Component>) -> u32 {
    let mut sum = 0;
    for symbol in symbols.iter() {
        sum += components
            .iter()
            .filter(|&c| symbol.start <= c.end && symbol.end >= c.start)
            .map(|c| c.value)
            .sum::<u32>();
        components.retain(|c| symbol.start >= c.end || symbol.end <= c.start);
    }
    sum
}

fn main() -> Result<(), Error> {
    let input = File::open("../inputs/input3")?;
    let input = BufReader::new(input);
    let mut previous_line: Vec<Component> = Vec::new();
    let mut curr_line: Vec<Component> = Vec::new();
    let mut prev_symbols: Vec<Symbol> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut val: String = "".to_string();
    let mut total: u32 = 0;
    for line in input.lines() {
        for (i, ch) in line.as_ref().unwrap().chars().enumerate() {
            if ch.is_ascii_digit() {
                val.push(ch);
                continue;
            }
            if !val.is_empty() {
                curr_line.push(make_serial(val.clone(), i));
                val.clear();
            }
            if ch == '.' {
                continue;
            } else {
                symbols.push(Symbol {
                    start: i - 1,
                    end: i + 1,
                });
            }
        }
        if !val.is_empty() {
            curr_line.push(make_serial(val.clone(), line.unwrap().len()));
            val.clear();
        }
        total += sum_and_reduce(&prev_symbols, &mut curr_line);
        total += sum_and_reduce(&symbols, &mut previous_line);
        total += sum_and_reduce(&symbols, &mut curr_line);
        previous_line = curr_line.clone();
        curr_line.clear();
        prev_symbols = symbols.clone();
        symbols.clear();
    }
    println!("{}", total);
    Ok(())
}
