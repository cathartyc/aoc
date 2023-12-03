use std::fs;

fn main() {
    let text = fs::read_to_string("../inputs/input2").unwrap();
    let mut game_id = 0;
    let mut id_sum = 0;
    let mut too_much: bool;
    let max = [12, 13, 14];
    let color = ["red", "green", "blue"];
    for line in text.lines() {
        game_id += 1;
        too_much = false;
        let extractions = line.split(": ").last().unwrap().split("; ");
        'out: for ext in extractions {
            let div = ext.split(", ");
            for comb in div {
                let mut comb = comb.split(" ");
                let amount = comb.next().clone();
                let amount: usize = amount.unwrap().parse().expect("Not a number");
                let col: &str = comb.next().unwrap();
                if amount > max[color.iter().position(|&c| c == col).expect("It should be here")] {
                    too_much = true;
                    break 'out;
                }
            } 
        }
        if !too_much {
            id_sum += game_id;
        }

    }
    println!("{}", id_sum);
}
