use shared::file_reader::FileReader;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut result = 0;

    for line in FileReader::open("./input", 20)? {
        if let Ok(line) = line {
            let values: Vec<i32> = line
                .trim_end()
                .split(&['-', ','][..])
                .map(|x| x.parse().unwrap())
                .collect();

            if eval(((values[0], values[1]), (values[2], values[3]))) {
                result += 1;
            }
        }
    }
    println!("{}", result);

    Ok(())
}

pub fn eval(values: ((i32, i32), (i32, i32))) -> bool {
    cmpr(values.0, values.1) || cmpr(values.1, values.0)
}

pub fn cmpr(v1: (i32, i32), v2: (i32, i32)) -> bool {
    (v1.0 >= v2.0 && v1.0 <= v2.1) || (v1.1 >= v2.0 && v1.1 <= v2.1)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let data = vec![
            ((2, 4), (6, 8), false),
            ((2, 3), (4, 5), false),
            ((5, 7), (7, 9), true),
            ((2, 8), (3, 7), true),
            ((5, 8), (4, 6), true),
            ((2, 6), (4, 8), true),
        ];

        for value in data {
            assert_eq!(eval((value.0, value.1)), value.2);
        }
    }
}
