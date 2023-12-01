use std::{fs, path::PathBuf};

pub fn read_puzzle_input(day: u8, test: bool) -> Vec<String> {
    let mut path = PathBuf::new();
    path = path.join(format!("./puzzle-input/day{}/", day));

    if test {
        path = path.join("input.test");
    } else {
        path = path.join("input");
    }

    println!("{:?}", path);

    let content = fs::read_to_string(path)
        .unwrap_or(String::from(""))
        .to_string();

    content
        .trim()
        .split("\n")
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
}
