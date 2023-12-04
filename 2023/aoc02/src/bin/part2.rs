use std::fs;

fn main() {
    let text = fs::read_to_string("../inputs/input2").unwrap();
    let mut max: Vec<usize>;
    let mut power_sum: usize = 0;
    let color = ["red", "green", "blue"];
    for line in text.lines() {
        max = vec![0, 0, 0];
        let extractions = line.split(": ").last().unwrap().split("; ");
        for ext in extractions {
            let div = ext.split(", ");
            for comb in div {
                let mut comb = comb.split(" ");
                let amount = comb.next();
                let amount: usize = amount.unwrap().parse().expect("Not a number");
                let col: &str = comb.next().unwrap();
                let index: usize = color
                    .iter()
                    .position(|&c| c == col)
                    .expect("It should be here")
                    .try_into()
                    .unwrap();
                if amount > max[index] {
                    max[index] = amount;
                }
            }
        }
        power_sum += max.iter().copied().reduce(|a, b| a * b).unwrap();
    }
    println!("{}", power_sum);
}
