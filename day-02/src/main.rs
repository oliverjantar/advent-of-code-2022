use shared::file_reader::FileReader;
use std::error::Error;

fn main() {
    task_1().unwrap();
    task_2().unwrap();
}

fn task_1() -> Result<(), Box<dyn Error>> {
    let mut score = 0;
    for line in FileReader::open("./input", 4)? {
        if let Ok(line) = line {
            if line.len() > 2 {
                let line: Vec<&str> = line.trim_end().split(' ').collect();
                let p1 = RPS::parse_from_str(line[0]);
                let p2 = RPS::parse_from_str(line[1]);

                let result = Game::evaluate(&p1, &p2);

                let value = p2.get_value();

                score += result + value;
            }
        }
    }

    println!("{}", score);

    Ok(())
}

fn task_2() -> Result<(), Box<dyn Error>> {
    let mut points = 0;
    for line in FileReader::open("./input", 4)? {
        if let Ok(line) = line {
            if line.len() > 2 {
                let line: Vec<&str> = line.trim_end().split(' ').collect();

                let p1 = RPS::parse_from_str(line[0]);
                let res = RSLT::parse_from_str(line[1]);

                let p2 = GameResult::evaluate(&p1, &res);

                points += res.get_value() + p2.get_value();
            }
        }
    }

    println!("{}", points);

    Ok(())
}

struct Game(RPS, RPS);

impl Game {
    pub fn evaluate(p1: &RPS, p2: &RPS) -> i32 {
        match (p1, p2) {
            (&RPS::Rock, &RPS::Paper)
            | (&RPS::Paper, &RPS::Scissors)
            | (&RPS::Scissors, &RPS::Rock) => 6,
            (&RPS::Paper, &RPS::Paper)
            | (&RPS::Rock, &RPS::Rock)
            | (&RPS::Scissors, &RPS::Scissors) => 3,
            _ => 0,
        }
    }
}

#[derive(PartialEq, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    pub fn get_value(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    pub fn parse_from_str(character: &str) -> Self {
        if character == "A" || character == "X" {
            RPS::Rock
        } else if character == "B" || character == "Y" {
            RPS::Paper
        } else if character == "C" || character == "Z" {
            RPS::Scissors
        } else {
            panic!("cannot parse character ");
        }
    }
}

struct GameResult(RPS, RSLT);

impl GameResult {
    pub fn evaluate(p1: &RPS, res: &RSLT) -> RPS {
        match (p1, res) {
            (&RPS::Rock, &RSLT::Win) => RPS::Paper,
            (&RPS::Rock, &RSLT::Lose) => RPS::Scissors,
            (&RPS::Paper, &RSLT::Win) => RPS::Scissors,
            (&RPS::Paper, &RSLT::Lose) => RPS::Rock,
            (&RPS::Scissors, &RSLT::Win) => RPS::Rock,
            (&RPS::Scissors, &RSLT::Lose) => RPS::Paper,
            (_, &RSLT::Draw) => p1.clone(),
        }
    }
}

enum RSLT {
    Win,
    Lose,
    Draw,
}

impl RSLT {
    pub fn parse_from_str(value: &str) -> RSLT {
        if value == "X" {
            RSLT::Lose
        } else if value == "Y" {
            RSLT::Draw
        } else if value == "Z" {
            RSLT::Win
        } else {
            panic!("error while parsing RSLT");
        }
    }

    pub fn get_value(&self) -> i32 {
        match self {
            RSLT::Win => 6,
            RSLT::Lose => 0,
            RSLT::Draw => 3,
        }
    }
}
