use std::io;

use std::io::BufRead;

type Coord = i32;
type Point = (Coord, Coord, Coord);

trait PointOps {
    fn dist(&self, other: &Point) -> usize;
    fn sub(&self, other: &Point) -> Point;
    fn norm(&self) -> usize;
}

impl PointOps for Point {
    fn dist(&self, other: &Point) -> usize {
        self.sub(other).norm()
    }
    fn sub(&self, other: &Point) -> Point {
        (self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
    fn norm(&self) -> usize {
        (self.0.abs() + self.1.abs() + self.2.abs()) as usize
    }
}

#[derive(Debug)]
struct NanoBot {
    position: Point,
    radius: usize,
}

fn read_input() -> Vec<NanoBot> {
    let stdin = io::stdin();
    let mut bots = Vec::new();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let segments: Vec<_> = l.split(", ").collect();
        let pos_segments: Vec<_> = segments[0].split('=').collect();
        let radius_segments: Vec<_> = segments[1].split('=').collect();
        let coords: Vec<Coord> = pos_segments[1]
            .trim_start_matches('<')
            .trim_end_matches('>')
            .split(',')
            .map(|s| s.parse().expect("coord not a number"))
            .collect();
        let radius = radius_segments[1].parse().expect("radius not a number");
        match coords.as_slice() {
            [x, y, z] => {
                bots.push(NanoBot {position: (*x, *y, *z), radius});
            },
            _ => panic!("invalid number of coords")
        }
    }
    return bots;
}

fn main() {
    let bots = read_input();

    let mut largest_radius_bot_opt: Option<&NanoBot> = None;

    for bot in &bots {
        largest_radius_bot_opt = match largest_radius_bot_opt {
            None => Some(bot),
            Some(b) => if bot.radius > b.radius { Some(bot) } else { Some(b) }
        }
    }

    let largest_radius_bot = largest_radius_bot_opt.unwrap();

    let mut counter = 0;

    for bot in &bots {
        if largest_radius_bot.position.dist(&bot.position) <= largest_radius_bot.radius {
            counter += 1;
        }
    }
    println!("{}", counter);
}
