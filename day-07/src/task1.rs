use std::{cell::RefCell, collections::HashMap, error::Error, rc::Rc};

use shared::file_reader::FileReader;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut x = Processor::new();
    let mut current = x.root.clone();
    for line in FileReader::open("./input", 100)? {
        if let Ok(line) = line {
            let line = line.trim_end();
            current = x.process_line(current, line);
        }
    }
    Processor::count_sizes(&x.root);
    println!("{}", x.get_total_size());

    print!("least size {}", x.least_size());
    Ok(())
}
#[derive(Debug)]
struct Item {
    path: String,
    size: u64,
    item_type: ItemType,
    child_items: Vec<Rc<RefCell<Item>>>,
    parent: Option<Rc<RefCell<Item>>>,
}

impl Item {
    fn create_root() -> Self {
        Item {
            path: String::from("/"),
            size: 0,
            item_type: ItemType::Directory,
            child_items: vec![],
            parent: None,
        }
    }
    fn new(
        path: String,
        size: u64,
        item_type: ItemType,
        parent: Option<Rc<RefCell<Item>>>,
    ) -> Self {
        Self {
            path,
            size,
            item_type,
            child_items: vec![],
            parent,
        }
    }
}

struct Processor {
    root: Rc<RefCell<Item>>,
    directories: HashMap<String, Rc<RefCell<Item>>>,
}

impl Processor {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(Item::create_root()));
        let mut dirs = HashMap::new();
        dirs.insert(String::from("/"), root.clone());
        Processor {
            root: root,
            directories: dirs,
        }
    }
    fn process_line(&mut self, current: Rc<RefCell<Item>>, line: &str) -> Rc<RefCell<Item>> {
        let params: Vec<&str> = line.split(' ').collect();
        if params[0] == "$" {
            if params[1] == "ls" {
                return current;
            }
            match (params[1], params[2]) {
                ("cd", "..") => return current.borrow().parent.as_ref().unwrap().clone(),
                ("cd", "/") => return self.root.clone(),
                ("cd", _) => {
                    let path = format!("{}{}/", current.borrow().path, params[2]);

                    return self
                        .directories
                        .get(&path)
                        .expect("dir does not exists")
                        .clone();
                }
                (_, _) => return current,
            }
        } else {
            if params[0].parse::<u32>().is_ok() {
                let item = Rc::new(RefCell::new(Item::new(
                    params[1].to_string(),
                    params[0].parse().unwrap(),
                    ItemType::File,
                    Some(current.clone()),
                )));
                current.borrow_mut().child_items.push(item.clone());
            } else {
                let path = format!("{}{}/", current.borrow().path, params[1]);
                let item = Rc::new(RefCell::new(Item::new(
                    path.clone(),
                    0,
                    ItemType::Directory,
                    Some(current.clone()),
                )));
                current.borrow_mut().child_items.push(item.clone());

                self.directories.insert(path, item);
            }
        }

        current
    }

    fn get_total_size(&self) -> u64 {
        let mut total_size = 0;
        for (_, directory) in self.directories.iter() {
            if directory.borrow().size <= 100_000 {
                total_size += directory.borrow().size;
            }
        }
        total_size
    }

    fn least_size(&self) -> u64 {
        let mut smallest = 100_000_000;
        for (_, directory) in self.directories.iter() {
            let size = directory.borrow().size;
            if size >= 6592386 && size < smallest {
                smallest = size;
            }
        }
        smallest
    }

    fn count_sizes(item: &Rc<RefCell<Item>>) {
        let mut size = 0;
        for child in item.borrow().child_items.iter() {
            if child.borrow().item_type == ItemType::Directory {
                Processor::count_sizes(&child);
            }
            size += child.borrow().size;
        }
        item.borrow_mut().size = size;
    }
}

#[derive(PartialEq, Debug)]
enum ItemType {
    Directory,
    File,
}

#[cfg(test)]
mod tests {

    use super::*;

    fn prepare_data() -> Processor {
        let mut x = Processor::new();

        let dir_a = Rc::new(RefCell::new(Item::new(
            String::from("a"),
            0,
            ItemType::Directory,
            Some(Rc::clone(&x.root)),
        )));
        let dir_e = Rc::new(RefCell::new(Item::new(
            String::from("e"),
            0,
            ItemType::Directory,
            Some(Rc::clone(&dir_a)),
        )));
        let file_i = Rc::new(RefCell::new(Item::new(
            String::from("i"),
            584,
            ItemType::File,
            Some(Rc::clone(&dir_e)),
        )));
        dir_e.borrow_mut().child_items.push(Rc::clone(&file_i));

        let file_f = Rc::new(RefCell::new(Item::new(
            String::from("f"),
            29116,
            ItemType::File,
            Some(Rc::clone(&dir_a)),
        )));
        let file_g = Rc::new(RefCell::new(Item::new(
            String::from("g"),
            2557,
            ItemType::File,
            Some(Rc::clone(&dir_a)),
        )));
        let file_h = Rc::new(RefCell::new(Item::new(
            String::from("h.lst"),
            62596,
            ItemType::File,
            Some(Rc::clone(&dir_a)),
        )));

        dir_a.borrow_mut().child_items = vec![Rc::clone(&dir_e), file_f, file_g, file_h];

        let file_b = Rc::new(RefCell::new(Item::new(
            String::from("b.txt"),
            14848514,
            ItemType::File,
            Some(Rc::clone(&x.root)),
        )));
        let file_c = Rc::new(RefCell::new(Item::new(
            String::from("c.dat"),
            8504156,
            ItemType::File,
            Some(Rc::clone(&x.root)),
        )));
        let dir_d = Rc::new(RefCell::new(Item::new(
            String::from("d"),
            0,
            ItemType::Directory,
            Some(Rc::clone(&x.root)),
        )));

        x.root.borrow_mut().child_items =
            vec![Rc::clone(&dir_a), file_b, file_c, Rc::clone(&dir_d)];

        let file_j = Rc::new(RefCell::new(Item::new(
            String::from("j"),
            4060174,
            ItemType::File,
            Some(Rc::clone(&dir_d)),
        )));
        let file_d_log = Rc::new(RefCell::new(Item::new(
            String::from("d.log"),
            8033020,
            ItemType::File,
            Some(Rc::clone(&dir_d)),
        )));
        let file_d_ext = Rc::new(RefCell::new(Item::new(
            String::from("d.ext"),
            5626152,
            ItemType::File,
            Some(Rc::clone(&dir_d)),
        )));
        let file_k = Rc::new(RefCell::new(Item::new(
            String::from("k"),
            7214296,
            ItemType::File,
            Some(Rc::clone(&dir_d)),
        )));

        dir_d.borrow_mut().child_items = vec![file_j, file_d_log, file_d_ext, file_k];

        x.directories.insert(String::from("/"), Rc::clone(&x.root));
        x.directories.insert(String::from("a"), Rc::clone(&dir_a));
        x.directories.insert(String::from("e"), Rc::clone(&dir_e));
        x.directories.insert(String::from("d"), Rc::clone(&dir_d));

        x
    }

    #[test]
    fn test() {
        let processor = prepare_data();

        processor.directories["e"].borrow_mut().size = 584;
        processor.directories["a"].borrow_mut().size = 94853;
        processor.directories["d"].borrow_mut().size = 24933642;
        processor.directories["/"].borrow_mut().size = 48381165;

        assert_eq!(processor.get_total_size(), 95437);
    }

    #[test]
    fn test2() {
        let processor = prepare_data();
        Processor::count_sizes(&processor.root);
        assert_eq!(processor.get_total_size(), 95437);
    }

    #[test]
    fn test3() {
        let lines = vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];

        let mut processor = Processor::new();
        let mut current = processor.root.clone();
        for line in lines {
            current = processor.process_line(current, line);
        }
        Processor::count_sizes(&processor.root);
        println!("{}", processor.get_total_size());

        assert_eq!(processor.get_total_size(), 95437);
    }
}
