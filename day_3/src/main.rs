use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<&str> = input.split_whitespace().collect();
    let groups: Vec<&[&str]> = lines.chunks(3).collect();
    let mut collective_score = 0;
    for group in groups {
        let mut group_vec = group.to_vec();
        let first_elf = group_vec.remove(0);
        let first_elf_chars: Vec<char> = first_elf.chars().collect();
        let mut used_chars: Vec<char> = vec![];

        for c in first_elf_chars {
            if used_chars.contains(&c) {
                continue;
            }
            let remaining_groups = group_vec.clone().into_iter();

            if remaining_groups
                .filter(|elf| elf.chars().any(|x| x == c))
                .count()
                == 2
            {
                collective_score += convert_char_to_priority(c);
                used_chars.push(c);
                continue;
            }
        }
    }
    println!("\nTotal score: {collective_score}");
}

fn convert_char_to_priority(item: char) -> i32 {
    let ascii_offset = if item.is_lowercase() { 96 } else { 38 };

    item as i32 - ascii_offset
}
