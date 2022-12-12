use regex::Regex;
use std::{cmp::min, fs, ops::Range};

struct Command {
    count: usize,
    from: usize,
    to: usize,
}

pub fn run() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut stacks = get_stacks(&input);
    let commands = get_commands(&input);
    stacks = move_crates(stacks, commands);
    print_solution(&stacks);
}

fn get_stacks(input: &str) -> Vec<Vec<String>> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let segment_size = 4;
    let segments = (lines[0].len() + 1) / segment_size;

    let columns_regex = Regex::new(r"( *\d){1,} ?").unwrap();
    let (column_line, _) = lines
        .iter()
        .enumerate()
        .find(|(_, s)| columns_regex.is_match(s))
        .unwrap();

    let mut stacks: Vec<Vec<String>> = vec![];

    for l in lines.iter().take(column_line) {
        for s in 0..segments {
            let segment = l
                .get(Range {
                    start: s * segment_size,
                    end: min(l.len(), s * segment_size + segment_size),
                })
                .unwrap();

            let letter = segment.replace(['[', ']', ' '], "");
            if letter.is_empty() && stacks.get(s).is_none() {
                stacks.insert(s, Vec::new());
                continue;
            }

            if letter.is_empty() {
                continue;
            }

            if stacks.get(s).is_none() {
                stacks.insert(s, vec![letter]);
                continue;
            }

            stacks[s].insert(0, letter);
        }
    }
    stacks
}

fn get_commands(input: &str) -> Vec<Command> {
    let command_list: Vec<&str> = input.split("\n\n").last().unwrap().split('\n').collect();
    let mut commands: Vec<Command> = vec![];

    for command in command_list {
        if command.is_empty() {
            continue;
        }
        let digits = extract_numbers(command);

        commands.push(Command {
            count: digits[0],
            from: digits[1] - 1,
            to: digits[2] - 1,
        });
    }

    commands
}

fn extract_numbers(input: &str) -> Vec<usize> {
    let mut numbers = Vec::new();
    for word in input.split_whitespace() {
        if let Ok(number) = word.parse::<usize>() {
            numbers.push(number);
        }
    }
    numbers
}

fn move_crates(mut stacks: Vec<Vec<String>>, commands: Vec<Command>) -> Vec<Vec<String>> {
    for command in commands {
        for _ in 0..command.count {
            let cr = stacks[command.from].pop().unwrap();
            stacks[command.to].push(cr);
        }
    }

    stacks
}

fn print_solution(stacks: &Vec<Vec<String>>) {
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}