use std::{collections::HashSet, error::Error};

use shared::file_reader::FileReader;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut result = 0;

    let mut data = HashSet::new();
    for line in FileReader::open("./input", 10)? {
        if let Ok(line) = line {
            let line = line.trim_end();

            data.insert(String::from(line));
        }
    }

    println!("{}", result);
    Ok(())
}

#[derive(Hash, PartialEq, Eq)]
struct Point(i32, i32);

impl Point {
    fn get_all_points(p1: &Point, p2: &Point) -> HashSet<Point> {
        let mut points = HashSet::new();
        match p1.0 == p2.0 {
            true => {
                for i in p1.1..=p2.1 {
                    points.insert(Point(p1.0, i));
                }
            }
            false => {
                for i in p1.1..=p2.1 {
                    points.insert(Point(i, p1.1));
                }
            }
        }

        points
    }

    fn get_points_from_string(line: &str) -> HashSet<Point> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(1, 1);
    }
}
