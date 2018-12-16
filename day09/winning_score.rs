use std::io;
use std::io::BufRead;
use std::collections::BTreeSet;
use std::cmp;

#[derive(Debug)]
struct MarbleCircle{
    numbers: Vec<usize>,
    selected_index: usize,
}
impl MarbleCircle {
    fn new() -> MarbleCircle {
        let mut new_circle = MarbleCircle { numbers: Vec::new(), selected_index: 0};
        new_circle.numbers.push(0);
        return new_circle;
    }
    fn numbers_index(&self, index: i32) -> usize {
        let n = self.numbers.len() as i32;
        let remainder = index % n;
        if remainder < 0 {
            return (remainder + n) as usize;
        } else {
            return remainder as usize;
        }
    }
    fn insert(&mut self, index: i32, value: usize, switch_selected: bool) {
        let mut i: usize = self.numbers_index(index);
        if i == 0 {
            i = self.numbers.len();
        }
        self.numbers.insert(i, value);
        if switch_selected {
                self.selected_index = i;
        } else if i <= self.selected_index {
            self.selected_index += 1;
        }
    }
    fn remove(&mut self, index: i32) {
        let i: usize = self.numbers_index(index);
        if i < self.selected_index {
            self.selected_index -= 1;
        }
        self.numbers.remove(i);
    }
    fn get(&self, index: i32) -> usize {
        let i: usize = self.numbers_index(index);
        return self.numbers[i];
    }
    fn set_selected_index(&mut self, index: i32) {
        self.selected_index = self.numbers_index(index);
    }
    fn get_selected_index(&self) -> i32 {
        return self.selected_index as i32;
    }
}

fn main() {
    let stdin = io::stdin();
    let line: String = stdin.lock().lines().next().unwrap().expect("missing line");
    let segments: Vec<&str> = line.trim().split(' ').collect();
    let num_of_players: usize = segments[0].parse().expect("not an integer");
    let max_marble_number: usize = segments[6].parse().expect("not an integer");
    assert!(num_of_players > 0);
    let mut player_scores = Vec::new();
    player_scores.resize(num_of_players, 0);
    let mut current_player_index = 0;
    let mut marble_numbers: BTreeSet<usize> = (1..=max_marble_number).collect();
    let mut circle = MarbleCircle::new();

    while !marble_numbers.is_empty() {
        let marble_num: usize = *marble_numbers.iter().next().unwrap();
        marble_numbers.remove(&marble_num);
        if marble_num % 23 == 0 {
            let old_selected_index = circle.get_selected_index();
            let inc = marble_num + circle.get(old_selected_index - 7);
            player_scores[current_player_index] += inc;
            circle.set_selected_index(old_selected_index - 6);
            circle.remove(old_selected_index - 7)
        } else {
            let i = circle.get_selected_index();
            circle.insert(i + 2, marble_num, true);
        }
        current_player_index = (current_player_index + 1) % num_of_players;
    }
    let best_score = player_scores.iter().fold(0, |m, el| cmp::max(m, *el));
    println!("{}", best_score);
}
