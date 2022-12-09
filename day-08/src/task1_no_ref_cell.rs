use std::error::Error;

use shared::file_reader::FileReader;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut forest = Forest::new();

    for line in FileReader::open("./input", 100)? {
        if let Ok(line) = line {
            let line = line.trim_end();
            let mut row = vec![];
            for c in line.chars() {
                let height = c.to_string().parse::<u8>().unwrap();
                let tree = Tree::new(height);
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
    trees: Vec<Vec<Tree>>,
}

impl Forest {
    fn new() -> Self {
        Forest { trees: vec![] }
    }

    fn set_visibility(&mut self) -> u32 {
        let mut count = 0;
        for row_index in 0..self.trees.len() {
            let row = &self.trees[row_index];
            for tree_index in 0..row.len() {
                if self.is_tree_visible(tree_index, row_index) {
                    self.trees[row_index][tree_index].is_visible = true;
                    count += 1;
                }
            }
        }
        count
    }

    fn is_tree_visible(&self, x: usize, y: usize) -> bool {
        let height = self.trees[y][x].height;

        self.check_north(height, x, y)
            || self.check_west(height, x, y)
            || self.check_south(height, x, y, self.trees.len() - 1)
            || self.check_east(height, x, y, self.trees[0].len() - 1)
    }

    fn is_taller(&self, height: u8, x: usize, y: usize) -> bool {
        self.trees[y][x].height < height
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
                Tree::new(3),
                Tree::new(0),
                Tree::new(3),
                Tree::new(7),
                Tree::new(3),
            ],
            vec![
                Tree::new(2),
                Tree::new(5),
                Tree::new(5),
                Tree::new(1),
                Tree::new(2),
            ],
            vec![
                Tree::new(6),
                Tree::new(5),
                Tree::new(3),
                Tree::new(3),
                Tree::new(2),
            ],
            vec![
                Tree::new(3),
                Tree::new(3),
                Tree::new(5),
                Tree::new(4),
                Tree::new(9),
            ],
            vec![
                Tree::new(3),
                Tree::new(5),
                Tree::new(3),
                Tree::new(9),
                Tree::new(0),
            ],
        ];
        forest
    }

    #[test]
    fn test1() {
        let mut forest = prepare_data();
        let count = forest.set_visibility();

        assert_eq!(count, 21);

        let mut expected = prepare_data();
        expected.trees[0][0].is_visible = true;
        expected.trees[0][1].is_visible = true;
        expected.trees[0][2].is_visible = true;
        expected.trees[0][3].is_visible = true;
        expected.trees[0][4].is_visible = true;

        expected.trees[1][0].is_visible = true;
        expected.trees[2][0].is_visible = true;
        expected.trees[3][0].is_visible = true;
        expected.trees[4][0].is_visible = true;

        expected.trees[1][4].is_visible = true;
        expected.trees[2][4].is_visible = true;
        expected.trees[3][4].is_visible = true;
        expected.trees[4][4].is_visible = true;

        expected.trees[4][1].is_visible = true;
        expected.trees[4][2].is_visible = true;
        expected.trees[4][3].is_visible = true;

        expected.trees[1][1].is_visible = true;
        expected.trees[1][2].is_visible = true;
        expected.trees[2][1].is_visible = true;
        expected.trees[2][3].is_visible = true;
        expected.trees[3][2].is_visible = true;

        assert_eq!(forest, expected);
    }
}
