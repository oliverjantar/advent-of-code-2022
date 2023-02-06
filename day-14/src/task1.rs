use std::{collections::HashSet, error::Error};

use shared::file_reader::FileReader;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut map: HashSet<Point> = HashSet::new();

    for line in FileReader::open("./input", 40)? {
        if let Ok(line) = line {
            let line = line.trim_end();

            let points = Point::get_points_from_str(line);

            map.extend(points.iter());
        }
    }

    let min = map.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;

    let mut x = Sand {
        map,
        lowest_point: min,
    };

    println!("{}", x.fall_until_it_can());
    Ok(())
}

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
struct Point(i32, i32);

impl Point {
    fn new_from_str(value: &str) -> Self {
        let coordinates: Vec<&str> = value.split(',').collect();
        Point(
            coordinates[0].parse::<i32>().unwrap(),
            coordinates[1].parse::<i32>().unwrap(),
        )
    }

    fn get_all_points(p1: &Point, p2: &Point) -> Vec<Point> {
        let mut points = Vec::new();
        match p1.0 == p2.0 {
            true => match p1.1 < p2.1 {
                true => {
                    for i in p1.1..=p2.1 {
                        points.push(Point(p1.0, i));
                    }
                }
                false => {
                    for i in p2.1..=p1.1 {
                        points.push(Point(p1.0, i));
                    }
                }
            },
            false => match p1.0 < p2.0 {
                true => {
                    for i in p1.0..=p2.0 {
                        points.push(Point(i, p1.1));
                    }
                }
                false => {
                    for i in p2.0..=p1.0 {
                        points.push(Point(i, p1.1));
                    }
                }
            },
        }

        points
    }

    fn get_points_from_str(line: &str) -> HashSet<Point> {
        let points_string: Vec<&str> = line.split(" -> ").collect();

        (1..points_string.len())
            .map(|x| {
                Point::get_all_points(
                    &Point::new_from_str(points_string[x - 1]),
                    &Point::new_from_str(points_string[x]),
                )
            })
            .fold(HashSet::new(), |mut acc: HashSet<Point>, x| {
                acc.extend(x.iter().cloned());
                acc
            })
    }
}

struct Sand {
    map: HashSet<Point>,
    lowest_point: i32,
}

enum SandState {
    Down,
    Left,
    Right,
    Stay,
}

impl Sand {
    fn fall(&mut self) -> bool {
        let mut grain = Point(500, 0);

        loop {
            match Self::check_below(&self.map, &grain) {
                SandState::Down => grain.1 += 1,
                SandState::Left => {
                    grain.0 -= 1;
                    grain.1 += 1;
                }
                SandState::Right => {
                    grain.0 += 1;
                    grain.1 += 1;
                }
                SandState::Stay => {
                    self.map.insert(grain);
                    return true;
                }
            }

            if grain.1 >= self.lowest_point {
                return false;
            }
        }
    }

    fn check_below(map: &HashSet<Point>, grain: &Point) -> SandState {
        if !map.contains(&Point(grain.0, grain.1 + 1)) {
            SandState::Down
        } else if !map.contains(&Point(grain.0 - 1, grain.1 + 1)) {
            SandState::Left
        } else if !map.contains(&Point(grain.0 + 1, grain.1 + 1)) {
            SandState::Right
        } else {
            SandState::Stay
        }
    }

    fn fall_until_it_can(&mut self) -> i32 {
        let mut number_of_sand = 0;

        while self.fall() {
            number_of_sand += 1;
        }
        number_of_sand
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fall_until_it_can() {
        let points = HashSet::from([
            Point(498, 4),
            Point(498, 5),
            Point(498, 6),
            Point(497, 6),
            Point(496, 6),
            Point(503, 4),
            Point(502, 4),
            Point(502, 5),
            Point(502, 6),
            Point(502, 7),
            Point(502, 8),
            Point(502, 9),
            Point(501, 9),
            Point(500, 9),
            Point(499, 9),
            Point(498, 9),
            Point(497, 9),
            Point(496, 9),
            Point(495, 9),
            Point(494, 9),
        ]);

        let mut x = Sand {
            map: points,
            lowest_point: 9,
        };

        assert_eq!(x.fall_until_it_can(), 24);
    }

    #[test]
    fn check_sand_fall() {
        let points = HashSet::from([
            Point(498, 4),
            Point(498, 5),
            Point(498, 6),
            Point(497, 6),
            Point(496, 6),
            Point(503, 4),
            Point(502, 4),
            Point(502, 5),
            Point(502, 6),
            Point(502, 7),
            Point(502, 8),
            Point(502, 9),
            Point(501, 9),
            Point(500, 9),
            Point(499, 9),
            Point(498, 9),
            Point(497, 9),
            Point(496, 9),
            Point(495, 9),
            Point(494, 9),
        ]);

        let mut x = Sand {
            map: points,
            lowest_point: 9,
        };

        assert!(x.fall());

        assert!(x.map.contains(&Point(500, 8)));

        assert!(x.fall());

        assert!(x.map.contains(&Point(499, 8)));

        assert!(x.fall());

        assert!(x.map.contains(&Point(501, 8)));

        assert!(x.fall());

        assert!(x.map.contains(&Point(500, 7)));

        assert!(x.fall());

        assert!(x.map.contains(&Point(498, 8)));
    }

    #[test]
    fn test_get_points_from_string() {
        let line = "498,4 -> 498,6 -> 496,6";

        let points = Point::get_points_from_str(line);

        let expected = HashSet::from([
            Point(498, 4),
            Point(498, 5),
            Point(498, 6),
            Point(497, 6),
            Point(496, 6),
        ]);

        assert_eq!(points, expected);

        let line = "503,4 -> 502,4 -> 502,9 -> 494,9";

        let points = Point::get_points_from_str(line);

        let expected = HashSet::from([
            Point(503, 4),
            Point(502, 4),
            Point(502, 5),
            Point(502, 6),
            Point(502, 7),
            Point(502, 8),
            Point(502, 9),
            Point(501, 9),
            Point(500, 9),
            Point(499, 9),
            Point(498, 9),
            Point(497, 9),
            Point(496, 9),
            Point(495, 9),
            Point(494, 9),
        ]);

        assert_eq!(points, expected);
    }
}
