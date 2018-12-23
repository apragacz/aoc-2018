use std::io;

use std::collections::HashSet;
use std::io::BufRead;

type Registers = [usize; 6];

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum OpCode{
    AddR,
    AddI,
    MulR,
    MulI,
    BAnR,
    BAnI,
    BOrR,
    BOrI,
    SetR,
    SetI,
    GTIR,
    GTRI,
    GTRR,
    EqIR,
    EqRI,
    EqRR,
}

type InstructionCode = (OpCode, usize, usize, usize);

impl OpCode {
    fn from_str(string: &str) -> OpCode {
        let string_lo = string.to_lowercase();
        for candidate in OpCode::values() {
            let candidate_string = format!("{:?}", candidate);
            if string_lo == candidate_string.to_lowercase() {
                return candidate;
            }
        }
        panic!("unrecognized opcode {}", string);
    }
    fn operator(&self, a: usize, b: usize) -> usize {
        match self {
            OpCode::AddR | OpCode::AddI => a + b,
            OpCode::MulR | OpCode::MulI => a * b,
            OpCode::BAnR | OpCode::BAnI => a & b,
            OpCode::BOrR | OpCode::BOrI => a | b,
            OpCode::SetR | OpCode::SetI => a,
            OpCode::GTIR | OpCode::GTRI | OpCode::GTRR => if a > b { 1 } else { 0 },
            OpCode::EqIR | OpCode::EqRI | OpCode::EqRR => if a == b { 1 } else { 0 },
        }
    }
    fn is_b_register(&self) -> bool {
        match self {
            OpCode::AddR | OpCode::MulR |
            OpCode::BAnR | OpCode::BOrR |
            OpCode::SetR | OpCode::GTIR | OpCode::GTRR | OpCode::EqIR | OpCode::EqRR => true,
            _ => false,
        }
    }
    fn is_a_register(&self) -> bool {
        match self {
            OpCode::SetI | OpCode::GTIR | OpCode::EqIR => false,
            _ => true,
        }
    }
    fn eval(&self, code: &InstructionCode, reg: &Registers) -> Result<Registers, &'static str> {
        let (_, a, b, c) = code;
        if self.is_a_register() && !is_valid_reg(*a) {
            return Err("a is invalid register");
        }
        if self.is_b_register() && !is_valid_reg(*b) {
            return Err("a is invalid register");
        }
        let av: usize = if self.is_a_register() { reg[*a] } else { *a };
        let bv: usize = if self.is_b_register() { reg[*b] } else { *b };
        let cv: usize = self.operator(av, bv);
        let mut result: Registers = [0; 6];
        result.clone_from_slice(reg);
        result[*c] = cv;
        return Ok(result);
    }
    fn values() -> HashSet<OpCode> {
        vec![
            OpCode::AddR, OpCode::AddI, OpCode::MulR , OpCode::MulI,
            OpCode::BAnR, OpCode::BAnI, OpCode::BOrR , OpCode::BOrI,
            OpCode::SetR, OpCode::SetI,
            OpCode::GTIR, OpCode::GTRI, OpCode::GTRR,
            OpCode::EqIR, OpCode::EqRI, OpCode::EqRR,
        ].iter().cloned().collect()

    }
}

fn is_valid_reg(value: usize) -> bool {
    value < 6
}

fn parse_ip_index(string: &String) -> usize {
    let segments: Vec<&str> = string
        .split(' ')
        .collect();
    assert!(segments.len() == 2);
    return segments[1].parse().expect("not an integer");
}

fn parse_instruction_code(string: &String) -> InstructionCode {
    let segments: Vec<&str> = string
        .split(' ')
        .collect();
    assert!(segments.len() == 4);

    let op_code = OpCode::from_str(&segments[0]);

    let values: Vec<usize> = segments.iter()
        .skip(1)
        .map(|s| s.parse().expect("not an integer"))
        .collect();
    assert!(values.len() == 3);
    return (op_code, values[0], values[1], values[2]);
}

fn main() {
    let stdin = io::stdin();
    let mut instructions = Vec::new();
    let mut ip_reg_index = 0;
    for (i, line) in stdin.lock().lines().enumerate() {
        let l = line.unwrap();
        if i == 0 {
            ip_reg_index = parse_ip_index(&l);
        } else {
            instructions.push(parse_instruction_code(&l));
        }
    }

    let mut ip: usize = 0;
    let mut registers: Registers = [0; 6];

    loop {
        if ip >= instructions.len() {
            break;
        }
        registers[ip_reg_index] = ip;
        let instruction = &instructions[ip];
        let (op_code, _, _, _) = instruction;
        registers = op_code.eval(instruction, &registers)
            .expect("instruction eval failed");
        ip = registers[ip_reg_index];
        ip += 1;
    }
    println!("{}", registers[0]);
}
