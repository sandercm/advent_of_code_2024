// None if valid, otherwise return first invalid index
fn find_first_invalid_index(line: &Vec<i64>) -> Option<usize> {
    let increasing = line[0] < line[1];
    let mut line = line.windows(2).enumerate();
    
    while let Some((index, window)) = line.next() {
        if (window[0] < window[1]) != increasing {
            return Some(index);
        }

        let diff = (window[0] - window[1]).abs();
        if diff > 3 || diff == 0 {
            return Some(index);
        }
    }
    None
}

fn check_valid_slice(line: &Vec<i64>) -> bool {
    let has_invalid = find_first_invalid_index(&line);

    if let None = has_invalid {
        return true;
    }

    let mut new_line = line.clone();
    new_line.remove(has_invalid.unwrap());
    if let None = find_first_invalid_index(&new_line) {
        return true;
    }

    let mut new_line = line.clone();
    new_line.remove(has_invalid.unwrap() + 1);
    if let None = find_first_invalid_index(&new_line) {
        return true;
    }

    if has_invalid.unwrap() == 0 {
        return false;
    }

    let mut new_line = line.clone();
    new_line.remove(has_invalid.unwrap() - 1);
    if let None = find_first_invalid_index(&new_line) {
        return true;
    }


    false
}

fn check_line_hard(line:&str) -> bool {
    let mut line: Vec<i64> = line
        .split_ascii_whitespace()
        .map(|num| num.parse::<i64>().unwrap()).collect();

    check_valid_slice(&mut line)
}

fn check_line_easy(line: &str) -> bool {
    let line: Vec<i64> = line
        .split_ascii_whitespace()
        .map(|num| num.parse::<i64>().unwrap()).collect();

    // check all increasing or decreasing
    let mut increasing: Option<bool> = None;

    let mut window = line.windows(2);
    let mut is_valid = true;

    while let Some(slice) = window.next() {
        let increasing_window = slice[0] < slice[1];

        if increasing.is_none() {
            increasing = Some(increasing_window);
        }

        if increasing_window != increasing.unwrap() {
            is_valid = false;
        }
        let diff = (slice[0] - slice[1]).abs();
        if diff > 3 || diff == 0 {
            is_valid = false;
        }
    }
    is_valid
}

fn main() {
    let input = include_str!("input/input.txt");

    // start easy

    let safe = input.lines().into_iter().map(check_line_easy).filter(|bool| {*bool}).count();
    println!("so many safe lines easy {}", safe);
    let safe = input.lines().into_iter().map(check_line_hard).filter(|bool| {*bool}).count();
    println!("so many safe lines hard {}", safe);
}
