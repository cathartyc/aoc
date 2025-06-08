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
    fn execute(&self, reg_a: &mut u64, reg_b: &mut u64, reg_c: &mut u64) {
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
            OpCode::Bxc => *reg_b ^= *reg_c,
            OpCode::Out => print!("{},", combo() & 0b0111),
            OpCode::Bdv => *reg_b = *reg_a >> combo(),
            OpCode::Cdv => *reg_c = *reg_a >> combo(),
            _ => (), // Don't care about jumps for part 2
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input17.txt");
    let input = fs::read_to_string(path)?;
    let mut lines = input.lines();
    lines.next();
    lines.next();
    lines.next();
    // Registers
    let mut reg_a: u64 = 0;
    let mut reg_b: u64 = 0;
    let mut reg_c: u64 = 0;

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
    // Program values
    let mut program_values: Vec<u8> = program_line
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();
    // Reverse the program, somehow.
    // Looking at the input, we notice that the program has a fixed
    // structure: OP1 OP2 ... OPN OUT_REG JNZ to the beginning where the
    // output register is fixed.
    //
    // Inspecting the operations (at least in my input), we can see
    // that:
    // - registers B and C are computed each time from register A;
    // - register B starts as A mod 8 (which is just A & 7);
    // - register C is obtained as a right-shift of A n times;
    // - the register A is right-shifted by 3 after the operations;
    // - the output comes from only one register (in my case, B);
    // - the cycle repeats until register A equals 0.
    //
    // We can deduce some details:
    // - The final value of register A is 0 (since the machine halts);
    // - The final value of the printing register is also 0, from the
    //   program values.
    //
    // The final value of A, before the last shift, is
    //      A -> |00...0|ABC|
    // where `ABC` is a 3-bit value, such that, at the end of the last
    // iteration, B & 7 = 0 (which is the last operand in the program).
    // Iterating on ABC from 0 to 7 and executing the program without
    // the last shift gives us the `values` of ABC that match with the
    // output.
    //
    // On the next iteration, we proceed in a similar way, starting from
    //      A -> |00...0ABC|DEF|
    // and finding values for DEF.
    //
    // NB: I said `values`: we have no proof that there is only a single
    // value of ABC which leads to the correct value of B - and indeed
    // this is the case.
    program_values.reverse();

    // Ok. Now we can drop the last three program instructions (shift,
    // print and jump to start)
    program.pop();
    program.pop();
    program.pop();
    // Collecting, for each program digit, possible values of register A
    let mut candidate_values: Vec<(usize, u64)> = vec![];
    let mut i = 0;
    loop {
        // The reason for this to be reversed will be clear after the
        // for loop.
        for a in (0..8).rev() {
            reg_a = (reg_a & !7) | a; // A -> |000...0|a| (a is 3-bit long)
            for instr in &program {
                instr.execute(&mut reg_a, &mut reg_b, &mut reg_c);
            }
            if reg_b & 7 == program_values[i] as u64 {
                candidate_values.push((i, reg_a));
            }
        }
        // It can happen that, in the above iteration, no value has been
        // found: this means that we need to go back to a previous,
        // greater, value of A, which is compatible with the solution.
        //
        // The puzzle requires us to find the _smallest_ initial value
        // of register A such that the output is the program itself.
        // This is why we keep the candidate_values (naturally) sorted
        // in descending order: any pop returns the smallest valid value
        // of A found which has not yet been explored.
        match candidate_values.pop() {
            Some((index, value)) => {
                i = index;
                reg_a = value;
            }
            None => return Err("No value of register A can be found".into()),
        }
        if i == program_values.len() - 1 {
            break;
        }
        i += 1;
        reg_a <<= 3;
    }
    println!("{reg_a}");
    Ok(())
}
