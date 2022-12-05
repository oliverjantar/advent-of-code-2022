use std::error::Error;

use shared::file_reader::FileReader;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut state = vec![
        Vec::from(['F', 'C', 'P', 'G', 'Q', 'R']),
        Vec::from(['W', 'T', 'C', 'P']),
        Vec::from(['B', 'H', 'P', 'M', 'C']),
        Vec::from(['L', 'T', 'Q', 'S', 'M', 'P', 'R']),
        Vec::from(['P', 'H', 'J', 'Z', 'V', 'G', 'N']),
        Vec::from(['D', 'P', 'J']),
        Vec::from(['L', 'G', 'P', 'Z', 'F', 'J', 'T', 'R']),
        Vec::from(['N', 'L', 'H', 'C', 'F', 'P', 'T', 'J']),
        Vec::from(['G', 'V', 'Z', 'Q', 'H', 'T', 'C', 'W']),
    ];
    for line in FileReader::open("./input", 20)? {
        if let Ok(line) = line {
            let cmd = Command::new(&line.trim());
            state = apply(state, cmd);
        }
    }

    println!("{}", get_result(&state));

    Ok(())
}

fn apply(mut state: Vec<Vec<char>>, command: Command) -> Vec<Vec<char>> {
    for _ in 0..command.0 {
        let item = state[command.1 - 1].pop().unwrap();
        state[command.2 - 1].push(item);
    }

    state
}

fn get_result(state: &Vec<Vec<char>>) -> String {
    let mut result = "".to_string();
    for column in state {
        match column.last() {
            Some(value) => result.push(*value),
            None => {}
        }
    }
    result
}

struct Command(i32, usize, usize);

impl Command {
    fn new(value: &str) -> Self {
        let x: Vec<i32> = value.split(' ').map(|x| x.parse().unwrap()).collect();
        Self(x[0], x[1] as usize, x[2] as usize)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let state: Vec<Vec<char>> = vec![
            Vec::from(['Z', 'N']),
            Vec::from(['M', 'C', 'D']),
            Vec::from(['P']),
        ];

        let command = Command(1, 2, 1);

        let expected: Vec<Vec<char>> = vec![
            Vec::from(['Z', 'N', 'D']),
            Vec::from(['M', 'C']),
            Vec::from(['P']),
        ];
        let state = apply(state, command);

        assert_eq!(state, expected);

        let command = Command(3, 1, 3);

        let expected: Vec<Vec<char>> = vec![
            Vec::from([]),
            Vec::from(['M', 'C']),
            Vec::from(['P', 'D', 'N', 'Z']),
        ];
        let state = apply(state, command);

        assert_eq!(state, expected);

        let command = Command(2, 2, 1);

        let expected: Vec<Vec<char>> = vec![
            Vec::from(['C', 'M']),
            Vec::from([]),
            Vec::from(['P', 'D', 'N', 'Z']),
        ];
        let state = apply(state, command);

        assert_eq!(state, expected);

        let command = Command(1, 1, 2);

        let expected: Vec<Vec<char>> = vec![
            Vec::from(['C']),
            Vec::from(['M']),
            Vec::from(['P', 'D', 'N', 'Z']),
        ];
        let state = apply(state, command);

        assert_eq!(state, expected);

        assert_eq!(get_result(&state), "CMZ".to_string());
    }
}
