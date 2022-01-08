use std::time::Instant;
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

    let time1 = Instant::now();
    let solution1 = part1(input);
    let elapsed1 = time1.elapsed();
    println!("Day 1, Part 1: {:?} ({:?})", solution1, elapsed1);
    let time2 = Instant::now();
    let solution2 = part2(input);
    let elapsed2 = time2.elapsed();
    println!("Day 1, Part 2: {:?} ({:?})", solution2, elapsed2);
}
