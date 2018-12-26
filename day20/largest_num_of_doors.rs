use std::cmp;
use std::fmt;
use std::io;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::Read;

type Coord = i32;
type Point = (Coord, Coord);

trait PointOps {
    fn add(&self, other: &Point) -> Point;
}
impl PointOps for Point {

    fn add(&self, other: &Point) -> Point {
        let (sx, sy) = self;
        let (ox, oy) = other;
        return (*sx + *ox, *sy + *oy);
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
enum Direction {
    N,
    W,
    S,
    E,
}
impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'N' => Direction::N,
            'W' => Direction::W,
            'S' => Direction::S,
            'E' => Direction::E,
            _ => panic!("direction not recognized: {}", c),
        }
    }
    fn to_delta_point(&self) -> Point {
        match self {
            Direction::N => (0, -1),
            Direction::W => (-1, 0),
            Direction::S => (0, 1),
            Direction::E => (1, 0),
        }
    }
    fn invert(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::W => Direction::E,
            Direction::S => Direction::N,
            Direction::E => Direction::W,
        }
    }
}

struct Field {
    doors: HashSet<Direction>,
}
impl Field {
    fn new() -> Field {
        Field { doors: HashSet::new() }
    }
    fn add_door(&mut self, direction: Direction) {
        self.doors.insert(direction);
    }
    fn has_door(&self, direction: Direction) -> bool {
        self.doors.contains(&direction)
    }
}

struct Map {
    fields: HashMap<Point, Field>,
    default_field: Field,
}
impl Map {
    fn new() -> Map {
        Map {
            fields: HashMap::new(),
            default_field: Field::new(),
        }
    }
    fn lower_bound(&self) -> Point {
        let x = self.fields.keys().fold(0, |m, (elx, _)| cmp::min(m, *elx));
        let y = self.fields.keys().fold(0, |m, (_, ely)| cmp::min(m, *ely));
        (x, y)
    }
    fn upper_bound(&self) -> Point {
        let x = self.fields.keys().fold(0, |m, (elx, _)| cmp::max(m, *elx));
        let y = self.fields.keys().fold(0, |m, (_, ely)| cmp::max(m, *ely));
        (x, y)
    }
    fn get_field(&self, point: &Point) -> &Field {
        self.fields.get(point).unwrap_or(&self.default_field)
    }
    fn move_to(&mut self, point: &Point, direction: Direction) {
        let next_point = point.add(&direction.to_delta_point());
        {
            let field = self.fields.entry(*point)
                .or_insert(Field::new());
            field.add_door(direction.clone());
        }
        {
            let next_field = self.fields.entry(next_point)
                .or_insert(Field::new());
            next_field.add_door(direction.invert());
        }
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (minx, miny) = self.lower_bound();
        let (maxx, maxy) = self.upper_bound();

        let mut buffer = String::new();

        for y in miny..=maxy {
            if y == miny {
                buffer.push('#');
                for _ in minx..=maxx {
                    buffer.push_str("##");
                }
                buffer.push('\n');
            }
            buffer.push('#');
            for x in minx..=maxx {
                let field = self.get_field(&(x, y));
                buffer.push('.');
                if field.has_door(Direction::E) {
                    buffer.push('|');
                } else {
                    buffer.push('#');
                }
            }
            buffer.push('\n');
            buffer.push('#');
            for x in minx..=maxx {
                let field = self.get_field(&(x, y));
                if field.has_door(Direction::S) {
                    buffer.push('-');
                } else {
                    buffer.push('#');
                }
                buffer.push('#');
            }
            buffer.push('\n');
        }
        writeln!(f, "{}", buffer)
    }
}

trait StackExtra<T> {
    fn top(&self) -> Option<T>;
}

impl StackExtra<Point> for Vec<Point> {
    fn top(&self) -> Option<Point> {
        self.last().cloned()
    }
}

fn build_map(direction_regex: &String) -> Map {
    let mut map = Map::new();
    let mut stack: Vec<Point> = Vec::new();
    let mut point: Point = (0, 0);
    for c in direction_regex.chars() {
        match c {
            '^' => {},
            '$' => {},
            '(' => {
                stack.push(point.clone());
            },
            '|' => {
                point = stack.top().unwrap();
            }
            ')' => {
                stack.pop();
            }
            '\n' => {},
            _ => {
                let direction = Direction::from_char(c);
                let delta = direction.to_delta_point();
                map.move_to(&point, direction);
                point = point.add(&delta);
            }
        }
    }
    return map;
}

fn get_furthest_room(map: &Map) -> usize {
    let mut distances: HashMap<Point, usize> = HashMap::new();
    let mut q = VecDeque::new();

    q.push_back(((0, 0), 0));

    while !q.is_empty() {
        let (p, dist) = q.pop_front().unwrap();
        let field = map.get_field(&p);
        match distances.get(&p).cloned() {
            None => {
                distances.insert(p.clone(), dist);
                for direction in &field.doors {
                    let next_p = p.add(&direction.to_delta_point());
                    q.push_back((next_p, dist + 1));
                }
            },
            Some(_) => {
                continue;
            }
        }
    }

    distances.values().fold(0, |acc, d| cmp::max(acc, *d))
}

fn main() {
    let mut regex = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut regex)
        .expect("reading stdin failed");
    let map = build_map(&regex);
    println!("{:?}", map);
    println!("{}", get_furthest_room(&map));
}

