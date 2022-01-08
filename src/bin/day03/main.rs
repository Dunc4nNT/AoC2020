use std::time::Instant;

struct Coordinates {
    x: usize,
    y: usize
}

fn process_data(input: &str) -> Vec<Vec<bool>> {
    let rows: Vec<&str> = input.lines().collect();
    let mut map: Vec<Vec<bool>> = Vec::new();

    for row in rows {
        let mut maprow: Vec<bool> = Vec::new();
        for c in row.chars() {
            if c == '#' {
                maprow.push(true);
            } else {
                maprow.push(false);
            }
        }
        map.push(maprow);
    }
    return map;
}

fn part1(input: &str, slope: Coordinates) -> i32 {
    let map: Vec<Vec<bool>> = process_data(input);
    let mut pos: Coordinates =  Coordinates{x:0, y:0};
    let mut trees_hit = 0;

    while pos.y <= map.len().try_into().unwrap() {
        if pos.y >= map.len() {
            break;
        }
        if pos.x >= map[0].len() {
            pos.x -= map[0].len();
        }
        if map[pos.y][pos.x] {
            trees_hit += 1;
        }
        pos.x += slope.x;
        pos.y += slope.y;
    }

    return trees_hit;
}


fn part2(input: &str) -> i64 {
    let mut number_of_trees: i64 = 0;
    let slopes: Vec<Coordinates> = get_slopes_part2();
    for slope in slopes {
        let trees_hit: i32 = part1(input, slope);
        if number_of_trees > 0 {
            number_of_trees *= i64::try_from(trees_hit).unwrap();
        } else if trees_hit > 0 {
            number_of_trees += 1 * i64::try_from(trees_hit).unwrap();
        }
    }

    return number_of_trees;
}

fn get_slopes_part2() -> Vec<Coordinates> {
    let mut slopes: Vec<Coordinates> = Vec::new();
    slopes.push(Coordinates{x: 1, y:1});
    slopes.push(Coordinates{x: 3, y:1});
    slopes.push(Coordinates{x: 5, y:1});
    slopes.push(Coordinates{x: 7, y:1});
    slopes.push(Coordinates{x: 1, y:2});

    return slopes;
}

fn main() {
    let input: &str = include_str!("input.txt");

    let time1 = Instant::now();
    let solution1 = part1(input, Coordinates{x:3, y:1});
    let elapsed1 = time1.elapsed();
    println!("Day 3, Part 1: {:?} ({:?})", solution1, elapsed1);
    let time2 = Instant::now();
    let solution2 = part2(input);
    let elapsed2 = time2.elapsed();
    println!("Day 3, Part 2: {:?} ({:?})", solution2, elapsed2);
}
