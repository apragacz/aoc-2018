use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn has_n_chars(str: &str, n: usize) -> bool {
    let mut chars_counters = HashMap::new();
    for ch in str.chars() {
        let counter = chars_counters.entry(ch).or_insert(0);
        *counter += 1
    }

    for (_, k) in &chars_counters {
        if *k == n {
            return true;
        }
    }
    return false;
}


fn main() {
    let mut two_cnt: usize = 0;
    let mut three_cnt: usize = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let segment = l.trim();
        if has_n_chars(segment, 2) {
            two_cnt += 1;
        }
        if has_n_chars(segment, 3) {
            three_cnt += 1;
        }
    }
    let checksum = two_cnt * three_cnt;
    println!("{}", checksum);
}
