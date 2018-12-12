use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let mut increments = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let segment = l.trim();
        let incr: i32 = segment.parse()
            .expect("Not a number!");
        increments.push(incr);
    }

    let mut freqs = HashSet::new();
    let mut first_freq_twice = None;
    let mut sum: i32 = 0;
    freqs.insert(0);
    loop {
        for incr in &increments {
            sum += incr;

            if freqs.contains(&sum) {
                first_freq_twice = Some(sum);
                break;
            }
            freqs.insert(sum);
        }
        match first_freq_twice {
            Some(_) => break,
            None => {},
        }
    }
    match first_freq_twice {
        Some(v) => println!("{}", v),
        None => {},
    }
}
