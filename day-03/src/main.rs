use std::error::Error;

use shared::file_reader::FileReader;

fn main() {
    task_1().unwrap();
}

fn task_1() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for line in FileReader::open("./input", 200)? {
        if let Ok(line) = line {
            let line = line.trim_end();

            let res = apply_fn(line);
            if let Some(res) = res {
                let point = get_point(res);
                sum += point;
            }
        }
    }
    println!("{}", sum);
    Ok(())
}

fn apply_fn(rucksack: &str) -> Option<char> {
    let right = &rucksack[..rucksack.len() / 2];
    let left = &rucksack[rucksack.len() / 2..];

    for c in right.chars() {
        if left.contains(c) {
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
    fn test1() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_sample_data() {
        let r1 = ("vJrwpWtwJgWrhcsFMMfFFhFp", 'p');
        let r2 = ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 'L');
        let r3 = ("PmmdzqPrVvPwwTWBwg", 'P');
        let r4 = ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 'v');
        let r5 = ("ttgJtRGJQctTZtZT", 't');
        let r6 = ("CrZsJsPPZsGzwwsLwLmpwMDw", 's');

        let data = vec![r1, r2, r3, r4, r5, r6];

        let mut sum = 0;
        for rucksack in data {
            assert_eq!(apply_fn(rucksack.0), Some(rucksack.1));
            sum += get_point(rucksack.1);
        }
        assert_eq!(sum, 157)
    }
}
