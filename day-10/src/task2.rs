use std::error::Error;

use shared::file_reader::FileReader;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut cycle: usize = 0;
    let mut reg: isize = 1;
    let mut wait = 0;
    let mut add_to_reg = 0;
    let mut file_iterator = FileReader::open("./input", 15)?;

    let mut crt: Vec<char> = vec![];

    loop {
        cycle += 1;

        if reg <= cycle as isize && reg + 2 >= cycle as isize {
            crt.push('#');
        } else {
            crt.push('.');
        }

        if wait == 0 {
            if let Some(line) = file_iterator.next() {
                if let Ok(line) = line {
                    let line: Vec<&str> = line.trim_end().split(' ').collect();

                    match line[0] {
                        "noop" => {}
                        "addx" => {
                            add_to_reg = line[1].parse::<isize>().unwrap();

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

        if cycle == 40 {
            println!("{:?}", &crt[0..39]);
            cycle = 0;
            crt = vec![];
        }
    }

    Ok(())
}
