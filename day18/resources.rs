use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::fmt;

type Point = (usize, usize);

#[derive(PartialEq, Eq, Hash, Clone)]
enum Field {
    OpenGround,
    Trees,
    Lumberyard,
}
impl Field {
    fn from_char(c: char) -> Field {
        match c {
            '.' => Field::OpenGround,
            '|' => Field::Trees,
            '#' => Field::Lumberyard,
            _ => panic!("field not recognized: {}", c),
        }
    }
    fn to_char(&self) -> char {
        match self {
            Field::OpenGround => '.',
            Field::Trees => '|',
            Field::Lumberyard => '#',
        }
    }
}
impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

type Matrix<T> = Vec<Vec<T>>;
type State = Matrix<Field>;

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

fn load_initial_state() -> State {
    let stdin = io::stdin();
    let mut state = Vec::new();
    for line in stdin.lock().lines() {
        let mut row = Vec::new();
        for c in line.unwrap().chars() {
            row.push(Field::from_char(c));
        }
        state.push(row);
    }
    return state;
}

fn count_neighborhood_fields(state: &State, point: &Point) -> HashMap<Field, usize> {
    let (width, height) = get_matrix_dimensions(state);
    let (x, y) = point;
    let mut counters = HashMap::new();

    counters.insert(Field::OpenGround, 0);
    counters.insert(Field::Trees, 0);
    counters.insert(Field::Lumberyard, 0);

    let min_x = if *x > 0 { *x - 1 } else { 0 };
    let max_x = if *x < width - 1 { *x + 1 } else { width - 1 };
    let min_y = if *y > 0 { *y - 1 } else { 0 };
    let max_y = if *y < height - 1 { *y + 1 } else { height - 1 };

    for cy in min_y..=max_y {
        for cx in min_x..=max_x {
            if cx == *x && cy == *y {
                continue;
            }
            let counter: &mut usize = counters.get_mut(&state[cy][cx]).unwrap();
            *counter += 1;
        }
    }
    return counters;
}

fn get_new_state(state: &State) -> State {
    let (width, height) = get_matrix_dimensions(state);
    let mut new_state = create_matrix(width, height, &Field::OpenGround);
    for y in 0..height {
        for x in 0..width {
            let counters = count_neighborhood_fields(&state, &(x, y));
            let new_field = match state[y][x] {
                Field::OpenGround => {
                    if counters[&Field::Trees] >= 3 {
                        Field::Trees
                    } else {
                        Field::OpenGround
                    }
                },
                Field::Trees => {
                    if counters[&Field::Lumberyard] >= 3 {
                        Field::Lumberyard
                    } else {
                        Field::Trees
                    }
                },
                Field::Lumberyard => {
                    if counters[&Field::Lumberyard] >= 1 && counters[&Field::Trees] >= 1 {
                        Field::Lumberyard
                    } else {
                        Field::OpenGround
                    }
                },
            };
            new_state[y][x] = new_field;
        }
    }

    return new_state;
}

fn calculate_resource_index(state: &State) -> usize {
    let mut num_of_trees = 0;
    let mut num_of_lumberyards = 0;
    for row in state {
        for field in row {
            match *field {
                Field::OpenGround => {},
                Field::Trees => {
                    num_of_trees += 1;
                },
                Field::Lumberyard => {
                    num_of_lumberyards += 1;
                },
            }
        }
    }

    return num_of_trees * num_of_lumberyards;
}

fn main() {
    let initial_state = load_initial_state();

    let mut state = initial_state;

    for _ in 0..10 {
        state = get_new_state(&state);
    }
    print_debug_matrix(&state);
    println!("{}", calculate_resource_index(&state));
}
