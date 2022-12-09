use std::{cell::RefCell, error::Error, vec};

use shared::file_reader::FileReader;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut forest = Forest::new();

    for line in FileReader::open("./input", 100)? {
        if let Ok(line) = line {
            let line = line.trim_end();
            let mut row = vec![];
            for c in line.chars() {
                let height = c.to_string().parse::<u8>().unwrap();
                let tree = RefCell::new(Tree::new(height));
                row.push(tree);
            }
            forest.trees.push(row);
        }
    }

    println!("{}", forest.set_scenic_points());

    Ok(())
}
#[derive(PartialEq, Debug)]
struct Forest {
    trees: Vec<Vec<RefCell<Tree>>>,
}

impl Forest {
    fn new() -> Self {
        Forest { trees: vec![] }
    }

    fn set_scenic_points(&self) -> u32 {
        let mut best_scenic_point = 0;
        for (row_index, row) in self.trees.iter().enumerate() {
            for (tree_index, _) in row.iter().enumerate() {
                self.set_tree_visibility(tree_index, row_index);
                if self.trees[row_index][tree_index].borrow().scenic_point > best_scenic_point {
                    best_scenic_point = self.trees[row_index][tree_index].borrow().scenic_point;
                }
            }
        }
        best_scenic_point
    }

    fn set_tree_visibility(&self, x: usize, y: usize) {
        let height = self.trees[y][x].borrow().height;

        let scenic_point = self.check_north(height, x, y)
            * self.check_west(height, x, y)
            * self.check_south(height, x, y, self.trees.len() - 1)
            * self.check_east(height, x, y, self.trees[0].len() - 1);

        self.trees[y][x].borrow_mut().scenic_point = scenic_point
    }

    fn is_taller(&self, height: u8, x: usize, y: usize) -> bool {
        self.trees[y][x].borrow().height < height
    }

    fn check_north(&self, height: u8, x: usize, mut y: usize) -> u32 {
        let mut scenic_point = 0;
        while y > 0 {
            y -= 1;
            scenic_point += 1;

            if !self.is_taller(height, x, y) {
                break;
            }
        }
        scenic_point
    }
    fn check_west(&self, height: u8, mut x: usize, y: usize) -> u32 {
        let mut scenic_point = 0;
        while x > 0 {
            x -= 1;
            scenic_point += 1;

            if !self.is_taller(height, x, y) {
                break;
            }
        }
        scenic_point
    }

    fn check_south(&self, height: u8, x: usize, mut y: usize, south_max: usize) -> u32 {
        let mut scenic_point = 0;
        while y < south_max {
            y += 1;
            scenic_point += 1;

            if !self.is_taller(height, x, y) {
                break;
            }
        }
        scenic_point
    }
    fn check_east(&self, height: u8, mut x: usize, y: usize, east_max: usize) -> u32 {
        let mut scenic_point = 0;
        while x < east_max {
            x += 1;
            scenic_point += 1;

            if !self.is_taller(height, x, y) {
                break;
            }
        }
        scenic_point
    }
}

#[derive(PartialEq, Debug)]
struct Tree {
    height: u8,
    scenic_point: u32,
}

impl Tree {
    fn new(height: u8) -> Self {
        Tree {
            height,
            scenic_point: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prepare_data() -> Forest {
        let mut forest = Forest::new();
        forest.trees = vec![
            vec![
                RefCell::new(Tree::new(3)),
                RefCell::new(Tree::new(0)),
                RefCell::new(Tree::new(3)),
                RefCell::new(Tree::new(7)),
                RefCell::new(Tree::new(3)),
            ],
            vec![
                RefCell::new(Tree::new(2)),
                RefCell::new(Tree::new(5)),
                RefCell::new(Tree::new(5)),
                RefCell::new(Tree::new(1)),
                RefCell::new(Tree::new(2)),
            ],
            vec![
                RefCell::new(Tree::new(6)),
                RefCell::new(Tree::new(5)),
                RefCell::new(Tree::new(3)),
                RefCell::new(Tree::new(3)),
                RefCell::new(Tree::new(2)),
            ],
            vec![
                RefCell::new(Tree::new(3)),
                RefCell::new(Tree::new(3)),
                RefCell::new(Tree::new(5)),
                RefCell::new(Tree::new(4)),
                RefCell::new(Tree::new(9)),
            ],
            vec![
                RefCell::new(Tree::new(3)),
                RefCell::new(Tree::new(5)),
                RefCell::new(Tree::new(3)),
                RefCell::new(Tree::new(9)),
                RefCell::new(Tree::new(0)),
            ],
        ];
        forest
    }

    #[test]
    fn test1() {
        let forest = prepare_data();
        let best_scenic_point = forest.set_scenic_points();

        assert_eq!(best_scenic_point, 8);

        let expected = prepare_data();
        expected.trees[0][0].borrow_mut().scenic_point = 0;
        expected.trees[0][1].borrow_mut().scenic_point = 0;
        expected.trees[0][2].borrow_mut().scenic_point = 0;
        expected.trees[0][3].borrow_mut().scenic_point = 0;
        expected.trees[0][4].borrow_mut().scenic_point = 0;

        expected.trees[1][0].borrow_mut().scenic_point = 0;
        expected.trees[1][1].borrow_mut().scenic_point = 1;
        expected.trees[1][2].borrow_mut().scenic_point = 4;
        expected.trees[1][3].borrow_mut().scenic_point = 1;
        expected.trees[1][4].borrow_mut().scenic_point = 0;

        expected.trees[2][0].borrow_mut().scenic_point = 0;
        expected.trees[2][1].borrow_mut().scenic_point = 6;
        expected.trees[2][2].borrow_mut().scenic_point = 1;
        expected.trees[2][3].borrow_mut().scenic_point = 2;
        expected.trees[2][4].borrow_mut().scenic_point = 0;

        expected.trees[3][0].borrow_mut().scenic_point = 0;
        expected.trees[3][1].borrow_mut().scenic_point = 1;
        expected.trees[3][2].borrow_mut().scenic_point = 8;
        expected.trees[3][3].borrow_mut().scenic_point = 3;
        expected.trees[3][4].borrow_mut().scenic_point = 0;

        expected.trees[4][0].borrow_mut().scenic_point = 0;
        expected.trees[4][1].borrow_mut().scenic_point = 0;
        expected.trees[4][2].borrow_mut().scenic_point = 0;
        expected.trees[4][3].borrow_mut().scenic_point = 0;
        expected.trees[4][4].borrow_mut().scenic_point = 0;

        assert_eq!(forest, expected);
    }
}
