use std::fs;

#[derive(Copy, Clone)]
enum Winner {
    Opponent = 0,
    Draw = 3,
    Me = 6,
}

pub fn run() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<&str> = input.split('\n').collect();
    let mut score = 0;
    for line in lines {
        let moves: Vec<&str> = line.split_whitespace().collect();
        let opponent = get_assigned_value(moves.first().unwrap());
        let me = get_assigned_value(moves.last().unwrap());

        let winner = determine_winner(opponent, me) as i32;

        score += winner + me;
    }

    println!("{score}");
}

fn get_assigned_value(r#move: &str) -> i32 {
    match r#move {
        "X" | "A" => 1,
        "Y" | "B" => 2,
        "Z" | "C" => 3,
        _ => 0,
    }
}

const fn determine_winner(mut opponent: i32, me: i32) -> Winner {
    if me == opponent {
        return Winner::Draw;
    }

    opponent -= 1;

    if me % 3 == opponent {
        return Winner::Opponent;
    }

    Winner::Me
}
