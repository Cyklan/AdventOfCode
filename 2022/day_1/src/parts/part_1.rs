use std::fs;

pub fn run() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let mut calories: Vec<i32> = vec![];
    let split: Vec<&str> = file_contents.split("\n\n").collect();

    for groups in &split {
        let lines: Vec<&str> = groups.split_whitespace().collect();
        let mut cals = 0;
        for line in &lines {
            cals += line.parse::<i32>().unwrap();
        }
        calories.push(cals);
    }

    calories.sort_unstable();

    println!("{}", calories.last().unwrap());
}
