use std::io;
use std::fmt;

use std::io::BufRead;

type Matrix<T> = Vec<Vec<T>>;
type Point = (usize, usize);

#[derive(Eq, PartialEq, Clone)]
enum RegionType {
    Rocky,
    Narrow,
    Wet,
}
impl RegionType {
    fn to_char(&self) -> char {
        match self {
            RegionType::Rocky => '.',
            RegionType::Narrow => '|',
            RegionType::Wet => '=',
        }
    }
}
impl fmt::Debug for RegionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn create_matrix<T: Clone>(num_of_rows: usize, num_of_cols: usize, value: &T) -> Matrix<T> {
    let mut mat = Vec::new();
    mat.resize(num_of_rows, Vec::new());
    for row in &mut mat {
        row.resize(num_of_cols, value.clone());
    }
    return mat;
}

fn get_matrix_dimensions<T>(mat: &Matrix<T>) -> (usize, usize) {
    if mat.is_empty() {
        return (0, 0);
    } else {
        return (mat.len(), mat[0].len());
    }
}

fn print_debug_matrix<T: fmt::Debug>(mat: &Matrix<T>) {
    for row in mat {
        for cell in row {
            print!("{:?} ", *cell);
        }
        println!("");
    }
}

fn read_input() -> (usize, Point) {
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    let depth: usize;
    let mut target: Point = (0, 0);

    {
        let first_line =  line_iter.next().unwrap().unwrap();
        let first_line_segments: Vec<_> = first_line.split(": ").collect();
        depth = first_line_segments[1].trim().parse().expect("not a number");
    }
    {
        let second_line =  line_iter.next().unwrap().unwrap();
        let second_line_segments: Vec<_> = second_line.split(": ").collect();
        let target_coords: Vec<usize> = second_line_segments[1].split(',')
            .map(|s| s.parse().expect("not a number")).collect();
        target = (target_coords[0], target_coords[1]);
    }
    return (depth, target)
}

const MODULO: usize = 20183;

fn generate_geologic_index_matrix_modulo(target: &Point, depth: usize) -> Matrix<usize> {
    let width = target.0 + 1;
    let height = target.1 + 1;
    let mut m = create_matrix(height, width, &0);

    for x in 1..width {
        m[0][x] = (x * 16807) % MODULO;
    }

    for y in 1..height {
        m[y][0] = (y * 48271) % MODULO;
    }

    for y in 1..height {
        for x in 1..width {
            if x == target.0 && y == target.1 {
                continue;
            }
            let el_up = geologic_index_to_erosion_level(m[y - 1][x], depth);
            let el_left = geologic_index_to_erosion_level(m[y][x - 1], depth);
            m[y][x] = (el_up * el_left) % MODULO;
        }
    }

    return m;
}

fn generate_map(target: &Point, depth: usize) -> Matrix<RegionType> {
    let gi = generate_geologic_index_matrix_modulo(target, depth);
    let (height, width) = get_matrix_dimensions(&gi);
    let mut map = create_matrix(height, width, &RegionType::Rocky);
    for y in 0..height {
        for x in 0..width {
            map[y][x] = match (geologic_index_to_erosion_level(gi[y][x], depth)) % 3 {
                0 => RegionType::Rocky,
                1 => RegionType::Wet,
                2 => RegionType::Narrow,
                _ => panic!("should not happen"),
            };
        }
    }
    return map;
}

fn geologic_index_to_erosion_level(index: usize, depth: usize) -> usize {
    (index + depth) % MODULO
}

fn calculate_risk(map: &Matrix<RegionType>) -> usize {
    let mut counter = 0;
    for row in map {
        for cell in row {
            counter += match cell {
                RegionType::Rocky => 0,
                RegionType::Wet => 1,
                RegionType::Narrow => 2,
            }
        }
    }
    return counter;
}

fn main() {
    let (depth, target) = read_input();
    println!("{} {:?}", depth, target);
    let map = generate_map(&target, depth);
    print_debug_matrix(&map);
    let risk = calculate_risk(&map);
    println!("{}", risk);
}

