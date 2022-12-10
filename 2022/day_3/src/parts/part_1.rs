use std::fs;

pub fn run() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<&str> = input.split_whitespace().collect();
    let mut collective_score = 0;
    for line in lines {
        let mut chars = line.chars();
        let first_compartment: Vec<char> = chars.by_ref().take(line.len() / 2).collect();
        let second_compartment: Vec<char> = chars.collect();

        let mut used_items: Vec<char> = vec![];

        for c in first_compartment {
            if second_compartment.contains(&c) && !used_items.contains(&c) {
                let prio = convert_char_to_priority(c);
                collective_score += prio;
                used_items.push(c);
            }
        }
    }
    println!("{collective_score}");
}

fn convert_char_to_priority(item: char) -> i32 {
    let ascii_offset = if item.is_lowercase() { 96 } else { 38 };

    item as i32 - ascii_offset
}
