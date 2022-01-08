use std::{collections::HashMap, time::Instant};
use regex::Regex;

fn process_data(input: &str) -> Vec<HashMap<&str, &str>> {
    let data: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut passpords: Vec<HashMap<&str, &str>> = Vec::new();
    for row in data {
        passpords.push(row.split_ascii_whitespace()
            .map(|e: &str| e.split_once(":").unwrap())
            .collect::<HashMap<&str, &str>>());
    }

    return passpords;
}

fn part1(input: &str) -> i32 {
    let required_values: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let passpords: Vec<HashMap<&str, &str>> = process_data(input);
    let mut valid_passpords: i32 = 0;

    for passpord in passpords {
        if required_values.iter().all(|e: &&str| passpord.contains_key(e)) {
            valid_passpords += 1;
        }
    }

    return valid_passpords;
}


fn part2(input: &str) -> i32 {
    let required_values: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let valid_ecl_values: Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let re_height: Regex = Regex::new(r"(\d+)(in|cm)").unwrap();
    let re_hcl: Regex = Regex::new(r"(#)([\da-f]{6})").unwrap();
    let re_pid: Regex = Regex::new(r"^(\d{9})$").unwrap();
    let passpords: Vec<HashMap<&str, &str>> = process_data(input);
    let mut valid_passpords: i32 = 0;

    for passpord in passpords {
        if !required_values.iter().all(|e: &&str| passpord.contains_key(e)) {
            continue;
        }

        let byr: i32 = passpord["byr"].parse().unwrap();
        if byr < 1920 || byr > 2002 { continue; }

        let iyr: i32 = passpord["iyr"].parse().unwrap();
        if iyr < 2010 || iyr > 2020 { continue; }

        let eyr: i32 = passpord["eyr"].parse().unwrap();
        if eyr < 2020 || eyr > 2030 { continue; }

        if re_height.is_match(passpord["hgt"]) {
            let hgt= re_height.captures(passpord["hgt"]).unwrap();
            let hgt_val: i32 = hgt[1].parse().unwrap();
            let hgt_sym: &str = &hgt[2];
            if hgt_sym == "cm" && (hgt_val < 150 || hgt_val > 193) {
                continue;
            } else if hgt_sym == "in" && (hgt_val < 59 || hgt_val > 76) {
                continue;
            }
        } else { continue; }

        let hcl: &str = passpord["hcl"];
        if !re_hcl.is_match(hcl) { continue; }

        let ecl: &str = passpord["ecl"];
        if !valid_ecl_values.contains(&ecl) { continue; }

        let pid: &str = passpord["pid"];
        if !re_pid.is_match(pid) { continue; }

        valid_passpords += 1;
    }

    return valid_passpords;
}

pub fn solve() {
    let input: &str = include_str!("input.txt");

    let time1 = Instant::now();
    let solution1 = part1(input);
    let elapsed1 = time1.elapsed();
    println!("Day 4, Part 1: {:?} ({:?})", solution1, elapsed1);
    let time2 = Instant::now();
    let solution2 = part2(input);
    let elapsed2 = time2.elapsed();
    println!("Day 4, Part 2: {:?} ({:?})", solution2, elapsed2);
}
