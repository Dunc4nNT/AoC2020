use std::time::Instant;
use array_tool::vec::Intersect;

fn process_data(input: &str) -> Vec<Vec<&str>> {
    let mut groups: Vec<Vec<&str>> = Vec::new();
    let data: Vec<&str> = input.split("\r\n\r\n").collect();
    for group in data {
        groups.push(group.split("\r\n").collect());
    }

    return groups;
}

fn part1(input: &str) -> i32 {
    let groups: Vec<Vec<&str>> = process_data(input);
    let mut total_answers: i32 = 0;

    for group in groups {
        let mut group_answered: String = String::new();
        for person in group {
            let message_chars: Vec<char> = person.chars().collect();
            for c in message_chars {
                if !group_answered.contains(c) {
                    total_answers += 1;
                    group_answered.push_str(&c.to_string());
                }
            }
        }
    }

    return total_answers;
}


fn part2(input: &str) -> i32 {
    let groups: Vec<Vec<&str>> = process_data(input);
    let mut total_answers: i32 = 0;

    for mut group in groups {
        let mut group_answered: Vec<char> = group[0].chars().collect();
        group.remove(0);
        for person in group {
            let person_answered: Vec<char> = person.chars().collect();
            group_answered = group_answered.intersect(person_answered);
        }
        let group_answers: i32 = group_answered.len().try_into().unwrap();
        total_answers += group_answers;
    }

    return total_answers
}

fn main() {
    let input: &str = include_str!("input.txt");

    let time1 = Instant::now();
    let solution1 = part1(input);
    let elapsed1 = time1.elapsed();
    println!("Day 6, Part 1: {:?} ({:?})", solution1, elapsed1);
    let time2 = Instant::now();
    let solution2 = part2(input);
    let elapsed2 = time2.elapsed();
    println!("Day 6, Part 2: {:?} ({:?})", solution2, elapsed2);
}
