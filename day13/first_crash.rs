use std::io;
use std::io::BufRead;
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Clone, Debug)]
enum Edge {
    Left,
    Top,
    Right,
    Bottom,
}

fn flip_edge(edge: Edge) -> Edge {
    match edge {
        Edge::Left => Edge::Right,
        Edge::Top => Edge::Bottom,
        Edge::Right => Edge::Left,
        Edge::Bottom => Edge::Top,
    }
}

fn rotate_edge_cw(edge: Edge) -> Edge {
    match edge {
        Edge::Left => Edge::Top,
        Edge::Top => Edge::Right,
        Edge::Right => Edge::Bottom,
        Edge::Bottom => Edge::Left,
    }
}

fn rotate_edge_ccw(edge: Edge) -> Edge {
    match edge {
        Edge::Left => Edge::Bottom,
        Edge::Top => Edge::Left,
        Edge::Right => Edge::Top,
        Edge::Bottom => Edge::Right,
    }
}

#[derive(PartialEq, Eq, Clone)]
enum CartState {
    GoLeft,
    GoStraight,
    GoRight,
}

fn next_cart_state(state: CartState) -> CartState {
    match state {
        CartState::GoLeft => CartState::GoStraight,
        CartState::GoStraight => CartState::GoRight,
        CartState::GoRight => CartState::GoLeft,
    }
}

#[derive(PartialEq, Eq, Clone)]
enum MapCell {
    Nothing,
    Crossing,
    Segment(Edge, Edge),
}

type Map = Vec<Vec<MapCell>>;

struct Cart {
    x: usize,
    y: usize,
    direction: Edge,
    crossing_state: CartState,
}
impl Cart {
    fn new(x: usize, y: usize, direction: Edge) -> Cart {
        Cart { x, y, direction, crossing_state: CartState::GoLeft }
    }
    fn step(&mut self, map: &Map) {
        let (delta_x, delta_y): (i32, i32) = match self.direction {
            Edge::Left => (-1, 0),
            Edge::Top => (0, -1),
            Edge::Right => (1, 0),
            Edge::Bottom => (0, 1),
        };
        let next_x = (self.x as i32 + delta_x) as usize;
        let next_y = (self.y as i32 + delta_y) as usize;
        let next_cell: &MapCell = &map[next_y][next_x];
        match next_cell {
            MapCell::Nothing => panic!("Cart should not land on field {},{}!", next_x, next_y),
            MapCell::Crossing => {
                let next_direction: Edge = match self.crossing_state {
                    CartState::GoLeft => rotate_edge_ccw(self.direction.clone()),
                    CartState::GoStraight => self.direction.clone(),
                    CartState::GoRight => rotate_edge_cw(self.direction.clone()),
                };
                let next_crossing_state = next_cart_state(self.crossing_state.clone());
                self.direction = next_direction;
                self.crossing_state = next_crossing_state;
            },
            MapCell::Segment(_, _) => {
                let next_direction = get_another_edge(next_cell, flip_edge(self.direction.clone())).unwrap();
                self.direction = next_direction;
            }
        }
        self.x = next_x;
        self.y = next_y;
    }
    fn get_order_key(&self) -> (usize, usize) {
        return (self.y, self.x);
    }
}

fn has_edge(cell: &MapCell, edge: Edge) -> bool {
    match cell {
        MapCell::Nothing => false,
        MapCell::Crossing => true,
        MapCell::Segment(e1, e2) => edge == *e1 || edge == *e2,
    }
}

fn get_another_edge(cell: &MapCell, edge: Edge) -> Option<Edge> {
    match cell {
        MapCell::Nothing => None,
        MapCell::Crossing => None,
        MapCell::Segment(e1, e2) => {
            if edge == *e1 {
                Some(e2.clone())
            } else if edge == *e2 {
                Some(e1.clone())
            } else {
                None
            }
        }
    }
}

fn has_bottom_edge(cell: &MapCell) -> bool {
    return has_edge(cell, Edge::Bottom);
}

fn main() {
    let mut map: Map = Vec::new();
    let mut carts: Vec<Cart> = Vec::new();
    let stdin = io::stdin();
    let vertical = MapCell::Segment(Edge::Top, Edge::Bottom);
    let horizontal = MapCell::Segment(Edge::Left, Edge::Right);
    for (y, line) in stdin.lock().lines().enumerate() {
        let l = line.unwrap();
        let mut map_row = Vec::new();
        for (x, c) in l.chars().enumerate() {
            let upper_cell: &MapCell = if y > 0 {
                &map[y - 1][x]
            } else {
                &MapCell::Nothing
            };
            let cell: MapCell = match c {
                ' ' => MapCell::Nothing,
                '|' => vertical.clone(),
                '^' => vertical.clone(),
                'v' => vertical.clone(),
                '-' => horizontal.clone(),
                '>' => horizontal.clone(),
                '<' => horizontal.clone(),
                '/' => {
                    if has_bottom_edge(upper_cell) {
                        MapCell::Segment(Edge::Left, Edge::Top)
                    } else {
                        MapCell::Segment(Edge::Right, Edge::Bottom)
                    }
                },
                '\\' => {
                    if has_bottom_edge(upper_cell) {
                        MapCell::Segment(Edge::Right, Edge::Top)
                    } else {
                        MapCell::Segment(Edge::Left, Edge::Bottom)
                    }
                },
                '+' => MapCell::Crossing,
                _ => panic!("Unknown char {}", c),
            };
            map_row.push(cell);

            let cart_direction = match c {
                '^' => Some(Edge::Top),
                'v' => Some(Edge::Bottom),
                '>' => Some(Edge::Right),
                '<' => Some(Edge::Left),
                _ => None,
            };
            match cart_direction {
                None => {}
                Some(direction) => {
                    carts.push(Cart::new(x, y, direction));
                }
            }
        }
        map.push(map_row);
    }

    let mut crash_detected = false;

    let mut cart_index_order_map: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    for (i, cart) in carts.iter().enumerate() {
        cart_index_order_map.insert(cart.get_order_key(), i);
    }
    while !crash_detected {
        // println!("Tick:");
        let cart_index_order: Vec<usize> = cart_index_order_map.values().cloned().collect();
        for i in cart_index_order {
            let cart = carts.get_mut(i).unwrap();
            // println!("  {}: {},{} {:?}", i, cart.x, cart.y, cart.direction);
            let old_order_key: (usize, usize) = cart.get_order_key();
            cart.step(&map);
            let new_order_key: (usize, usize) = cart.get_order_key();
            cart_index_order_map.remove(&old_order_key);
            if cart_index_order_map.contains_key(&new_order_key){
                println!("{},{}", cart.x, cart.y);
                crash_detected = true;
                break;
            }
            cart_index_order_map.insert(new_order_key, i);
        }
    }
}
