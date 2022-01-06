use std::collections::HashSet;

fn process_data(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn get_seat_ids(passes: Vec<&str>) -> Vec<i32> {
    let mut seat_ids: Vec<i32> = Vec::new();

    for pass in passes {
        let mut row_low: i32 = 0;
        let mut row_high: i32 = 127;
        let mut col_low: i32 = 0;
        let mut col_high: i32 = 7;

        for c in pass.chars() {
            if c == 'F' {
                row_high = ((row_high - row_low) / 2) + row_low - 1;
            } else if c == 'B' {
                row_low = ((row_high - row_low) / 2) + row_low + 1;
            } else if c == 'L' {
                col_high = ((col_high - col_low) / 2) + col_low - 1;
            } else if c == 'R' {
                col_low = ((col_high - col_low) / 2) + col_low + 1;
            }
        }
        let id: i32 = row_low * 8 + col_low;
        seat_ids.push(id);
    }

    return seat_ids;
}

fn part1(input: &str) -> i32 {
    let passes: Vec<&str> = process_data(input);
    let seat_ids: Vec<i32> = get_seat_ids(passes);

    return *seat_ids.iter().max().unwrap();
}


fn part2(input: &str) -> i32 {
    let passes: Vec<&str> = process_data(input);
    let seat_ids: HashSet<i32> = get_seat_ids(passes).iter().cloned().collect();
    let min: i32 = *seat_ids.iter().min().unwrap();
    let max: i32 = *seat_ids.iter().max().unwrap();
    let all_seats: HashSet<i32> = (min..max).collect();

    let free_seats: Vec<&i32> = Vec::from_iter::<HashSet<&i32>>(all_seats.difference(&seat_ids).collect());

    return *free_seats[0];
}

fn main() {
    let input: &str = include_str!("input.txt");

    println!("Day 5, Part 1: {:?}", part1(input));
    println!("Day 5, Part 2: {:?}", part2(input));
}
