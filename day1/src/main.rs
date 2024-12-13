fn find_occurance(key: i64, list: &Vec<i64>) -> i64 {
    list.iter().filter(|num|{**num == key}).count() as i64
}

fn handle_easy_input(input: &str) {
    let mut difference_sum = 0;

    let mut left: Vec<i64> = vec![];
    let mut right: Vec<i64> = vec![];

    for line in input.lines() {
        let split_lines = line.split(" ").filter(
            |item| {*item != ""}
        ).map(|str| {
            str.parse::<i64>().unwrap()
        });
        let split_lines: Vec<i64> = split_lines.collect();
        left.push(split_lines[0]);
        right.push(split_lines[1]);
    }

    left.sort();
    right.sort();

    for (left, right) in left.iter().zip(right.iter()) {
        difference_sum += (left - right).abs();
    }
    // end of easy
    println!("{}", difference_sum);

    // start of hard
    let mut simularity = 0;
    for key in left {
        simularity += key * find_occurance(key, &right);
    }

    println!("{}", simularity);
}

fn main() {
    let input = include_str!("input/easy_input.txt");
    handle_easy_input(&input);

}
