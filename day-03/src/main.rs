use shared::file_reader::FileReader;
fn main() {
    println!("Hello, world!");
}

fn apply_fn(rucksack: &str) -> char {
    'a'
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

        for rucksack in data {
            assert_eq!(apply_fn(rucksack.0), rucksack.1);
        }
    }
}
