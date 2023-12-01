pub fn part1(input: &Vec<String>) -> i32 {
    input
        .iter()
        .map(|line| {
            let digits = find_digits(line);

            let a = digits.first().unwrap();
            let b = digits.last().unwrap();

            (a * 10 + b) as i32
        })
        .sum()
}

pub fn part2(input: &Vec<String>) -> i32 {
    input
        .iter()
        .map(|line| {
            let digits = find_string_digits(line);

            let a = digits.first().unwrap();
            let b = digits.last().unwrap();

            (a * 10 + b) as i32
        })
        .sum()
}

fn find_digits(input: &str) -> Vec<u8> {
    let mut digits: Vec<u8> = vec![];

    for ch in input.chars() {
        let ascii_ch = ch as u8;

        if ascii_ch > 47 && ascii_ch < 58 {
            digits.push(ascii_ch - 48)
        }
    }

    digits
}

fn find_string_digits(input: &str) -> Vec<u8> {
    let mut digits: Vec<u8> = vec![];
    let mut buffer = String::new();
    let lookup = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for ch in input.chars() {
        buffer.push(ch.to_ascii_lowercase());
        let ascii_ch = ch as u8;

        if ascii_ch > 47 && ascii_ch < 58 {
            digits.push(ascii_ch - 48)
        }

        for n in 0..lookup.len() {
            if buffer.ends_with(lookup[n]) {
                digits.push((n + 1) as u8);
                break;
            }
        }
    }

    digits
}

pub fn run(input: Vec<String>) {
    println!("part 1: ( {} )", part1(&input));
    println!("part 2: ( {} )", part2(&input));
}
