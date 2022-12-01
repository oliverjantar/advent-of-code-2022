use {shared::file_reader::FileReader, std::error::Error};

fn main() {
    task_1().unwrap();
    task_2().unwrap();
}

fn task_2() -> Result<(), Box<dyn Error>> {
    let mut data = Vec::new();

    let mut current = 0;
    for line in FileReader::open("./input", 16)? {
        match line {
            Ok(line) => match line[..line.len() - 1].parse::<i32>() {
                Ok(number) => current += number,
                Err(_) => {
                    data.push(current);
                    current = 0
                }
            },
            Err(_) => panic!("error while reading file"),
        }
    }
    data.push(current);

    data.sort();
    data.reverse();

    println!("biggest: {}", data[0] + data[1] + data[2]);

    Ok(())
}

fn task_1() -> Result<(), Box<dyn Error>> {
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

    println!("sum of top three: {}", biggest);

    Ok(())
}
