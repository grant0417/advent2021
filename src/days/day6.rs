// https://adventofcode.com/2021/day/6

fn parse_input(input: impl AsRef<str>) -> Vec<usize> {
    input
        .as_ref()
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn comput_fish_count(input: impl AsRef<str>, day: usize) -> usize {
    let parsed_input = parse_input(input);
    let mut fish_cohorts = [0usize; 9];

    for i in parsed_input {
        fish_cohorts[i as usize] += 1;
    }

    for _ in 0..day {
        fish_cohorts.rotate_left(1);
        fish_cohorts[6] += fish_cohorts[8];
    }

    fish_cohorts.iter().sum()
}

pub fn part1(input: impl AsRef<str>) -> String {
    comput_fish_count(input, 80).to_string()
}

pub fn part2(input: impl AsRef<str>) -> String {
    comput_fish_count(input, 256).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_real, get_test};
    use test::{black_box, Bencher};

    const DAY_NUM: u32 = 6;

    const TEST_PART_1_RESULT: &'static str = "5934";
    const TEST_PART_2_RESULT: &'static str = "26984457539";
    const REAL_PART_1_RESULT: &'static str = "379414";
    const REAL_PART_2_RESULT: &'static str = "1705008653296";

    #[test]
    fn test_part1() {
        let result = part1(get_test(DAY_NUM));
        println!("Part 1 test result: {}", result);
        assert_eq!(result, TEST_PART_1_RESULT);
    }

    #[test]
    fn test_part2() {
        let result = part2(get_test(DAY_NUM));
        println!("Part 2 test result: {}", result);
        assert_eq!(result, TEST_PART_2_RESULT);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(get_real(DAY_NUM));
        println!("Part 1 real result: {}", result);
        assert_eq!(result, REAL_PART_1_RESULT);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(get_real(DAY_NUM));
        println!("Part 2 real result: {}", result);
        assert_eq!(result, REAL_PART_2_RESULT);
    }

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        let input = get_real(DAY_NUM);
        b.iter(|| {
            black_box(part1(input.clone()));
        });
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        let input = get_real(DAY_NUM);
        b.iter(|| {
            black_box(part2(input.clone()));
        });
    }
}
