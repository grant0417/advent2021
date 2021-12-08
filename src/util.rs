use std::{io::Read, path::Path};

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
