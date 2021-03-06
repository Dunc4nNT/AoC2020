use std::time::Instant;
use regex::Regex;

struct LineStruct {
    password: String,
    character: char,
    min: i32,
    max: i32
}

fn process_line(input: &str) -> LineStruct {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let captures = re.captures(input).unwrap();

    return LineStruct{
        password: captures[4].to_string(),
        character: captures[3].parse::<char>().unwrap(),
        min: captures[1].parse::<i32>().unwrap(),
        max: captures[2].parse::<i32>().unwrap()
    }
}

fn part1(input: &str) -> i32 {
    let mut valid_passwords: i32 = 0;

    for line in input.lines() {
        let data: LineStruct = process_line(line);
        let count: i32 = data.password.matches(data.character).count().try_into().unwrap();
        if count >= data.min && count <= data.max {
            valid_passwords += 1;
        }
    }

    return valid_passwords;
}


fn part2(input: &str) -> i32 {
    let mut valid_passwords: i32 = 0;

    for line in input.lines() {
        let data: LineStruct = process_line(line);
        let cmin: char = data.password.chars().nth((data.min-1).try_into().unwrap()).unwrap();
        let cmax: char = data.password.chars().nth((data.max-1).try_into().unwrap()).unwrap();

        if (cmin == data.character) ^ (cmax == data.character) {
            valid_passwords += 1;
        }
    }

    return valid_passwords;
}

pub fn solve() {
    let input: &str = include_str!("input.txt");

    let time1 = Instant::now();
    let solution1 = part1(input);
    let elapsed1 = time1.elapsed();
    println!("Day 2, Part 1: {:?} ({:?})", solution1, elapsed1);
    let time2 = Instant::now();
    let solution2 = part2(input);
    let elapsed2 = time2.elapsed();
    println!("Day 2, Part 2: {:?} ({:?})", solution2, elapsed2);
}
