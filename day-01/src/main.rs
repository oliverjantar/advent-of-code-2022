use {
    shared::file_reader::FileReader,
    std::{
        error::Error,
        fs::File,
        io::{self, prelude::*, BufReader},
    },
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut biggest = 0;
    let mut current = 0;
    for line in FileReader::open("./input", 16)? {
        // println!("{:?}", line);

        current = match line {
            Ok(line) => match line.parse() {},
            Err(_) => panic!("error while reading file"),
        }
    }

    Ok(())
}
