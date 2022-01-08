SOLVE_TEMPLATE = """use std::time::Instant;

fn process_data(input: &str) -> usize {
    return 1;
}

fn part1(input: &str) -> usize {
    let data = process_data(input);

    return 1;
}


// fn part2(input: &str) -> usize {

// }

pub fn solve() {
    let input: &str = include_str!("input.txt");

    let time1 = Instant::now();
    let solution1 = part1(input);
    let elapsed1 = time1.elapsed();
    println!("Day {{day}} Part 1: {:?} ({:?})", solution1, elapsed1);
    // let time2 = Instant::now();
    // let solution2 = part2(input);
    // let elapsed2 = time2.elapsed();
    // println!("Day {{day}}, Part 2: {:?} ({:?})", solution2, elapsed2);
}
"""

MOD_TEMPLATE = """pub mod solve;
"""

DAY_MAIN_TEMPLATE = """mod solve;

fn main() {
    solve::solve();
}
"""


SOLUTION_TEMPLATE = """PART 1: 
PART 2: """


BIN_TEMPLATE = """// pub mod day01;
// pub mod day02;
// pub mod day03;
// pub mod day04;
// pub mod day05;
// pub mod day06;
// pub mod day07;
// pub mod day08;
// pub mod day09;
// pub mod day10;
// pub mod day11;
// pub mod day12;
// pub mod day13;
// pub mod day14;
// pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
// pub mod day20;
// pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;
// pub mod day25;

pub fn solve_all() {
    // day01::solve::solve();
    // day02::solve::solve();
    // day03::solve::solve();
    // day04::solve::solve();
    // day05::solve::solve();
    // day06::solve::solve();
    // day07::solve::solve();
    // day08::solve::solve();
    // day09::solve::solve();
    // day10::solve::solve();
    // day11::solve::solve();
    // day12::solve::solve();
    // day13::solve::solve();
    // day14::solve::solve();
    // day15::solve::solve();
    // day16::solve::solve();
    // day17::solve::solve();
    // day18::solve::solve();
    // day19::solve::solve();
    // day20::solve::solve();
    // day21::solve::solve();
    // day22::solve::solve();
    // day23::solve::solve();
    // day24::solve::solve();
    // day25::solve::solve();
}
"""

MAIN_TEMPLATE = """mod bin; 

use std::time::Instant;

use bin::solve_all;

fn main() {
    let timer = Instant::now();
    solve_all();
    println!("Total Time Elapsed: {:?}", timer.elapsed());
}
"""