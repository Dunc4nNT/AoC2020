use std::collections::{HashMap, VecDeque, HashSet};
use regex::Regex;

fn process_data(input: &str) -> HashMap<String, Vec<(String, i32)>> {
    let mut data: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    let re_rules = Regex::new(r"([a-z ]+) bags contain (.*)").unwrap();
    let re_content = Regex::new(r"(\d+) ([a-z ]+) bags?").unwrap();

    for line in input.lines() {
        let captures = re_rules.captures(line).unwrap();
        let bag_colour = captures.get(1).unwrap().as_str().to_string();
        let contents: Vec<(String, i32)> = re_content.captures_iter(captures.get(2).unwrap().as_str())
            .map(|e| (
                e.get(2).unwrap().as_str().to_string(),
                e.get(1).unwrap().as_str().parse().unwrap()))
            .collect();

        data.insert(bag_colour, contents);
    }

    return data;
}

fn part1(input: &str) -> i32 {
    let data = process_data(input);
    let mut bags_containing_gold: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<(String, Vec<String>)> = VecDeque::new();

    for (bag, _contents) in &data {
        queue.push_back((bag.clone(), vec![]));
    }

    while !queue.is_empty() {
        let (cur, mut path): (String, Vec<String>) = queue.pop_front().unwrap();
        path.push(cur.clone());
        let contents = data.get(&cur).unwrap();
        for (colour, _amount) in contents.iter() {
            if colour == "shiny gold" {
                for e in &path {
                    bags_containing_gold.insert(e.clone());
                }
            } else {
                queue.push_back((colour.to_string(), path.clone()));
            }
        }
    }

    return bags_containing_gold.len().try_into().unwrap();
}

fn get_sum(data: HashMap<String, Vec<(String, i32)>>, cur: &str) -> i32 {
    let mut total = 0;

    let contents = data.get(cur).unwrap();
    for (colour, amount) in contents.iter() {
        total += amount + amount * get_sum(data.clone(), colour);
    }

    return total
}

fn part2(input: &str) -> i32 {
    let data = process_data(input);

    return get_sum(data, "shiny gold");
}

fn main() {
    let input: &str = include_str!("input.txt");

    println!("Day 7, Part 1: {:?}", part1(input));
    println!("Day 7, Part 2: {:?}", part2(input));
}
