use std::error::Error;

use shared::file_reader::FileReader;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut cycle = 0;
    let mut reg = 1;
    let mut wait = 0;
    let mut add_to_reg = 0;
    let mut file_iterator = FileReader::open("./input", 15)?;

    let mut read = true;

    let mut total_sum = 0;
    loop {
        cycle += 1;

        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                total_sum += cycle * reg;
                println!("{}", total_sum);
            }
            _ => {}
        };
        if wait == 0 {
            if let Some(line) = file_iterator.next() {
                if let Ok(line) = line {
                    let line: Vec<&str> = line.trim_end().split(' ').collect();

                    match line[0] {
                        "noop" => {}
                        "addx" => {
                            add_to_reg = line[1].parse::<i32>().unwrap();

                            wait = 1;
                        }
                        _ => panic!("command not found"),
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            wait -= 1;
            reg += add_to_reg;
        }
    }

    println!("{}", total_sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(1, 1);
    }
}
