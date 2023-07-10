use std::{collections::HashMap, hash::Hash};

// https://adventofcode.com/2021/day/14

fn parse_input(input: impl AsRef<str>) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut split = input.as_ref().split("\n\n");

    let starting_poly = split.next().unwrap().chars().collect();

    let mut chem_map = HashMap::new();

    for mapping in split.next().unwrap().lines() {
        let mut mapping_split = mapping.split(" -> ");

        let mut from_chems = mapping_split.next().unwrap().chars();
        let from_chem_a = from_chems.next().unwrap();
        let from_chem_b = from_chems.next().unwrap();

        let to_chem = mapping_split.next().unwrap().chars().next().unwrap();

        chem_map.insert((from_chem_a, from_chem_b), to_chem);
    }

    (starting_poly, chem_map)
}

pub fn part1(input: impl AsRef<str>) -> String {
    let (starting_poly, chem_map) = parse_input(input);

    let mut current_poly = starting_poly;

    for i in 0..40 {
        println!("{}", i);
        let mut next_poly = Vec::new();

        for c in current_poly.windows(2) {
            next_poly.push(c[0]);
            next_poly.push(chem_map[&(c[0], c[1])]);
        }

        next_poly.push(*current_poly.last().unwrap());

        current_poly = next_poly;
    }

    let mut quantities = HashMap::new();

    for c in current_poly {
        match quantities.get_mut(&c) {
            Some(v) => *v += 1,
            None => {
                quantities.insert(c, 1);
            }
        }
    }

    let mut sorted_quantites = quantities.iter().collect::<Vec<_>>();
    sorted_quantites.sort_by(|(_, a), (_, b)| b.cmp(a));

    let val = sorted_quantites.first().unwrap().1 - sorted_quantites.last().unwrap().1;

    val.to_string()
}

pub fn part2(input: impl AsRef<str>) -> String {
    let parsed_input = parse_input(input);

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_real, get_test};
    use test::{black_box, Bencher};

    const DAY_NUM: u32 = 14;

    const TEST_PART_1_RESULT: &'static str = "1588";
    const TEST_PART_2_RESULT: &'static str = "";
    const REAL_PART_1_RESULT: &'static str = "2003";
    const REAL_PART_2_RESULT: &'static str = "";

    #[test]
    fn test_part1() {
        let result = part1(get_test(DAY_NUM));
        assert_eq!(result, TEST_PART_1_RESULT);
        println!("Day {} Part 1 test result: {}", DAY_NUM, result);
    }

    #[test]
    fn test_part2() {
        let result = part2(get_test(DAY_NUM));
        assert_eq!(result, TEST_PART_2_RESULT);
        println!("Day {} Part 2 test result: {}", DAY_NUM, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(get_real(DAY_NUM));
        assert_eq!(result, REAL_PART_1_RESULT);
        println!("Day {} Part 1 real result: {}", DAY_NUM, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(get_real(DAY_NUM));
        assert_eq!(result, REAL_PART_2_RESULT);
        println!("Day {} Part 2 real result: {}", DAY_NUM, result);
    }

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        let input = get_real(DAY_NUM);
        b.iter(move || {
            black_box(part1(input.clone()));
        });
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        let input = get_real(DAY_NUM);
        b.iter(move || {
            black_box(part2(input.clone()));
        });
    }
}
