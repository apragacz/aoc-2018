use std::io;
use std::cmp;
use std::io::prelude::*;

#[derive(Debug)]
struct Rect {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

fn main() {
    let mut rects = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let segments: Vec<&str> = l.trim().split(' ').collect();
        let xy_str: &str = segments[2].trim_end_matches(':');
        let wh_str: &str = segments[3];
        let xy_segments: Vec<&str> = xy_str.split(',').collect();
        let wh_segments: Vec<&str> = wh_str.split('x').collect();
        let xy: Vec<usize> = xy_segments.into_iter().map(|x| x.parse().expect("not an integer")).collect();
        let wh: Vec<usize> = wh_segments.into_iter().map(|x| x.parse().expect("not an integer")).collect();
        rects.push(Rect {
            x: xy[0], y: xy[1],
            w: wh[0], h: wh[1],
        });
    }

    let max_w = rects.iter().fold(0, |m, r| cmp::max(m, r.x + r.w));
    let max_h = rects.iter().fold(0, |m, r| cmp::max(m, r.y + r.h));
    let mut parts = Vec::new();
    parts.resize(max_h, Vec::new());

    for row in &mut parts {
        row.resize(max_w, 0);
    }

    for r in &rects {
        for i in r.x..(r.x + r.w) {
            for j in r.y..(r.y + r.h) {
                parts[j][i] += 1
            }
        }
    }

    let mut counter = 0;
    for row in &parts {
        for cell in row {
            if *cell >= 2 {
                counter += 1;
            }
        }
    }

    println!("{}", counter);
}
