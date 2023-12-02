pub fn part1(input: &Vec<String>) -> u16 {
    let red_count = 12;
    let green_count = 13;
    let blue_count = 14;

    input
        .iter()
        .map(|line| {
            let game = get_sets_from_line(line);

            for group in game.groups.iter() {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;

                group.iter().for_each(|col| match col.color {
                    Colors::Red => red += col.number,
                    Colors::Green => green += col.number,
                    Colors::Blue => blue += col.number,
                });

                if red > red_count || green > green_count || blue > blue_count {
                    return 0;
                }
            }

            return game.id as u16;
        })
        .sum()
}

pub fn part2(input: &Vec<String>) -> u32 {
    input
        .iter()
        .map(|line| {
            let game = get_sets_from_line(line);

            let colors = game
                .groups
                .into_iter()
                .flatten()
                .collect::<Vec<ColorCount>>();

            let red = colors
                .iter()
                .filter(|col| col.color == Colors::Red)
                .max_by_key(|col| col.number)
                .unwrap()
                .number as usize;
            let green = colors
                .iter()
                .filter(|col| col.color == Colors::Green)
                .max_by_key(|col| col.number)
                .unwrap()
                .number as usize;
            let blue = colors
                .iter()
                .filter(|col| col.color == Colors::Blue)
                .max_by_key(|col| col.number)
                .unwrap()
                .number as usize;

            (red * green * blue) as u32
        })
        .sum()
}

#[derive(Debug, PartialEq)]
enum Colors {
    Red,
    Blue,
    Green,
}

#[derive(Debug)]
struct ColorCount {
    color: Colors,
    number: u8,
}

#[derive(Debug)]
struct Game {
    id: u8,
    groups: Vec<Vec<ColorCount>>,
}

fn get_sets_from_line(line: &str) -> Game {
    let split: Vec<&str> = line.split(":").collect();
    let label_str = split.first().unwrap();
    let game_id = label_str.replace("Game ", "").parse::<u8>().unwrap();

    let groups_str = split.last().unwrap().trim();
    let groups_vec = groups_str.split(";").collect::<Vec<&str>>();
    let groups: Vec<Vec<ColorCount>> = groups_vec
        .iter()
        .map(|group| {
            group
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|color| {
                    let split: Vec<&str> = color.trim().split_whitespace().collect();
                    let number = split.first().unwrap().parse::<u8>().unwrap();
                    let col = split.last().unwrap().to_owned();

                    ColorCount {
                        color: match col {
                            "red" => Colors::Red,
                            "blue" => Colors::Blue,
                            "green" => Colors::Green,
                            _ => panic!(),
                        },
                        number,
                    }
                })
                .collect()
        })
        .collect();

    Game {
        id: game_id,
        groups,
    }
}

pub fn run(input: Vec<String>) {
    println!("part 1: ( {} )", part1(&input));
    println!("part 2: ( {} )", part2(&input));
}
