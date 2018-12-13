use std::io;
use std::io::Read;
use std::cmp;
use std::collections::HashSet;

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

    let available_chars: HashSet<char> = polymer_units.chars().map(|u| u.to_ascii_uppercase()).collect();

    let mut min_reduced_len = polymer_units.len();

    for blocking_char in available_chars.iter() {
        let mut stack: Vec<char> = Vec::new();
        let polymer_units_without_blocking_char = polymer_units.replace(|c| c == *blocking_char || c == blocking_char.to_ascii_lowercase(), "");

        for unit in polymer_units_without_blocking_char.chars() {
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

        min_reduced_len = cmp::min(min_reduced_len, stack.len());
    }
    println!("{}", min_reduced_len);

}
