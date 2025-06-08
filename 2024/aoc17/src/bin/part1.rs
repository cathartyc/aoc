use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl OpCode {
    fn from_code(code: u8) -> Result<Self, Box<dyn Error>> {
        match code {
            0 => Ok(OpCode::Adv),
            1 => Ok(OpCode::Bxl),
            2 => Ok(OpCode::Bst),
            3 => Ok(OpCode::Jnz),
            4 => Ok(OpCode::Bxc),
            5 => Ok(OpCode::Out),
            6 => Ok(OpCode::Bdv),
            7 => Ok(OpCode::Cdv),
            _ => Err("OP code {code} not valid".into()),
        }
    }
}

struct Instr {
    op_code: OpCode,
    operand: u8,
}

impl Instr {
    fn execute(&self, reg_a: &mut u64, reg_b: &mut u64, reg_c: &mut u64, reg_ip: &mut u64) {
        let combo = || match self.operand {
            0..=3 => self.operand as u64,
            4 => *reg_a,
            5 => *reg_b,
            6 => *reg_c,
            _ => unimplemented!(),
        };
        match self.op_code {
            OpCode::Adv => *reg_a >>= combo(),
            OpCode::Bxl => *reg_b ^= self.operand as u64,
            OpCode::Bst => *reg_b = combo() & 0b0111,
            OpCode::Jnz if *reg_a != 0 => *reg_ip = (self.operand / 2) as u64,
            OpCode::Jnz => *reg_ip += 1,
            OpCode::Bxc => *reg_b ^= *reg_c,
            OpCode::Out => print!("{},", combo() & 0b0111),
            OpCode::Bdv => *reg_b = *reg_a >> combo(),
            OpCode::Cdv => *reg_c = *reg_a >> combo(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input17.txt");
    let input = fs::read_to_string(path)?;
    let mut lines = input.lines();
    // Registers
    let mut reg_a: u64 = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();
    let mut reg_b: u64 = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();
    let mut reg_c: u64 = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();
    // Instruction pointer
    let mut reg_ip: u64 = 0;

    // Blank line
    lines.next();

    // Program
    let re = Regex::new(r"(?<op>\d),(?<operand>\d),?").unwrap();
    let program_line = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1];
    let mut program: Vec<Instr> = vec![];
    for (_, [op, operand]) in re.captures_iter(program_line).map(|c| c.extract()) {
        program.push(Instr {
            op_code: OpCode::from_code(op.parse().unwrap())?,
            operand: operand.parse().unwrap(),
        });
    }

    // Execute the program
    while let Some(instruction) = program.get(reg_ip as usize) {
        instruction.execute(&mut reg_a, &mut reg_b, &mut reg_c, &mut reg_ip);
        if instruction.op_code != OpCode::Jnz {
            reg_ip += 1;
        }
    }
    Ok(())
}
