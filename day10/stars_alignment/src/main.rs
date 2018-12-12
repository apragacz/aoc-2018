extern crate image;

use std::io;
use std::io::BufRead;
use std::cmp;
use std::env;

#[derive(PartialEq, Eq, Hash, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn add(&self, other: &Point) -> Point {
        return Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
    fn sub(&self, other: &Point) -> Point {
        return Point {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
    fn div(&self, f: i32) -> Point {
        return Point {
            x: self.x / f,
            y: self.y / f,
        };
    }
    fn mul(&self, f: i32) -> Point {
        return Point {
            x: self.x * f,
            y: self.y * f,
        };
    }
}
impl Clone for Point {
    fn clone(&self) -> Point { *self }
}

fn parse_point(string: &str) -> Point {
    let coords: Vec<i32> = string.trim()
        .split(',')
        .map(|el| el.trim())
        .map(|el| el.parse().expect("not an integer"))
        .collect();
    match coords.as_slice() {
        [x, y] => return Point {x: *x, y: *y},
        _ => panic!("invalid coordinates"),
    }
}

fn generate_image(frame_id: usize, top_left: &Point, bottom_right: &Point, positions: &Vec<Point>, stride: usize) {
    let width = ((bottom_right.x - top_left.x) as u32) / stride as u32;
    let height = ((bottom_right.y - top_left.y) as u32) / stride as u32;
    let mut img = image::GrayImage::new(width, height);
    for p in positions.iter() {
        let c = p.sub(top_left).div(stride as i32);
        if 0 <= c.x && c.x < width as i32 && 0 <= c.y && c.y < height as i32 {
            img.put_pixel(c.x as u32, c.y as u32, image::Luma([255u8]));
        }
    }
    img.save(format!("output/out_{:04}.png", frame_id)).unwrap();
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let iterations: usize = if args.len() >= 2 { args[1].parse().expect("not an integer") } else { 1000 };
    let start: usize = if args.len() >= 3 { args[2].parse().expect("not an integer") } else { 0 };
    let stride: usize = if args.len() >= 4 { args[3].parse().expect("not an integer") } else { 1 };

    println!("iterations: {}", iterations);
    println!("start: {}", start);
    println!("stride: {}", stride);

    let stdin = io::stdin();
    let mut positions: Vec<Point> = Vec::new();
    let mut velocities: Vec<Point> = Vec::new();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let segments: Vec<&str> = l.trim().split(|c| c == '<' || c == '>').collect();
        let p = parse_point(segments[1]);
        let v = parse_point(segments[3]);
        positions.push(p);
        velocities.push(v);
    }

    let multipled_velocities: Vec<Point> = velocities.iter().map(|v| v.mul(stride as i32)).collect();

    positions = positions.iter().zip(velocities.iter()).map(|(p, v)| p.add(&v.mul(start as i32))).collect();

    let min_bound = positions.iter().fold(
        positions.get(0).cloned().unwrap(),
        |b, p| Point {x: cmp::min(b.x, p.x), y: cmp::min(b.y, p.y)});
    let max_bound = positions.iter().fold(
        positions.get(0).cloned().unwrap(),
        |b, p| Point {x: cmp::max(b.x, p.x), y: cmp::max(b.y, p.y)});

    println!("{}x{} (unscaled)", max_bound.x - min_bound.x, max_bound.y - min_bound.y);

    for i in 0..iterations {
        println!("Generating image {}", i);
        generate_image(i, &min_bound, &max_bound, &positions, stride);
        positions = positions.iter().zip(multipled_velocities.iter()).map(|(p, v)| p.add(v)).collect();
    }
}
