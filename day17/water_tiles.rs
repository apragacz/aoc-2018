use std::io;
use std::io::BufRead;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp;
use std::cmp::Ordering;
use std::fmt;

type Point = (usize, usize);
type Matrix<T> = Vec<Vec<T>>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
enum WaterState {
    NoWater,
    WaterDown,
    WaterSide,
    WaterUp,
}

impl fmt::Debug for WaterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            WaterState::NoWater => ".",
            WaterState::WaterDown => "D",
            WaterState::WaterSide => "S",
            WaterState::WaterUp => "U",
        };
        write!(f, "{}", string)
    }
}


fn map_to_points(coords_map: &HashMap<&str, [usize; 2]>) -> Vec<Point> {
    let xr = coords_map.get("x").unwrap();
    let yr = coords_map.get("y").unwrap();
    let mut points = Vec::new();
    for y in yr[0]..=yr[1] {
        for x in xr[0]..=xr[1] {
            points.push((x, y));
        }
    }
    return points;
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

fn print_debug_bool_matrix(mat: &Matrix<bool>) {
    for row in mat {
        for cell in row {
            let cell_string = match *cell {
                true => "#",
                false => ".",
            };
            print!("{} ", cell_string);
        }
        println!("");
    }
}

fn print_clay_and_water_map_with_range(clay_map: &Matrix<bool>, water_map: &Matrix<WaterState>, min_height: Option<usize>, max_height: Option<usize>) {
    let (height, width) = get_matrix_dimensions(clay_map);
    let height_start = min_height.map(|x| cmp::max(x, 0)).unwrap_or(0);
    let height_end = max_height.map(|x| cmp::min(x, height)).unwrap_or(height);
    for y in height_start..height_end {
        for x in 0..width {
            let string = match (&clay_map[y][x], &water_map[y][x]) {
                (true, _) => "#",
                (false, WaterState::WaterUp) => "^",
                (false, WaterState::WaterSide) => "~",
                (false, WaterState::WaterDown) => "v",
                (false, WaterState::NoWater) => ".",
            };
            print!("{} ", string);
        }
        println!("");
    }
}

fn print_clay_and_water_map(clay_map: &Matrix<bool>, water_map: &Matrix<WaterState>) {
    print_clay_and_water_map_with_range(clay_map, water_map, None, None)
}


fn build_depth_map(clay_map: &Matrix<bool>) -> Matrix<Option<usize>> {
    let (height, width) = get_matrix_dimensions(clay_map);
    let mut depth_map: Matrix<Option<usize>> = create_matrix(height, width, &None);
    let mut q = VecDeque::new();
    let max_y = height - 1;

    for i in 0..width {
        if !clay_map[max_y][i] {
            q.push_back(((i, max_y), 0));
        }
    }

    while !q.is_empty() {
        let ((x, y), d) = q.pop_front().unwrap();

        if clay_map[y][x] {
            continue;
        }

        let old_depth_opt: Option<usize> = depth_map[y][x];
        let new_depth_opt: Option<usize> = match old_depth_opt {
            None => Some(d),
            Some(old_d) => if old_d > d { Some (d) } else { old_depth_opt },
        };

        if old_depth_opt == new_depth_opt {
            continue;
        }

        depth_map[y][x] = new_depth_opt;

        if y > 0 {
            q.push_back(((x, y - 1), d));
        }
        if x > 0 {
            q.push_back(((x - 1, y), d));
        }
        if x < width - 1 {
            q.push_back(((x + 1, y), d));
        }
        if y < height - 1 {
            q.push_back(((x, y + 1), d + 1));
        }
    }

    return depth_map;
}

fn is_blocked_by_clay(clay_map: &Matrix<bool>, p: &Point) -> bool {
    let (x, y) = p;
    return clay_map[*y][*x];
}

fn is_blocked_by_clay_and_water(clay_map: &Matrix<bool>, water_map: &Matrix<WaterState>, p: &Point) -> bool {
    if is_blocked_by_clay(clay_map, p) {
        return true;
    }
    let (x, y) = p;
    return match water_map[*y][*x] {
        WaterState::NoWater => false,
        WaterState::WaterDown => false,
        WaterState::WaterSide => true,
        WaterState::WaterUp => true,
    };
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct WaterQueueItem {
    insert_order: usize,
    point: Point,
    state: WaterState,
}
impl Ord for WaterQueueItem {
    fn cmp(&self, other: &WaterQueueItem) -> Ordering {
        let result = self.point.1.cmp(&other.point.1)
            .then_with(|| other.state.cmp(&self.state))
            .then_with(|| other.insert_order.cmp(&self.insert_order));
        //println!(" cmp {:?} {:?} -> {:?}", self, other, result);
        return result;
    }
}
impl PartialOrd for WaterQueueItem {
    fn partial_cmp(&self, other: &WaterQueueItem) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct WaterQueue {
    heap: BinaryHeap<WaterQueueItem>,
    insert_counter: usize,
}
impl WaterQueue {
    fn new() -> WaterQueue {
        WaterQueue {
            heap: BinaryHeap::new(),
            insert_counter: 0,
        }
    }
    fn push(&mut self, item: (Point, WaterState)) {
        let (point, state) = item;
        self.heap.push(WaterQueueItem {
            point,
            state,
            insert_order: self.insert_counter,
        });
        self.insert_counter += 1;
    }
    fn pop(&mut self) -> Option<(Point, WaterState)> {
        self.heap.pop().map(|item| (item.point, item.state))
    }
    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

fn build_water_map(clay_map: &Matrix<bool>, depth_map: &Matrix<Option<usize>>, start_point: &Point) -> Matrix<WaterState> {
    let (height, width) = get_matrix_dimensions(clay_map);
    let mut water_map: Matrix<WaterState> = create_matrix(height, width, &WaterState::NoWater);
    let mut q: WaterQueue = WaterQueue::new();

    q.push((start_point.clone(), WaterState::WaterDown));

    while !q.is_empty() {
        let (p, water_state) = q.pop().unwrap();
        let (x, y) = p;

        //println!("{:?} {:?}", p, water_state);

        if clay_map[y][x] {
            continue;
        }

        if !(water_state > water_map[y][x]) {
            continue;
        }

        //println!("  OK");

        water_map[y][x] = water_state.clone();

        match water_state {
            WaterState::NoWater => {
                panic!("This should not happen");
            },
            WaterState::WaterDown => {
                if y < height - 1 {
                    let down_p: Point = (x, y + 1);
                    if is_blocked_by_clay(clay_map, &down_p) {
                        q.push((p, WaterState::WaterSide));
                    } else {
                        q.push((down_p, WaterState::WaterDown));
                    }
                }
            },
            WaterState::WaterSide => {
                let mut down_blocked = false;

                if y < height - 1 {
                    let down_p: Point = (x, y + 1);
                    down_blocked = is_blocked_by_clay_and_water(clay_map, &water_map, &down_p);
                    q.push((down_p, WaterState::WaterDown));
                }

                if down_blocked {
                    if x < width - 1 {
                        let right_p: Point = (x + 1, y);
                        q.push((right_p, water_state.clone()));
                    }
                    if x > 0 {
                        let left_p: Point = (x - 1, y);
                        q.push((left_p, water_state.clone()));
                    }
                }

                q.push((p, WaterState::WaterUp));
            }
            WaterState::WaterUp => {
                let mut down_blocked = false;
                let mut left_blocked = false;
                let mut right_blocked = false;

                if y < height - 1 {
                    let down_p: Point = (x, y + 1);
                    down_blocked = is_blocked_by_clay_and_water(clay_map, &water_map, &down_p);
                }

                if down_blocked {
                    if x < width - 1 {
                        let right_p: Point = (x + 1, y);
                        right_blocked = is_blocked_by_clay_and_water(clay_map, &water_map, &right_p);
                    }
                    if x > 0 {
                        let left_p: Point = (x - 1, y);
                        left_blocked = is_blocked_by_clay_and_water(clay_map, &water_map, &left_p);
                    }
                }

                if down_blocked && left_blocked && right_blocked && y > 0 {
                    let up_p: Point = (x, y - 1);
                    if !is_blocked_by_clay(&clay_map, &up_p) && depth_map[y - 1][x].unwrap() + 1 == depth_map[y][x].unwrap() {
                        q.push((up_p, WaterState::WaterSide));
                    }
                }
            }

        }
        //print_clay_and_water_map_with_range(clay_map, &water_map, Some(30), Some(60));
        //println!("");
    }

    return water_map;
}

fn main() {
    let mut points: Vec<Point> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let segments: Vec<&str> = l.split(", ").collect();
        let mut coords_map: HashMap<&str, [usize; 2]> = HashMap::new();
        for segment in &segments {
            let coord_strings: Vec<&str> = segment.split('=').collect();
            let coord_name = coord_strings[0];
            let coord_values: Vec<usize> = coord_strings[1].split("..").map(|s| s.parse().expect("not an integer")).collect();
            if coord_values.len() == 2 {
                coords_map.insert(coord_name, [coord_values[0], coord_values[1]]);
            } else if coord_values.len() == 1 {
                coords_map.insert(coord_name, [coord_values[0], coord_values[0]]);
            } else {
                panic!("invalid data");
            }
        }
        points.extend(map_to_points(&coords_map).iter());
    }

    let clay_max_y = points.iter().fold(0, |my, (_, ely)| cmp::max(my, *ely));
    let clay_min_y = points.iter().fold(clay_max_y, |my, (_, ely)| cmp::min(my, *ely));
    let clay_max_x = points.iter().fold(0, |mx, (elx, _)| cmp::max(mx, *elx));
    let clay_min_x = points.iter().fold(clay_max_x, |mx, (elx, _)| cmp::min(mx, *elx));

    assert!(clay_min_x > 0);
    let max_y = clay_max_y;
    // add +1 tile on left and right in case water could flow on the edge. also ensure that water source is within.
    let min_x = cmp::min(clay_min_x - 1, 500);
    let max_x = cmp::max(clay_max_x + 1, 500);

    let height = max_y + 1;
    let width = max_x + 1 - min_x;
    let offset = min_x;

    let mut clay_map = create_matrix(height, width, &false);
    for (x, y) in &points {
        clay_map[*y][*x - offset] = true;
    }

    let depth_map = build_depth_map(&clay_map);

    let start_map_point = (500 - offset, 0);

    let water_map = build_water_map(&clay_map, &depth_map, &start_map_point);

    //print_debug_matrix(&water_map);

    let mut water_counter = 0;

    for (y, row) in water_map.iter().enumerate() {
        if y < clay_min_y {
            continue;
        }
        for cell in row.iter() {
            water_counter += match cell {
                WaterState::NoWater => 0,
                WaterState::WaterUp => 1,
                WaterState::WaterDown => 1,
                WaterState::WaterSide => 1,
            };
        }
    }

    print_clay_and_water_map(&clay_map, &water_map);

    println!("{}", water_counter);
}
