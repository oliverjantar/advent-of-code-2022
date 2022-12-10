use std::{collections::HashSet, error::Error};

use shared::file_reader::FileReader;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut processor = Processor::new();
    for line in FileReader::open("./input", 10)? {
        if let Ok(line) = line {
            let params: Vec<&str> = line.trim_end().split(' ').collect();
            let steps = params[1].parse::<i32>().unwrap();
            processor.process_move(params[0], steps);
        }
    }

    println!("{}", processor.moves.len());
    Ok(())
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Copy)]
struct Position(i32, i32);

struct Processor {
    body: Vec<Position>,
    moves: HashSet<Position>,
}

impl Processor {
    fn new() -> Self {
        Processor {
            body: Vec::from([Position(0, 0); 10]),
            moves: HashSet::new(),
        }
    }
    fn process_move(&mut self, direction: &str, steps: i32) {
        for _ in 0..steps {
            match direction {
                "U" => self.body[0].1 += 1,
                "D" => self.body[0].1 -= 1,
                "L" => self.body[0].0 -= 1,
                "R" => self.body[0].0 += 1,
                _ => panic!("move not allowed"),
            }

            self.move_tail();
            self.moves.insert(self.body.last().unwrap().clone());
        }
    }

    fn move_tail(&mut self) {
        for i in 0..self.body.len() - 1 {
            match (
                self.body[i + 1].0 - self.body[i].0,
                self.body[i + 1].1 - self.body[i].1,
            ) {
                (-2, 0) => self.body[i + 1].0 += 1,
                (2, 0) => self.body[i + 1].0 -= 1,
                (0, 2) => self.body[i + 1].1 -= 1,
                (0, -2) => self.body[i + 1].1 += 1,
                (-1, -2) => {
                    self.body[i + 1].0 += 1;
                    self.body[i + 1].1 += 1
                }
                (1, -2) => {
                    self.body[i + 1].0 -= 1;
                    self.body[i + 1].1 += 1;
                }
                (-1, 2) => {
                    self.body[i + 1].0 += 1;
                    self.body[i + 1].1 -= 1;
                }
                (1, 2) => {
                    self.body[i + 1].0 -= 1;
                    self.body[i + 1].1 -= 1;
                }
                (-2, -1) => {
                    self.body[i + 1].0 += 1;
                    self.body[i + 1].1 += 1;
                }
                (-2, 1) => {
                    self.body[i + 1].0 += 1;
                    self.body[i + 1].1 -= 1;
                }
                (2, -1) => {
                    self.body[i + 1].0 -= 1;
                    self.body[i + 1].1 += 1;
                }
                (2, 1) => {
                    self.body[i + 1].0 -= 1;
                    self.body[i + 1].1 -= 1;
                }
                (-2, -2) => {
                    self.body[i + 1].0 += 1;
                    self.body[i + 1].1 += 1;
                }
                (-2, 2) => {
                    self.body[i + 1].0 += 1;
                    self.body[i + 1].1 -= 1;
                }
                (2, -2) => {
                    self.body[i + 1].0 -= 1;
                    self.body[i + 1].1 += 1;
                }
                (2, 2) => {
                    self.body[i + 1].0 -= 1;
                    self.body[i + 1].1 -= 1;
                }
                (_, _) => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let moves = vec![
            ("R", 5),
            ("U", 8),
            ("L", 8),
            ("D", 3),
            ("R", 17),
            ("D", 10),
            ("L", 25),
            ("U", 20),
        ];

        let mut x = Processor::new();

        for m in moves {
            x.process_move(m.0, m.1);
        }
        let mut expected = HashSet::new();
        expected.insert(Position(0, 0));
        expected.insert(Position(1, 1));
        expected.insert(Position(2, 2));
        expected.insert(Position(1, 3));
        expected.insert(Position(2, 4));
        expected.insert(Position(3, 5));
        expected.insert(Position(4, 5));
        expected.insert(Position(5, 5));
        expected.insert(Position(6, 4));
        expected.insert(Position(7, 3));

        expected.insert(Position(8, 2));
        expected.insert(Position(8, 1));
        expected.insert(Position(10, 0));
        expected.insert(Position(9, -1));
        expected.insert(Position(8, -2));
        expected.insert(Position(7, -3));
        expected.insert(Position(6, -4));
        expected.insert(Position(5, -5));
        expected.insert(Position(4, -5));
        expected.insert(Position(3, -5));

        expected.insert(Position(2, -5));
        expected.insert(Position(1, -5));
        expected.insert(Position(0, -5));
        expected.insert(Position(-1, -5));
        expected.insert(Position(-2, -5));
        expected.insert(Position(-3, -4));
        expected.insert(Position(-4, -3));
        expected.insert(Position(-5, -2));
        expected.insert(Position(-6, -1));
        expected.insert(Position(-7, 0));

        expected.insert(Position(-8, 1));
        expected.insert(Position(-9, 2));
        expected.insert(Position(-10, 3));
        expected.insert(Position(-11, 4));
        expected.insert(Position(-11, 5));
        expected.insert(Position(-12, 6));

        assert_eq!(x.moves.len(), 36);
    }
}
