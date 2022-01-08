use std::time::Instant;

fn process_data(input: &str) -> Vec<i32> {
    input.lines()
        .filter_map(|i| i.parse().ok())
        .collect()
}

fn part1(input: &str) -> i32 {
    let mut data = process_data(input);
    data.sort();
    let mut one_jolt: i32 = 0;
    let mut three_jolt: i32 = 0;

    for i in 0..data.len() {
        let mut diff: i32 = 0;
        if i == 0 {
            diff += data[i];
        } else {
            diff += data[i] - data[i-1];
        }
        if diff == 1 {
            one_jolt += 1;
        } else if diff == 3 {
            three_jolt += 1;
        }
    }

    return one_jolt * (three_jolt + 1);
}


fn part2(input: &str) -> i32 {
    let mut data = process_data(input);


    return 1;
}

fn main() {
    let input: &str = include_str!("input.txt");

    let time1 = Instant::now();
    let solution1 = part1(input);
    let elapsed1 = time1.elapsed();
    println!("Day 10 Part 1: {:?} ({:?})", solution1, elapsed1);
    let time2 = Instant::now();
    let solution2 = part2(input);
    let elapsed2 = time2.elapsed();
    println!("Day 10, Part 2: {:?} ({:?})", solution2, elapsed2);
}
