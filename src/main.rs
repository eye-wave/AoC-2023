mod days;
mod utils;

fn main() {
    let day = utils::env::get_day_from_env().unwrap_or(0);
    if day == 0 {
        println!("Please provide a day number 1-25.\nexample: (day2)");
        return;
    }

    let test = utils::env::get_test_from_env();
    let content = utils::reader::read_puzzle_input(day, test);

    println!("running day {}...", day);

    match day {
        1 => days::day1::run(content),
        _ => println!("day {} not implemented yet.", day),
    }
}
