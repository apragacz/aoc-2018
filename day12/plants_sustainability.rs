use std::io;
use std::io::BufRead;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

type Pattern = [bool; 5];

fn pattern_vec_to_array(pat_vec: &Vec<bool>) -> Pattern {
    return match *pat_vec.as_slice() {
        [h1, h2, h3, h4, h5] => [h1, h2, h3, h4, h5],
        _ => panic!("invalid pattern"),
    }
}

struct State {
    set: HashSet<i32>,
}
impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = self.lower_bound();
        let b = self.upper_bound();
        let mut string = String::new();
        for i in a..=b {
            string.push(if self.get(i) { '#' } else { '.' });
        }
        write!(f, "{}", string)
    }
}
impl State {
    fn from_vec(vec: &Vec<bool>) -> State {
        let mut new_state = State { set: HashSet::new() };
        for (i, el) in vec.iter().enumerate() {
            new_state.put(i as i32, *el);
        }
        return new_state;
    }
    fn get(&self, index: i32) -> bool {
        return self.set.contains(&index);
    }
    fn put(&mut self, index: i32, value: bool) {
        match value {
            true => self.set.insert(index),
            false => self.set.remove(&index),
        };
        return;
    }
    fn get_pattern(&self, index: i32) -> Pattern {
        let mut pat_vec: Vec<bool> = Vec::new();
        for i in (index - 2)..=(index + 2) {
            pat_vec.push(self.get(i));
        }
        return pattern_vec_to_array(&pat_vec);
    }
    fn lower_bound(&self) -> i32 {
        return self.set.iter().fold(0, |m, el| cmp::min(m, *el));
    }
    fn upper_bound(&self) -> i32 {
        return self.set.iter().fold(0, |m, el| cmp::max(m, *el));
    }
    fn apply_rules(&self, rules: &Vec<Rule>) -> State {
        let mut rule_map: HashMap<Pattern, bool> = HashMap::new();
        for rule in rules.iter() {
            rule_map.insert(rule.head, rule.tail);
        }
        let a = self.lower_bound() - 2;
        let b = self.upper_bound() + 2;
        let mut new_state = State { set: HashSet::new() };

        for i in a..=b {
            let pat = self.get_pattern(i);
            match rule_map.get(&pat) {
                Some(v) => {
                    new_state.put(i, *v);
                },
                None => {
                    new_state.put(i, false);
                },
            }
        }
        return new_state;

    }
    fn set_index_sum(&self) -> i32 {
        return self.set.iter().fold(0, |s, el| s + *el);
    }
}

#[derive(Debug)]
struct Rule {
    head: Pattern,
    tail: bool,
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|r| r.expect("could not read")).collect();
    let mut rules: Vec<Rule> = Vec::new();
    let initial_state_vec: Vec<bool> = lines[0].trim().chars().skip(15).map(|c| c == '#').collect();
    let initial_state = State::from_vec(&initial_state_vec);

    for line in lines.iter().skip(2) {
        let segments: Vec<&str> = line.trim().split("=>").map(|s| s.trim()).collect();
        let tail: bool = segments[1].chars().next().unwrap() == '#';
        let head_vec: Vec<bool> = segments[0].chars().map(|c| c == '#').collect();
        let head = pattern_vec_to_array(&head_vec);
        rules.push(Rule { head, tail });
    }

    let mut state = initial_state;
    for _ in 0..20 {
        state = state.apply_rules(&rules);
    }
    println!("{}", state.set_index_sum());
}
