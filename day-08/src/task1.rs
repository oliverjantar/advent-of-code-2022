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

    println!("{}", forest.set_visibility());

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

    fn set_visibility(&self) -> u32 {
        let mut count = 0;
        for (row_index, row) in self.trees.iter().enumerate() {
            for (tree_index, _) in row.iter().enumerate() {
                self.set_tree_visibility(tree_index, row_index);
                if self.trees[row_index][tree_index].borrow().is_visible {
                    count += 1;
                }
            }
        }
        count
    }

    fn set_tree_visibility(&self, x: usize, y: usize) {
        let height = self.trees[y][x].borrow().height;

        if self.check_north(height, x, y)
            || self.check_west(height, x, y)
            || self.check_south(height, x, y, self.trees.len() - 1)
            || self.check_east(height, x, y, self.trees[0].len() - 1)
        {
            {
                self.trees[y][x].borrow_mut().is_visible = true;
            }
        }
    }

    fn is_taller(&self, height: u8, x: usize, y: usize) -> bool {
        self.trees[y][x].borrow().height < height
    }

    fn check_north(&self, height: u8, x: usize, mut y: usize) -> bool {
        while y > 0 {
            y -= 1;

            if !self.is_taller(height, x, y) {
                return false;
            }
        }
        true
    }
    fn check_west(&self, height: u8, mut x: usize, y: usize) -> bool {
        while x > 0 {
            x -= 1;

            if !self.is_taller(height, x, y) {
                return false;
            }
        }
        true
    }

    fn check_south(&self, height: u8, x: usize, mut y: usize, south_max: usize) -> bool {
        while y < south_max {
            y += 1;

            if !self.is_taller(height, x, y) {
                return false;
            }
        }
        true
    }
    fn check_east(&self, height: u8, mut x: usize, y: usize, east_max: usize) -> bool {
        while x < east_max {
            x += 1;

            if !self.is_taller(height, x, y) {
                return false;
            }
        }
        true
    }
}

#[derive(PartialEq, Debug)]
struct Tree {
    height: u8,
    is_visible: bool,
}

impl Tree {
    fn new(height: u8) -> Self {
        Tree {
            height,
            is_visible: false,
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
        let count = forest.set_visibility();

        assert_eq!(count, 21);

        let expected = prepare_data();
        expected.trees[0][0].borrow_mut().is_visible = true;
        expected.trees[0][1].borrow_mut().is_visible = true;
        expected.trees[0][2].borrow_mut().is_visible = true;
        expected.trees[0][3].borrow_mut().is_visible = true;
        expected.trees[0][4].borrow_mut().is_visible = true;

        expected.trees[1][0].borrow_mut().is_visible = true;
        expected.trees[2][0].borrow_mut().is_visible = true;
        expected.trees[3][0].borrow_mut().is_visible = true;
        expected.trees[4][0].borrow_mut().is_visible = true;

        expected.trees[1][4].borrow_mut().is_visible = true;
        expected.trees[2][4].borrow_mut().is_visible = true;
        expected.trees[3][4].borrow_mut().is_visible = true;
        expected.trees[4][4].borrow_mut().is_visible = true;

        expected.trees[4][1].borrow_mut().is_visible = true;
        expected.trees[4][2].borrow_mut().is_visible = true;
        expected.trees[4][3].borrow_mut().is_visible = true;

        expected.trees[1][1].borrow_mut().is_visible = true;
        expected.trees[1][2].borrow_mut().is_visible = true;
        expected.trees[2][1].borrow_mut().is_visible = true;
        expected.trees[2][3].borrow_mut().is_visible = true;
        expected.trees[3][2].borrow_mut().is_visible = true;

        assert_eq!(forest, expected);
    }
}
