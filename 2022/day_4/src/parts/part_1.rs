use std::{fs, ops::RangeInclusive};

pub fn run() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<&str> = input.split_whitespace().collect();
    let mut overlaps = 0;
    for line in lines {
        let ranges: Vec<RangeInclusive<i32>> = line
            .split(',')
            .map(|range| {
                let parts: Vec<i32> = range
                    .split('-')
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect();
                parts[0]..=parts[1]
            })
            .collect();

        let first_range = ranges[0].clone();
        let second_range = ranges[1].clone();

        if first_range.clone().all(|i| second_range.contains(&i))
            || second_range.clone().all(|i| first_range.contains(&i))
        {
            overlaps += 1;
        }
    }

    println!("{overlaps}");
}
