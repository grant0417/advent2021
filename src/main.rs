use std::env::args;

use crate::days::{day1, day2, day3};

mod days;
mod util;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("Please provide a day number");
        return;
    }

    let result = match args[1].as_str() {
        "1.1" => day1::part1(),
        "1.2" => day1::part2(),
        "2.1" => day2::part1(),
        "2.2" => day2::part2(),
        "3.1" => day3::part1(),
        "3.2" => day3::part2(),
        _ => format!("Day {} not implemented", args[1]),
    };

    println!("{}", result);
}
