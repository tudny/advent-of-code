use std::io::BufRead;

enum Play {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Loss,
    Draw,
    Win,
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

    fn of_result_via_result(&self, result: Result) -> Self {
        match (self, result) {
            (Play::Rock, Result::Loss) => Play::Scissors,
            (Play::Rock, Result::Draw) => Play::Rock,
            (Play::Rock, Result::Win) => Play::Paper,
            (Play::Paper, Result::Loss) => Play::Rock,
            (Play::Paper, Result::Draw) => Play::Paper,
            (Play::Paper, Result::Win) => Play::Scissors,
            (Play::Scissors, Result::Loss) => Play::Paper,
            (Play::Scissors, Result::Draw) => Play::Scissors,
            (Play::Scissors, Result::Win) => Play::Rock,
        }
    }
}

fn main() {
    fn map_letter_to_play(letter: &str) -> Play {
        match letter {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => panic!("Cannot find matching move!"),
        }
    }

    fn map_letter_to_result(letter: &str) -> Result {
        match letter {
            "X" => Result::Loss,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!("Cannot find matching result!"),
        }
    }

    let mut score = 0;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let (a, b) = line.split_once(' ').expect("Invalid input");
        let a = map_letter_to_play(a);
        let b = map_letter_to_result(b);

        let play = a.of_result_via_result(b);

        let result = 3 * (play.cmp(a) + 1) + play.ordinal();
        score += result;
    }

    println!("{}", score);
}
