use std::io;
use std::io::prelude::*;

fn main() {
    let mut sum: i32 = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let segment = l.trim();
        let change: i32 = segment.parse()
            .expect("Not a number!");
        sum += change;
    }
    println!("{}", sum);
}
