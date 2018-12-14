use std::env;

fn calculate_cell_power_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let power_level1 = rack_id * y;
    let power_level2 = power_level1 + serial_number;
    let power_level3 = power_level2 * rack_id;
    let power_level4 = (power_level3 / 100) % 10;
    let power_level5 = power_level4 - 5;
    return power_level5;
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let serial_number: i32 = args[1].parse().expect("argument is not an integer");

    let mut map = Vec::new();
    map.resize(300, Vec::new());
    for row in &mut map {
        row.resize(300, 0);
    }

    for x in 1i32..=300 {
        for y in 1i32..=300 {
            let power_level = calculate_cell_power_level(x, y, serial_number);
            map[(y - 1) as usize][(x - 1) as usize] = power_level;
        }
    }

    let mut best_x = -1;
    let mut best_y = -1;
    let mut best_sum = -9 * 9;

    for x in 1i32..=300-2 {
        for y in 1i32..=300-2 {
            let mut sum = 0;
            for i in 0..3 {
                for j in 0..3 {
                    sum += map[(y + j - 1) as usize][(x + i - 1) as usize];
                }
            }

            if sum > best_sum {
                best_sum = sum;
                best_x = x;
                best_y = y;
            }
        }
    }
    println!("{},{}", best_x, best_y);
    println!("total_power: {}", best_sum);
}
