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
    components: Vec<u32>,
}

fn make_serial(val: String, last_pos: usize) -> Component {
    Component { 
        value: val.parse().expect("Should be a number here"), 
        start: last_pos - val.len(),
        end: last_pos - 1 }
}

fn add_components(symbols: &mut [Symbol], components: &[Component]) {
    for symbol in symbols.iter_mut() {
        symbol.components.extend(components
            .iter()
            .filter(|&c| symbol.start <= c.end && symbol.end >= c.start)
            .map(|c| c.value)
        );
    }
}

fn main() -> Result<(), Error> {
    let input = File::open("../inputs/input3")?;
    let input = BufReader::new(input);
    let mut previous_line = Vec::<Component>::new();
    let mut curr_line: Vec<Component> = Vec::new(); 
    let mut prev_symbols = Vec::<Symbol>::new();
    let mut curr_symbols = Vec::<Symbol>::new();
    let mut valid_gears = Vec::<Symbol>::new();
    let mut val: String = "".to_string();
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
            } else if ch == '*' {
                curr_symbols.push(Symbol { start: i-1, end: i+1, components: Vec::new() });
            }
        }
        if !val.is_empty() {
            curr_line.push(make_serial(val.clone(), line.unwrap().len()));
            val.clear();
        }
        add_components(&mut prev_symbols, &curr_line);
        valid_gears.extend(prev_symbols
                           .iter()
                           .filter(|s| s.components.len() == 2)
                           .cloned()
        );
        add_components(&mut curr_symbols, &previous_line);
        add_components(&mut curr_symbols, &curr_line);
        previous_line = curr_line.clone();
        curr_line.clear();
        prev_symbols = curr_symbols.clone();
        curr_symbols.clear();
    }
    let total: u32 = valid_gears
        .iter()
        .map(|s| s.components
             .iter()
             .product::<u32>()
             )
        .sum();
    println!("{}", total);
    Ok(())
}
