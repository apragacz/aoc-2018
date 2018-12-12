use std::io;
use std::io::prelude::*;

fn common_letters(str1: &str, str2: &str) -> Option<String> {
    let mut common_letters = String::new();
    let mut mismatch_counter: usize = 0;
    if str1.len() != str2.len() {
        return None;
    }
    for (ch1, ch2) in str1.chars().zip(str2.chars()) {
        if ch1 == ch2 {
            common_letters.push(ch1);
        } else {
            mismatch_counter += 1;
        }
    }

    if mismatch_counter > 1 {
        return None;
    }

    return Some(common_letters);
}

fn main() {
    let mut segments = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let segment = String::from(l.trim());
        segments.push(segment);
    }

    for (i1, segment1) in segments.iter().enumerate() {
        for (i2, segment2) in segments.iter().enumerate() {
            if i1 >= i2 {
                continue;
            }
            match common_letters(segment1, segment2) {
                Some(cl) => {
                    println!("{}", cl);
                    break;
                },
                None => {},
            }
        }
    }
}
