use std::fs::read_to_string;

fn part_one() -> i32 {
    let content = read_to_string("input.txt").unwrap();
    return content
        .lines()
        .map(|x| {
            let nums = x
                .split("")
                .filter_map(|c| c.parse::<i32>().ok())
                .collect::<Vec<i32>>();

            match (nums.first(), nums.last()) {
                (Some(first), Some(last)) => first * 10 + last,
                _ => 0,
            }
        })
        .sum();
}

fn part_two() -> i32 {
    let num_strings: Vec<(&str, i32)> = vec![
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ];
    let content = read_to_string("input.txt").unwrap();
    return content
        .lines()
        .map(|line| {
            let mut nums = num_strings
                .iter()
                .flat_map(|(s, i)| line.match_indices(s).map(|(p, _)| (*i, p)))
                .collect::<Vec<_>>();
            nums.sort_by(|a, b| a.1.cmp(&b.1));
            match (nums.first(), nums.last()) {
                (Some(first), Some(last)) => first.0 * 10 + last.0,
                _ => 0,
            }
        })
        .sum();
}

fn main() {
    println!("parse one: {}, part two: {}", part_one(), part_two());
}
