use std::cmp;
use std::io;

use std::io::BufRead;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum ParsePointError{
    InvalidFormat(String),
    InvalidNumber(String),
}

#[derive(Debug, Clone)]
struct Point {
    coords: [i32; 4],
}
impl Point {
    fn dist(&self, other: &Self) -> usize {
        let mut d = 0;
        for i in 0..4 {
            d += (self.coords[i] - other.coords[i]).abs() as usize;
        }
        return d;
    }
    fn parse_int<T: FromStr>(s: &str) -> Result<T, ParsePointError> {
        match s.parse() {
            Ok(m) => Ok(m),
            Err(_) => Err(ParsePointError::InvalidNumber(s.to_string())),
        }
    }
}
impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Vec<_> = s.split(',').collect();
        let coords: Vec<Result<i32, ParsePointError>> = segments.iter().map(|seg| Self::parse_int(seg)).collect();
        if coords.len() != 4 {
            return Err(ParsePointError::InvalidFormat(s.to_string()))
        }
        return Ok(
            Point {
                coords: [
                    coords[0].clone()?,
                    coords[1].clone()?,
                    coords[2].clone()?,
                    coords[3].clone()?,
                ]
            }
        );
    }
}

struct FindUnion {
    parent_indexes: Vec<usize>,
    depths: Vec<usize>,
}
impl FindUnion {
    fn new(n: usize) -> Self {
        let mut parent_indexes = Vec::new();
        parent_indexes.reserve(n);
        for i in 0..n {
            parent_indexes.push(i);
        }
        let mut depths = Vec::new();
        depths.resize(n, 0);
        return FindUnion {
            parent_indexes,
            depths,
        };
    }

    fn find_root(&self, x: usize) -> usize {
        let mut next_x = x;
        while self.parent_indexes[next_x] != next_x {
            next_x = self.parent_indexes[next_x];
        }
        return next_x;
    }

    fn find_root_and_compress_path(&mut self, x: usize) -> usize {
        if self.parent_indexes[x] == x {
            return x;
        }
        let parent = self.parent_indexes[x];
        let root = self.find_root_and_compress_path(parent);
        self.parent_indexes[x] = root;
        self.depths[x] = 0;
        return root;
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find_root_and_compress_path(x);
        let root_y = self.find_root_and_compress_path(y);
        if self.depths[root_x] > self.depths[root_y] {
            // x is the root
            self.parent_indexes[root_y] = root_x;
            self.depths[root_x] = cmp::max(self.depths[root_x], self.depths[root_y] + 1);
        } else {
            // y is the root
            self.parent_indexes[root_x] = root_y;
            self.depths[root_y] = cmp::max(self.depths[root_x] + 1, self.depths[root_y]);
        }
    }
}

fn read_input() -> Vec<Point> {
    let stdin = io::stdin();
    let mut points = Vec::new();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let point_str = l.trim();
        let p: Point = point_str.parse().expect("Could not read line");
        points.push(p);
    }
    return points;
}

fn main() {
    let points = read_input();
    let n = points.len();
    let mut find_union = FindUnion::new(n);
    for i in 0..n {
        for j in (i + 1)..n {
            if points[i].dist(&points[j]) <= 3 {
                find_union.union(i, j);
            }
        }
    }
    let mut roots: HashSet<usize> = HashSet::new();
    for i in 0..n {
        roots.insert(find_union.find_root_and_compress_path(i));
    }
    println!("{}", roots.len());
}
