use serde::{Deserialize, Serialize};
use std::{error::Error, vec};

use shared::file_reader::FileReader;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut result = 0;
    let mut file_reader = FileReader::open("./input", 10)?;
    let mut index = 0;
    loop {
        match file_reader.next() {
            Some(line1) => match file_reader.next() {
                Some(line2) => {
                    index += 1;
                    let line1 = line1.unwrap();
                    let line1 = line1.trim_end();
                    let line2 = line2.unwrap();
                    let line2 = line2.trim_end();

                    let item1: Item = serde_json::from_str(line1).unwrap();
                    let item2: Item = serde_json::from_str(line2).unwrap();

                    if Item::compare(&item1, &item2).unwrap() {
                        result += index;
                    }
                    file_reader.next();
                }
                None => {
                    panic!("Input is invalid");
                }
            },
            None => {
                break;
            }
        }
    }

    println!("{}", result);
    Ok(())
}

pub fn run_task2() -> Result<(), Box<dyn Error>> {
    let mut result = 0;
    let mut items = vec![];
    for line in FileReader::open("./input", 10)? {
        if let Ok(line) = line {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let item: Item = serde_json::from_str(line).unwrap();
            items.push(item);
        }
    }

    Item::sort_packets(&mut items);

    let delimiter1: Item = serde_json::from_str("[[2]]").unwrap();
    let delimiter2: Item = serde_json::from_str("[[6]]").unwrap();

    for (index, item) in items.iter().enumerate() {
        if item == &delimiter1 {
            result = index + 1;
        } else if item == &delimiter2 {
            result *= index + 1;
            break;
        }
    }

    println!("{}", result);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
enum Item {
    Int(i32),
    List(Vec<Item>),
}

impl Item {
    fn compare(item1: &Item, item2: &Item) -> Option<bool> {
        return match (item1, item2) {
            (Item::Int(val1), Item::Int(val2)) => match val1.cmp(val2) {
                std::cmp::Ordering::Less => Some(true),
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => Some(false),
            },
            (Item::Int(val1), Item::List(_)) => {
                let list1 = Item::List(vec![Item::Int(*val1)]);

                return Item::compare(&list1, item2);
            }
            (Item::List(_), Item::Int(val2)) => {
                let list2 = Item::List(vec![Item::Int(*val2)]);

                return Item::compare(item1, &list2);
            }
            (Item::List(list1), Item::List(list2)) => {
                for i in 0..list1.len() {
                    if i >= list2.len() {
                        return Some(false);
                    }

                    let item1 = &list1[i];
                    let item2 = &list2[i];

                    if let Some(value) = Item::compare(item1, item2) {
                        return Some(value);
                    }
                }
                if list1.len() < list2.len() {
                    return Some(true);
                }
                return None;
            }
        };
    }

    fn sort_packets(data: &mut Vec<Item>) {
        let mut index = 0;
        while index < data.len() - 1 {
            let item1 = &data[index];
            let item2 = &data[index + 1];

            if !Item::compare(item1, item2).unwrap() {
                data.swap(index, index + 1);
                index = 0;
                continue;
            }
            index += 1;
        }
        // for item in data.iter() {
        //     println!("{:?}", item);
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Item::{Int, List};

    #[test]
    fn parse_variants() {
        let line = "[1,1,3,1,1]";
        let parsed_line: Item = serde_json::from_str(line).unwrap();

        let expected = List(vec![Int(1), Int(1), Int(3), Int(1), Int(1)]);

        assert_eq!(expected, parsed_line);

        let line = "[[1],[2,3,4]]";

        let parsed_line: Item = serde_json::from_str(line).unwrap();

        let expected = List(vec![List(vec![Int(1)]), List(vec![Int(2), Int(3), Int(4)])]);

        assert_eq!(expected, parsed_line);

        let line = "[[8,7,6]]";

        let parsed_line: Item = serde_json::from_str(line).unwrap();

        let expected = List(vec![List(vec![Int(8), Int(7), Int(6)])]);

        assert_eq!(expected, parsed_line);

        let line = "[1,[2,[3,[4,[5,6,7]]]],8,9]";

        let parsed_line: Item = serde_json::from_str(line).unwrap();

        let expected = List(vec![
            Int(1),
            List(vec![
                Int(2),
                List(vec![
                    Int(3),
                    List(vec![Int(4), List(vec![Int(5), Int(6), Int(7)])]),
                ]),
            ]),
            Int(8),
            Int(9),
        ]);

        assert_eq!(expected, parsed_line);
    }

    #[test]
    fn test_compare() {
        let item1: Item = serde_json::from_str("[1,1,3,1,1]").unwrap();
        let item2: Item = serde_json::from_str("[1,1,5,1,1]").unwrap();

        assert_eq!(true, Item::compare(&item1, &item2).unwrap());

        let item1: Item = serde_json::from_str("[[1],[2,3,4]]").unwrap();
        let item2: Item = serde_json::from_str("[[1],4]").unwrap();

        assert_eq!(true, Item::compare(&item1, &item2).unwrap());

        let item1: Item = serde_json::from_str("[9]").unwrap();
        let item2: Item = serde_json::from_str("[[8,7,6]]").unwrap();

        assert_eq!(false, Item::compare(&item1, &item2).unwrap());

        let item1: Item = serde_json::from_str("[[4,4],4,4]").unwrap();
        let item2: Item = serde_json::from_str("[[4,4],4,4,4]").unwrap();

        assert_eq!(true, Item::compare(&item1, &item2).unwrap());

        let item1: Item = serde_json::from_str("[7,7,7,7]").unwrap();
        let item2: Item = serde_json::from_str("[7,7,7]").unwrap();

        assert_eq!(false, Item::compare(&item1, &item2).unwrap());

        let item1: Item = serde_json::from_str("[]").unwrap();
        let item2: Item = serde_json::from_str("[3]").unwrap();

        assert_eq!(true, Item::compare(&item1, &item2).unwrap());

        let item1: Item = serde_json::from_str("[[[]]]").unwrap();
        let item2: Item = serde_json::from_str("[[]]").unwrap();

        assert_eq!(false, Item::compare(&item1, &item2).unwrap());

        let item1: Item = serde_json::from_str("[1,2,3,[1,2,3],4,1]").unwrap();
        let item2: Item = serde_json::from_str("[1,2,3,[1,2,3],4,0]").unwrap();

        assert_eq!(false, Item::compare(&item1, &item2).unwrap());

        let item1: Item = serde_json::from_str("[[8,[[7]]]]").unwrap();
        let item2: Item = serde_json::from_str("[[[[8]]]]").unwrap();

        assert_eq!(false, Item::compare(&item1, &item2).unwrap());

        let item1: Item = serde_json::from_str("[[8,[[7]]]]").unwrap();
        let item2: Item = serde_json::from_str("[[[[8],2]]]").unwrap();

        assert_eq!(true, Item::compare(&item1, &item2).unwrap());
    }
}
