use std::{cell::RefCell, collections::VecDeque, error::Error, rc::Rc};

use shared::file_reader::FileReader;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut input = vec![];
    for line in FileReader::open("./input", 10)? {
        if let Ok(line) = line {
            let line: Vec<char> = line.trim_end().chars().collect();
            input.push(line);
        }
    }

    let result = Node::find_shortest_path(&input, (0, 20), (149, 20)).unwrap();

    println!("{}", result);

    let result_task_2 = shortest_path_from_height_1(&input);

    println!("{}", result_task_2);
    Ok(())
}

struct Node {
    position: (usize, usize),
    height: u32,
    distance: Option<u32>,
    neighbours: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn shortest_path(node1: &Rc<RefCell<Node>>, node2: &Rc<RefCell<Node>>) -> Option<u32> {
        let mut items_to_process = VecDeque::from([node1.clone()]);

        loop {
            let item = items_to_process.pop_front();

            match item {
                Some(item) => {
                    if item.borrow().position == node2.borrow().position {
                        return item.borrow().distance;
                    }

                    let neighbour_distance = item.borrow().distance.unwrap() + 1;

                    for i in 0..item.borrow().neighbours.len() {
                        let neighbour = item.borrow().neighbours[i].clone();

                        if neighbour.borrow().distance.is_none()
                            || neighbour.borrow().distance.unwrap() > neighbour_distance
                        {
                            neighbour.borrow_mut().distance = Some(neighbour_distance);
                            items_to_process.push_back(neighbour);
                        }
                    }
                }
                None => return None,
            }
        }
    }

    fn find_shortest_path(
        input: &Vec<Vec<char>>,
        node1: (usize, usize),
        node2: (usize, usize),
    ) -> Option<u32> {
        let mut input_nodes = vec![];
        for (y_index, row) in input.iter().enumerate() {
            let mut new_row = vec![];
            for (x_index, c) in row.iter().enumerate() {
                new_row.push(Rc::new(RefCell::new(Node {
                    height: char_to_int(*c),
                    position: (x_index, y_index),
                    neighbours: vec![],
                    distance: None,
                })));
            }
            input_nodes.push(new_row);
        }

        create_graph(&mut input_nodes);

        let node1 = &input_nodes[node1.1][node1.0];
        let node2 = &input_nodes[node2.1][node2.0];

        node1.borrow_mut().distance = Some(0);

        Node::shortest_path(node1, node2)
    }
}

fn char_to_int(c: char) -> u32 {
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
        'S' => 1,
        'E' => 26,
        _ => panic!("invalid character"),
    }
}

fn create_graph(nodes: &mut Vec<Vec<Rc<RefCell<Node>>>>) {
    for y in 0..nodes.len() {
        for x in 0..nodes[0].len() {
            let node = &nodes[y][x];

            //top
            if y > 0 {
                add_neighbour(node, &nodes[y - 1][x]);
            }

            //bottom
            if y < nodes.len() - 1 {
                add_neighbour(node, &nodes[y + 1][x]);
            }

            //left
            if x > 0 {
                add_neighbour(node, &nodes[y][x - 1]);
            }

            //right
            if x < nodes[0].len() - 1 {
                add_neighbour(node, &nodes[y][x + 1]);
            }
        }
    }
}

fn add_neighbour(node: &Rc<RefCell<Node>>, neighbour: &Rc<RefCell<Node>>) {
    if neighbour.borrow().height - 1 <= node.borrow().height {
        node.borrow_mut().neighbours.push(neighbour.clone());
    }
}

fn shortest_path_from_height_1(input: &Vec<Vec<char>>) -> u32 {
    let starting_positions = [
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (0, 5),
        (0, 6),
        (0, 7),
        (0, 8),
        (0, 9),
        (0, 10),
        (0, 11),
        (0, 12),
        (0, 13),
        (0, 14),
        (0, 15),
        (0, 16),
        (0, 17),
        (0, 18),
        (0, 19),
        (0, 20),
        (0, 21),
        (0, 22),
        (0, 23),
        (0, 24),
        (0, 25),
        (0, 26),
        (0, 27),
        (0, 28),
        (0, 29),
        (0, 30),
        (0, 31),
        (0, 32),
        (0, 33),
        (0, 34),
        (0, 35),
        (0, 36),
        (0, 37),
        (0, 38),
        (0, 39),
        (0, 40),
    ];

    let mut results = vec![];
    for item in starting_positions {
        if let Some(result) = Node::find_shortest_path(&input, item, (149, 20)) {
            results.push(result);
        }
    }

    results.sort();
    results[0]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let input = vec![
            vec!['S', 'a', 'b', 'q', 'p', 'o', 'n', 'm'],
            vec!['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l'],
            vec!['a', 'c', 'c', 's', 'z', 'E', 'x', 'k'],
            vec!['a', 'c', 'c', 't', 'u', 'v', 'w', 'j'],
            vec!['a', 'b', 'd', 'e', 'f', 'g', 'h', 'i'],
        ];

        assert_eq!(Node::find_shortest_path(&input, (0, 0), (5, 2)), Some(31));
    }
}
