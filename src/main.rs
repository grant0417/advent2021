use std::{env::args, io::Read};

use crate::days::{day1, day2, day3, day4};

mod days;
mod util;

/// Load day input from input folder
pub fn get_input(day: u32) -> String {
    let mut file = std::fs::File::open(format!("data/real/day{}.txt", day)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

/// Load day test from test folder
pub fn get_test(day: u32) -> String {
    let mut file = std::fs::File::open(format!("data/test/day{}.txt", day)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}


fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("Please provide a day number");
        return;
    }

    let result = match args[1].as_str() {
        "1.1" => day1::part1(get_input(1)),
        "1.2" => day1::part2(get_input(1)),
        "2.1" => day2::part1(get_input(2)),
        "2.2" => day2::part2(get_input(2)),
        "3.1" => day3::part1(get_input(3)),
        "3.2" => day3::part2(get_input(3)),
        "4.1" => day4::part1(get_input(4)),
        "4.2" => day4::part2(get_input(4)),
        _ => format!("Day {} not implemented", args[1]),
    };

    println!("{}", result);
}
