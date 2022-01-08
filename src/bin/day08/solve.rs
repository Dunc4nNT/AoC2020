use std::time::Instant;
use std::collections::HashSet;
use regex::Regex;

fn process_data(input: &str) -> Vec<(String, i32)> {
    let re = Regex::new(r"(\w+) (\+\d+|-\d+)").unwrap();
    let mut instructions: Vec<(String, i32)> = Vec::new();

    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        instructions.push((captures[1].to_string(), captures[2].parse().unwrap()));
    }

    return instructions;
}

fn check_instructions(instructions: Vec<(String, i32)>) -> (i32, bool) {
    let mut cur: i32 = 0;
    let mut acc: i32 = 0;

    let mut seen: HashSet<i32> = HashSet::new();
    while !seen.contains(&cur) {
        if cur >= instructions.len().try_into().unwrap() {
            return (acc, true);
        }
        seen.insert(cur);
        let (instruction, amount) = &instructions[cur as usize];
        if instruction == "nop" {
            cur += 1;
        } else if instruction == "acc" {
            cur += 1;
            acc += amount;
        } else if instruction == "jmp" {
            cur += amount;
        }
    }

    return (acc, false);
}

fn part1(input: &str) -> i32 {
    let instructions = process_data(input);
    let (acc, _valid) = check_instructions(instructions);

    return acc;
}

fn part2(input: &str) -> i32 {
    let instructions = process_data(input);
    let acc: i32 = 0;

    let (acc_value, valid) = check_instructions(instructions.clone());
    if valid {
        return acc_value;
    }

    for (i, (instruction, amount)) in instructions.iter().enumerate() {
        let mut clone_instructions = instructions.clone();

        if instruction == "nop" {
            clone_instructions[i] = ("jmp".to_string(), *amount);
        } else if instruction == "jmp" {
            clone_instructions[i] = ("nop".to_string(), *amount);
        }

        let (acc_value, is_valid) = check_instructions(clone_instructions.clone());
        if is_valid {
            return acc_value;
        }
    }

    return acc;
}

pub fn solve() {
    let input: &str = include_str!("input.txt");

    let time1 = Instant::now();
    let solution1 = part1(input);
    let elapsed1 = time1.elapsed();
    println!("Day 8, Part 1: {:?} ({:?})", solution1, elapsed1);
    let time2 = Instant::now();
    let solution2 = part2(input);
    let elapsed2 = time2.elapsed();
    println!("Day 8, Part 2: {:?} ({:?})", solution2, elapsed2);
}
