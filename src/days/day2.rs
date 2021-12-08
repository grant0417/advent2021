// https://adventofcode.com/2021/day/2

pub fn part1(input: impl AsRef<str>) -> String {
    let mut depth = 0;
    let mut hor_distance = 0;

    for line in input.as_ref().lines() {
        let mut data = line.split(' ');

        let direction = data.next().unwrap();
        let x = data.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => hor_distance += x,
            "up" => depth -= x,
            "down" => depth += x,
            _ => panic!("Unknown direction: {}", direction),
        }
    }

    format!("{}", hor_distance * depth)
}

pub fn part2(input: impl AsRef<str>) -> String {
    let mut aim = 0;
    let mut hor_distance = 0;
    let mut depth = 0;

    for line in input.as_ref().lines() {
        let mut data = line.split(' ');

        let direction = data.next().unwrap();
        let x = data.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => {
                hor_distance += x;
                depth += x * aim;
            }
            "up" => aim -= x,
            "down" => aim += x,
            _ => panic!("Unknown direction: {}", direction),
        }
    }

    format!("{}", hor_distance * depth)
}

#[cfg(test)]
mod tests {
    use crate::{get_input, get_test};
    use test::{black_box, Bencher};

    const DAY_NUM: u32 = 2;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_test(2)), "150");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_test(2)), "900");
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(part1(get_input(2)), "1815044");
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2(get_input(2)), "1739283308");
    }

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        let input = get_input(DAY_NUM);
        b.iter(move || {
            black_box(part1(input.clone()));
        });
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        let input = get_input(DAY_NUM);
        b.iter(move || {
            black_box(part2(input.clone()));
        });
    }

}
