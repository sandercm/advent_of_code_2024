use std::time::{SystemTime, UNIX_EPOCH};

fn solve_hard(input: &str) {}

fn solve_easy(input: &str) {}

fn main() {
    let input = include_str!("input/input.txt");

    // start easy
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    solve_easy(&input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("easy took {:?}", end-start);
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    solve_hard(&input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("hard took {:?}", end-start); 
}
