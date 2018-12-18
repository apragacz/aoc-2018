use std::io;
use std::io::BufRead;
use std::collections::HashSet;

type Registers = [usize; 4];
type InstructionCode = [usize; 4];

fn vec_to_array(vec: &Vec<usize>) -> [usize; 4] {
    match vec.as_slice() {
        [a, b, c, d] => [*a, *b, *c, *d],
        _ => panic!("invalid data"),
    }
}

fn parse_registers(string: &String, prefix: &str) -> Registers {
    let vec: Vec<usize> = string
        .trim_start_matches(prefix)
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(' ')
        .map(|s| s.trim_end_matches(','))
        .map(|s| s.parse().expect("not an integer"))
        .collect();
    return vec_to_array(&vec);
}

fn parse_instruction_code(string: &String) -> InstructionCode {
    let vec: Vec<usize> = string
        .split(' ')
        .map(|s| s.parse().expect("not an integer"))
        .collect();
    return vec_to_array(&vec);
}

fn line_option_is_empty(line_input: &Option<String>) -> bool {
    match line_input {
        None => true,
        Some(s) => s.is_empty(),
    }
}

fn line_input_to_option(line_input: Option<Result<String, io::Error>>) -> Option<String> {
    match line_input {
        None => None,
        Some(s) => Some(s.expect("no data")),
    }
}

fn is_valid_reg(value: usize) -> bool {
    value < 4
}

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
impl OpCode {
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
    fn eval(&self, code: &InstructionCode, reg: &Registers) -> Option<Registers> {
        let [_, a, b, c] = code;
        if self.is_a_register() && !is_valid_reg(*a) {
            return None;
        }
        if self.is_b_register() && !is_valid_reg(*b) {
            return None;
        }
        let av: usize = if self.is_a_register() { reg[*a] } else { *a };
        let bv: usize = if self.is_b_register() { reg[*b] } else { *b };
        let cv: usize = self.operator(av, bv);
        let mut result: Registers = [0; 4];
        result.clone_from_slice(reg);
        result[*c] = cv;
        return Some(result);
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


fn main() {
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock().lines();
    let mut counter = 0;
    let op_codes = OpCode::values();
    loop {
        let before_line_opt = line_input_to_option(lines_iter.next());
        let codes_opt = line_input_to_option(lines_iter.next());
        let after_line_opt = line_input_to_option(lines_iter.next());
        lines_iter.next();
        if line_option_is_empty(&before_line_opt) {
            break;
        }
        if line_option_is_empty(&codes_opt) {
            break;
        }
        if line_option_is_empty(&after_line_opt) {
            break;
        }
        let reg_before = parse_registers(&before_line_opt.unwrap(), "Before:");
        let instruction_code = parse_instruction_code(&codes_opt.unwrap());
        let reg_after = parse_registers(&after_line_opt.unwrap(), "After:");

        let mut op_code_match_counter = 0;

        for op_code in &op_codes {
            match op_code.eval(&instruction_code, &reg_before) {
                None => {}
                Some(reg_output) => {
                    if reg_output == reg_after {
                        op_code_match_counter += 1;
                    }
                }
            }
        }

        if op_code_match_counter >= 3 {
            counter += 1;
        }
    }
    println!("{}", counter);
}
