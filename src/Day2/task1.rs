use std::io::BufRead;

enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn cmp(&self, other: Self) -> i64 {
        match (self, other) {
            (Play::Rock, Play::Paper) => -1,
            (Play::Rock, Play::Scissors) => 1,
            (Play::Paper, Play::Rock) => 1,
            (Play::Paper, Play::Scissors) => -1,
            (Play::Scissors, Play::Rock) => -1,
            (Play::Scissors, Play::Paper) => 1,
            _ => 0,
        }
    }

    fn ordinal(&self) -> i64 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
}

fn main() {
    fn map_letter_to_play(letter: &str) -> Play {
        match letter {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissors,
            _ => panic!("Cannot find matching move!"),
        }
    }

    let mut score = 0;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let (a, b) = line.split_once(' ').expect("Invalid input");
        let a = map_letter_to_play(a);
        let b = map_letter_to_play(b);

        let result = 3 * (b.cmp(a) + 1) + b.ordinal();
        score += result;
    }

    println!("{}", score);
}
