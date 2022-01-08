use std::time::Instant;
use itertools::Itertools;

fn process_data(input: &str) -> Vec<i64> {
    input.lines()
        .filter_map(|i| i.parse().ok())
        .collect()
}

fn check_sum(numbers: Vec<i64>, num: i64) -> bool {
    for pair in numbers.iter().combinations(2) {
        if pair[0] + pair[1] == num {
            return true;
        }
    }
    return false;
}

fn get_bad_num(data: Vec<i64>, size: usize) -> i64 {
    let mut bad_num: i64 = 0;
    let mut last_25: Vec<i64> = Vec::new();

    for i in 0..size {
        last_25.push(data[i]);
    }

    for i in size..data.len() {
        if check_sum(last_25.clone(), data[i]) {
            last_25.remove(0);
            last_25.push(data[i]);
        } else {
            bad_num = data[i];
            break;
        }
    }

    return bad_num;
}

fn part1(input: &str, size: usize) -> i64 {
    let data: Vec<i64> = process_data(input);
    let bad_num = get_bad_num(data, size);

    return bad_num;
}


fn part2(input: &str, size: usize) -> i64 {
    let data: Vec<i64> = process_data(input);
    let bad_num = get_bad_num(data.clone(), size);
    let mut result: i64 = 0;

    for i in 3..size+1 {
        for window in data.windows(i) {
            let sum: i64 = window.iter().sum();
            if sum == bad_num {
                let min = window.iter().min().unwrap();
                let max = window.iter().max().unwrap();
                result = min + max;
                return result;
            }
        }
    }

    return result;
}

fn main() {
    let input: &str = include_str!("input.txt");
    let size: usize = 25;

    let time1 = Instant::now();
    let solution1 = part1(input, size);
    let elapsed1 = time1.elapsed();
    println!("Day 9, Part 1: {:?} ({:?})", solution1, elapsed1);
    let time2 = Instant::now();
    let solution2 = part2(input, size);
    let elapsed2 = time2.elapsed();
    println!("Day 9, Part 2: {:?} ({:?})", solution2, elapsed2);
}
