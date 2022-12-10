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

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Position(i32, i32);

struct Processor {
    head: Position,
    tail: Position,
    moves: HashSet<Position>,
}

impl Processor {
    fn new() -> Self {
        Processor {
            head: Position(0, 0),
            tail: Position(0, 0),
            moves: HashSet::new(),
        }
    }
    fn process_move(&mut self, direction: &str, steps: i32) {
        for _ in 0..steps {
            match direction {
                "U" => self.head.1 += 1,
                "D" => self.head.1 -= 1,
                "L" => self.head.0 -= 1,
                "R" => self.head.0 += 1,
                _ => panic!("move not allowed"),
            }

            self.move_tail();
            self.moves.insert(self.tail.clone());
        }
    }

    fn move_tail(&mut self) {
        match (self.tail.0 - self.head.0, self.tail.1 - self.head.1) {
            (-2, 0) => self.tail.0 += 1,
            (2, 0) => self.tail.0 -= 1,
            (0, 2) => self.tail.1 -= 1,
            (0, -2) => self.tail.1 += 1,
            (-1, -2) => {
                self.tail.0 += 1;
                self.tail.1 += 1
            }
            (1, -2) => {
                self.tail.0 -= 1;
                self.tail.1 += 1;
            }
            (-1, 2) => {
                self.tail.0 += 1;
                self.tail.1 -= 1;
            }
            (1, 2) => {
                self.tail.0 -= 1;
                self.tail.1 -= 1;
            }
            (-2, -1) => {
                self.tail.0 += 1;
                self.tail.1 += 1;
            }
            (-2, 1) => {
                self.tail.0 += 1;
                self.tail.1 -= 1;
            }
            (2, -1) => {
                self.tail.0 -= 1;
                self.tail.1 += 1;
            }
            (2, 1) => {
                self.tail.0 -= 1;
                self.tail.1 -= 1;
            }
            (_, _) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let moves = vec![
            ("R", 4, Position(4, 0), Position(3, 0)),
            ("U", 4, Position(4, 4), Position(4, 3)),
            ("L", 3, Position(1, 4), Position(2, 4)),
            ("D", 1, Position(1, 3), Position(2, 4)),
            ("R", 4, Position(5, 3), Position(4, 3)),
            ("D", 1, Position(5, 2), Position(4, 3)),
            ("L", 5, Position(0, 2), Position(1, 2)),
            ("R", 2, Position(2, 2), Position(1, 2)),
        ];

        let mut x = Processor::new();

        for m in moves {
            x.process_move(m.0, m.1);
            assert_eq!(x.head, m.2);
            assert_eq!(x.tail, m.3);
        }
        assert_eq!(x.moves.len(), 13);
    }
}
