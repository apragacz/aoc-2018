use std::io;
use std::io::BufRead;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

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
    fn dist(&self, other: &Point) -> usize {
        return ((self.x - other.x).abs() as usize) + ((self.y - other.y).abs() as usize);
    }
}
impl Clone for Point {
    fn clone(&self) -> Point { *self }
}

#[derive(Debug)]
struct Location {
    id: usize,
    point: Point,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum MapCell {
    OneLocation(usize, usize),  // id, dist
    MultipleLocations(usize),
}

fn main() {
    let stdin = io::stdin();
    let mut points: Vec<Point> = Vec::new();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let coords: Vec<i32> = l.trim()
            .split(' ')
            .map(|el| el.trim_end_matches(','))
            .map(|el| el.parse().expect("not an integer"))
            .collect();
        match coords.as_slice() {
            [x, y] => points.push(Point {x: *x, y: *y}),
            _ => panic!("invalid coordinates"),
        }
    }

    let min_bound = points.iter().fold(
        points.get(0).cloned().unwrap(),
        |b, p| Point {x: cmp::min(b.x, p.x), y: cmp::min(b.y, p.y)});
    let max_bound = points.iter().fold(
        points.get(0).cloned().unwrap(),
        |b, p| Point {x: cmp::max(b.x, p.x), y: cmp::max(b.y, p.y)});
    let max_dist = min_bound.dist(&max_bound);
    let mut locations = Vec::new();
    for (i, p) in points.iter().enumerate() {
        locations.push(Location {
            id: i,
            point: *p,
        });
    }

    let mut map: HashMap<Point, MapCell> = HashMap::new();

    let mut infinite_location_ids: HashSet<usize> = HashSet::new();

    let deltas = [
        Point{x: -1, y: 0},
        Point{x: 0, y: -1},
        Point{x: 1, y: 0},
        Point{x: 0, y: 1},
    ];

    let mut q = VecDeque::new();

    for l in &locations {
        q.push_back((l.id, l.point, 0));
    }

    while !q.is_empty() {
        let (loc_id, p, dist) = q.pop_front().unwrap();

        let mut propagate = true;

        let final_loop = dist > max_dist / 2 + 2;

        if final_loop {
            propagate = false;
        }

        match map.get(&p).cloned(){
            None => {
                if final_loop {
                    infinite_location_ids.insert(loc_id);
                }
                map.insert(p, MapCell::OneLocation(loc_id, dist));
            },
            Some(MapCell::OneLocation(old_loc_id, old_dist)) => {
                if old_dist == dist && old_loc_id != loc_id {
                    map.insert(p, MapCell::MultipleLocations(dist));
                } else {
                    propagate = false;
                }
            },
            Some(MapCell::MultipleLocations(_)) => {
                propagate = false;
            },
        }

        if !propagate {
            continue;
        }

        for delta in &deltas {
            let next_point = p.add(delta);
            q.push_back((loc_id, next_point, dist + 1));
        }
    }

    let mut location_areas = HashMap::new();

    for (_, cell) in &map {
        match *cell {
            MapCell::MultipleLocations(_) => {},
            MapCell::OneLocation(loc_id, _) => {
                if !infinite_location_ids.contains(&loc_id) {
                    let counter = location_areas.entry(loc_id).or_insert(0);
                    *counter += 1;
                }
            },
        }
    }

    let max_area = location_areas.values().fold(0, |m, x| cmp::max(m, *x));

    println!("{}", max_area);
}
