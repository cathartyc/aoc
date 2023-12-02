use std::{fs, u32, char};

fn main() {
    let input: &str = "../inputs/input1";
    let mut sum: u32 = 0;
    for line in fs::read_to_string(input).unwrap().lines() {
        let chars: Vec<char> = line.to_string().chars().collect();
        let mut first: u32 = 0;
        let mut last: u32 = 0;
        let mut index: usize = 0;
        for (i, ch) in chars.iter().enumerate() {
            if ch.is_digit(10) {
                first = ch.to_digit(10).unwrap();
                index = i;
                println!("The first digit is {}", first);
                break;
            }
        }
        println!("There are {} letters to scan", chars.len() - (index + 1));
        for i in (index..chars.len()).rev() {
            if chars[i].is_digit(10) {
                last = chars[i].to_digit(10).unwrap();
                println!("The second is {}", last);
                break;
            } 
        }   
        sum += first*10 + last;
        println!("The actual value now is {}", sum);
    }
    println!("{}", sum);
    assert!(sum == 54597);
}
