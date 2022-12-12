use std::{collections::VecDeque, error::Error, vec};

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut monkeys = vec![
        Monkey {
            items: VecDeque::from([74, 73, 57, 77, 74]),
            operation: |x| x * 11,
            decision: (6, 7),
            total_visited_items: 0,
            divide_by: 19,
        },
        Monkey {
            items: VecDeque::from([99, 77, 79]),
            operation: |x| x + 8,
            decision: (6, 0),
            total_visited_items: 0,
            divide_by: 2,
        },
        Monkey {
            items: VecDeque::from([64, 67, 50, 96, 89, 82, 82]),
            operation: |x| x + 1,
            decision: (5, 3),
            total_visited_items: 0,
            divide_by: 3,
        },
        Monkey {
            items: VecDeque::from([88]),
            operation: |x| x * 7,
            decision: (5, 4),
            total_visited_items: 0,
            divide_by: 17,
        },
        Monkey {
            items: VecDeque::from([80, 66, 98, 83, 70, 63, 57, 66]),
            operation: |x| x + 4,
            decision: (0, 1),
            total_visited_items: 0,
            divide_by: 13,
        },
        Monkey {
            items: VecDeque::from([81, 93, 90, 61, 62, 64]),
            operation: |x| x + 7,
            decision: (1, 4),
            total_visited_items: 0,
            divide_by: 7,
        },
        Monkey {
            items: VecDeque::from([69, 97, 88, 93]),
            operation: |x| x * x,
            decision: (7, 2),
            total_visited_items: 0,
            divide_by: 5,
        },
        Monkey {
            items: VecDeque::from([59, 80]),
            operation: |x| x + 6,
            decision: (2, 3),
            total_visited_items: 0,
            divide_by: 11,
        },
    ];
    process_rounds(&mut monkeys, 10_000, 11 * 5 * 7 * 13 * 17 * 3 * 2 * 19);

    for monkey in monkeys {
        println!("{}", monkey.total_visited_items);
    }

    Ok(())
}

fn process_rounds(monkeys: &mut Vec<Monkey>, rounds: i32, worry_number: i64) {
    let mut x = 0;
    while x < rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let mut items_to_throw = vec![];

            while let Some(item) = monkey.items.pop_front() {
                monkey.total_visited_items += 1;
                items_to_throw.push(monkey.monkey_business(item, worry_number));
            }

            for item in items_to_throw.into_iter() {
                monkeys[item.0].items.push_back(item.1);
            }
        }
        // println!("x: {}, {:?}", x, monkeys);

        x += 1;
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    total_visited_items: i32,
    decision: (usize, usize),
    divide_by: i64,
    operation: fn(i64) -> i64,
}

impl Monkey {
    fn monkey_business(&self, item: i64, worry_number: i64) -> (usize, i64) {
        let mut item_worry_level = (self.operation)(item);

        if item_worry_level >= worry_number {
            item_worry_level = item_worry_level % worry_number
        }

        match item_worry_level % self.divide_by {
            0 => (self.decision.0, item_worry_level),
            _ => (self.decision.1, item_worry_level),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut monkeys = vec![
            Monkey {
                items: VecDeque::from([79, 98]),
                operation: |x| x * 19,
                decision: (2, 3),
                total_visited_items: 0,
                divide_by: 23,
            },
            Monkey {
                items: VecDeque::from([54, 65, 75, 74]),
                operation: |x| x + 6,
                decision: (2, 0),
                total_visited_items: 0,
                divide_by: 19,
            },
            Monkey {
                items: VecDeque::from([79, 60, 97]),
                operation: |x| x * x,
                decision: (1, 3),
                total_visited_items: 0,
                divide_by: 13,
            },
            Monkey {
                items: VecDeque::from([74]),
                operation: |x| x + 3,
                decision: (0, 1),
                total_visited_items: 0,
                divide_by: 17,
            },
        ];

        process_rounds(&mut monkeys, 1, 96577);
        assert_eq!(monkeys[0].total_visited_items, 2);
        assert_eq!(monkeys[1].total_visited_items, 4);
        assert_eq!(monkeys[2].total_visited_items, 3);
        assert_eq!(monkeys[3].total_visited_items, 6);

        process_rounds(&mut monkeys, 19, 96577);
        assert_eq!(monkeys[0].total_visited_items, 99);
        assert_eq!(monkeys[1].total_visited_items, 97);
        assert_eq!(monkeys[2].total_visited_items, 8);
        assert_eq!(monkeys[3].total_visited_items, 103);

        process_rounds(&mut monkeys, 9980, 96577);
        assert_eq!(monkeys[0].total_visited_items, 52166);
        assert_eq!(monkeys[1].total_visited_items, 47830);
        assert_eq!(monkeys[2].total_visited_items, 1938);
        assert_eq!(monkeys[3].total_visited_items, 52013);
    }
}
