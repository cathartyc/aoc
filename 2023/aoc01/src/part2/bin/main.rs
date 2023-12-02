use std::{fs, u32, char};

fn main() {
    let input: &str = "../inputs/input1";
    let numbers = vec!["baguette", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut sum: u32 = 0;
    for line in fs::read_to_string(input).unwrap().lines() {
        let lstr: String = line.to_string();
        println!("The string is {}", lstr);
        let chars: Vec<char> = lstr.chars().collect();
        let mut first: u32 = 0;
        let mut last: u32 = 0;
        let mut index: usize = 0;
        let mut lit_pos_first: usize = usize::MAX;
        let mut lit_pos_first_val: usize = 11;
        let mut lit_pos_last: usize = 0;
        let mut lit_pos_last_val: usize = 11;
        for lit in numbers.iter() {
            let positions: Vec<(usize,&str)> = lstr.match_indices(lit).collect();
            if positions.len() > 0 {
                if positions[0].0 < lit_pos_first {
                    lit_pos_first = positions[0].0;
                    lit_pos_first_val = numbers.iter().position(|&n| n == lit.to_string()).unwrap();
                }
                if positions[positions.len()-1].0 >= lit_pos_last {
                    lit_pos_last = positions[positions.len()-1].0;
                    lit_pos_last_val = numbers.iter().position(|&n| n == lit.to_string()).unwrap();
                }
            }
        }
        if lit_pos_last_val != 11 {
            lit_pos_last = lit_pos_last + numbers.get(lit_pos_last_val).unwrap().len() - 1;
        }
        for (i, ch) in chars.iter().enumerate() {
            if ch.is_digit(10) {
                first = ch.to_digit(10).unwrap();
                index = i;
                break;
            }
        }
        first = if index < lit_pos_first || lit_pos_first_val == 11 {first} else {lit_pos_first_val.try_into().unwrap()};
        println!("The first digit is {}", first);
        for i in (index..chars.len()).rev() {
            if chars[i].is_digit(10) {
                last = chars[i].to_digit(10).unwrap();
                index = i;
                break;
            } 
        }   
        last = if index > lit_pos_last || lit_pos_last_val == 11 {last} else {lit_pos_last_val.try_into().unwrap()};
        println!("The second is {}", last);
        sum += first*10 + last;
        println!("The actual value now is {}", sum);
    }
    println!("{}", sum);
}
