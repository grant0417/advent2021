// https://adventofcode.com/2021/day/1

pub fn part1(input: impl AsRef<str>) -> String {
    let values = input.as_ref().lines().map(|x| x.parse::<i32>().unwrap());

    let mut increments = 0;
    let mut previous_value = None;

    for value in values {
        if let Some(v) = previous_value {
            if value > v {
                increments += 1;
            }
        }

        previous_value = Some(value);
    }

    increments.to_string()
}

pub fn part2(input: impl AsRef<str>) -> String {
    let values = input
        .as_ref()
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let windows = values.windows(3).map(|window| window.iter().sum::<i32>());

    let mut increments = 0;
    let mut previous_value = None;

    for value in windows {
        if let Some(v) = previous_value {
            if value > v {
                increments += 1;
            }
        }

        previous_value = Some(value);
    }

    increments.to_string()
}

#[cfg(test)]
mod tests {
    use crate::{get_real, get_test};
    use test::{black_box, Bencher};

    const DAY_NUM: u32 = 1;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_test(1)), "7");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_test(1)), "5");
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(part1(get_real(1)), "1466");
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2(get_real(1)), "1491");
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
