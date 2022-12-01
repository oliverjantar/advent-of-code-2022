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
        match line {
            Ok(line) => match line[..line.len() - 1].parse::<i32>() {
                Ok(number) => current += number,
                Err(_) => current = 0,
            },
            Err(_) => panic!("error while reading file"),
        }

        if current > biggest {
            biggest = current;
        }
    }

    println!("biggest: {}", biggest);

    Ok(())
}
