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
        let outcome = get_win_status(moves.last().unwrap());

        let correct_outcome = get_correct_play(opponent, outcome);
        score += outcome as i32 + correct_outcome;
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

fn get_win_status(outcome: &str) -> Winner {
    match outcome {
        "X" => Winner::Opponent,
        "Y" => Winner::Draw,
        _ => Winner::Me,
    }
}

const fn get_correct_play(opponent: i32, outcome: Winner) -> i32 {
    let mut play = match outcome {
        Winner::Draw => opponent,
        Winner::Me => opponent + 1,
        Winner::Opponent => opponent - 1,
    };

    if play > 3 {
        play -= 3;
    } else if play <= 0 {
        play += 3;
    }

    play
}
