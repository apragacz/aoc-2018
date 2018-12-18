use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    let num_of_iterations: usize = args[1].parse().expect("argument is not an integer");

    let mut recipe_scores: Vec<u8> = Vec::new();
    recipe_scores.push(3);
    recipe_scores.push(7);
    let mut ai: usize = 0;
    let mut bi: usize = 1;

    while recipe_scores.len() < num_of_iterations + 2 {
        println!("{} {}   {:?}", ai, bi, recipe_scores);
        let a_score: u8 = recipe_scores[ai];
        let b_score: u8 = recipe_scores[bi];
        let sum = a_score + b_score;
        if sum >= 10 {
            recipe_scores.push(1);
            recipe_scores.push(sum % 10);
        } else {
            recipe_scores.push(sum);
        }
        ai = (ai + 1 + (a_score as usize)) % recipe_scores.len();
        bi = (bi + 1 + (b_score as usize)) % recipe_scores.len();
    }

    let start = if recipe_scores.len() >= 10 { recipe_scores.len() - 10 } else { 0 };
    let mut result = String::new();

    for i in start..recipe_scores.len() {
        result.push(('0' as u8 + recipe_scores[i]) as char);
    }
    println!("{}", result);
}
