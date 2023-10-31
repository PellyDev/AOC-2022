use std::fs;

pub fn new_line_parser(day: u8) -> Vec<String> {
    let path: String = format!("inputs/input{}.txt", day);
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| String::from(line))
        .collect::<Vec<String>>()
}
