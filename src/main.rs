use std::{env::args, io::Read, path::Path};

use crate::days::*;

mod days;
mod util;

fn get_data(file_path: impl AsRef<Path>) -> String {
    let mut file = std::fs::File::open(file_path.as_ref()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

/// Load day input from input folder
pub fn get_input(day: u32) -> String {
    get_data(format!("data/real/day{}.txt", day))
}

/// Load day test from test folder
pub fn get_test(day: u32) -> String {
    get_data(format!("data/test/day{}.txt", day))
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
        "5.1" => day5::part1(get_input(5)),
        "5.2" => day5::part2(get_input(5)),
        _ => format!("Day {} not implemented", args[1]),
    };

    println!("{}", result);
}
