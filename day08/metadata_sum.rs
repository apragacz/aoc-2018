use std::io;
use std::io::Read;

struct VecPos {
    vec: Vec<usize>,
    pos: usize,
}
impl VecPos {
    fn next(&mut self) -> Option<usize> {
        let opt = self.vec.get(self.pos).cloned();
        self.pos += 1;
        return opt;
    }
}

fn sum_tree_numbers(vec_pos: &mut VecPos) -> usize {
    let n: usize = vec_pos.next().unwrap();
    let m: usize = vec_pos.next().unwrap();
    let mut sum = 0;
    for _ in 0..n {
        sum += sum_tree_numbers(vec_pos);
    }
    for _ in 0..m {
        sum += vec_pos.next().unwrap();
    }
    return sum;
}

fn main() {
    let mut payload = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut payload).expect("reading file failed");
    payload = payload.trim().to_string();
    let numbers: Vec<usize> = payload.split(' ').map(|el| el.parse().expect("not a number")).collect();

    let mut vec_pos = VecPos {vec: numbers, pos: 0};
    let sum = sum_tree_numbers(&mut vec_pos);
    println!("{}", sum);
}
