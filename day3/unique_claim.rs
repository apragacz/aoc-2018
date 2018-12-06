use std::cmp;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Claim {
    id: u32,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

fn main() {
    let mut claims = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let segments: Vec<&str> = l.trim().split(' ').collect();
        let id = segments[0].trim_start_matches('#').parse().expect("not an integer");
        let xy_str: &str = segments[2].trim_end_matches(':');
        let wh_str: &str = segments[3];
        let xy_segments: Vec<&str> = xy_str.split(',').collect();
        let wh_segments: Vec<&str> = wh_str.split('x').collect();
        let xy: Vec<usize> = xy_segments.into_iter().map(|x| x.parse().expect("not an integer")).collect();
        let wh: Vec<usize> = wh_segments.into_iter().map(|x| x.parse().expect("not an integer")).collect();
        claims.push(Claim {
            id,
            x: xy[0], y: xy[1],
            w: wh[0], h: wh[1],
        });
    }

    let max_w = claims.iter().fold(0, |m, r| cmp::max(m, r.x + r.w));
    let max_h = claims.iter().fold(0, |m, r| cmp::max(m, r.y + r.h));
    let mut parts: Vec<Vec<HashSet<u32>>> = Vec::new();
    parts.resize(max_h, Vec::new());

    for row in &mut parts {
        row.resize(max_w, HashSet::new());
    }

    for c in &claims {
        for i in c.x..(c.x + c.w) {
            for j in c.y..(c.y + c.h) {
                let claim_id: u32 = c.id;
                parts[j][i].insert(claim_id);
            }
        }
    }

    let mut unique_claim_id_candidates: HashSet<u32> = HashSet::new();

    for row in &parts {
        for cell_claim_ids in row {
            if cell_claim_ids.len() == 1 {
                unique_claim_id_candidates.insert(*cell_claim_ids.iter().next().unwrap());
            }
        }
    }

    for row in &parts {
        for cell_claim_ids in row {
            if cell_claim_ids.len() > 1 {
                if !unique_claim_id_candidates.is_disjoint(&cell_claim_ids) {
                    unique_claim_id_candidates = unique_claim_id_candidates.difference(&cell_claim_ids).cloned().collect();
                }
            }
        }
    }

    for id in unique_claim_id_candidates {
        println!("{}", id);
    }


}
