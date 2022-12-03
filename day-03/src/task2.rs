use std::error::Error;

use shared::file_reader::FileReader;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    let mut trinity: (String, String, String) = ("".to_string(), "".to_string(), "".to_string());
    let mut line_number = 1;
    for line in FileReader::open("./input", 200)? {
        if let Ok(line) = line {
            let line = line.trim_end();

            match line_number % 3 {
                0 => {
                    trinity.2 = line.to_string();
                    let res = apply_fn(trinity);
                    sum += get_point(res.unwrap());
                    trinity = ("".to_string(), "".to_string(), "".to_string());
                }
                1 => trinity.0 = line.to_string(),
                2 => trinity.1 = line.to_string(),
                _ => panic!("error"),
            }
            line_number += 1;
        }
    }
    println!("{}", sum);
    Ok(())
}

fn apply_fn(trinity: (String, String, String)) -> Option<char> {
    for c in trinity.0.chars() {
        if trinity.1.contains(c) && trinity.2.contains(c) {
            return Some(c);
        }
    }
    None
}

fn get_point(c: char) -> i32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!("invalid character"),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sample_data() {
        let r1 = (
            (
                String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
                String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
                String::from("PmmdzqPrVvPwwTWBwg"),
            ),
            'r',
        );
        let r2 = (
            (
                String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
                String::from("ttgJtRGJQctTZtZT"),
                String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
            ),
            'Z',
        );

        let data = vec![r1, r2];

        let mut sum = 0;
        for rucksack in data {
            assert_eq!(apply_fn(rucksack.0), Some(rucksack.1));
            sum += get_point(rucksack.1);
        }
        assert_eq!(sum, 70)
    }
}
