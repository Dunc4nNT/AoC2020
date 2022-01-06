use itertools::Itertools;

fn get_numbers(input: &str) -> Vec<i32> {
    input.lines()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn part1(input: &str) -> i32 {
    let numbers: Vec<i32> = get_numbers(input);

    for pair in numbers.iter().combinations(2) {
        if pair[0] + pair[1] == 2020 {
            return pair[0] * pair[1];
        }
    }
    return 0;
}


fn part2(input: &str) -> i32 {
    let numbers: Vec<i32> = get_numbers(input);

    for pair in numbers.iter().combinations(3) {
        if pair[0] + pair[1] + pair[2] == 2020 {
            return pair[0] * pair[1] * pair[2];
        }
    }
    return 0;
}

fn main() {
    let input: &str = include_str!("input.txt");

    println!("Day 1, Part 1: {:?}", part1(input));
    println!("Day 1, Part 2: {:?}", part2(input));
}
