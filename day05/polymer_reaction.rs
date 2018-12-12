use std::io;
use std::io::Read;

fn is_reacting(unit1: char, unit2: char) -> bool {
    if unit1 == unit2 {
        return false;
    }
    if unit1.to_ascii_uppercase() == unit2.to_ascii_uppercase() {
        return true;
    }
    return false;
}

fn main() {
    let mut polymer_units = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut polymer_units).expect("reading file failed");
    polymer_units = polymer_units.trim().to_string();

    let mut stack: Vec<char> = Vec::new();

    for unit in polymer_units.chars() {
        let stack_top = stack.last().cloned();
        match stack_top {
            Some(u) => {
                if is_reacting(u, unit) {
                    stack.pop();
                } else {
                    stack.push(unit);
                }
            },
            None => {
                stack.push(unit);
            },
        }
    }

    println!("{}", stack.len());
}
