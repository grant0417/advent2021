// https://adventofcode.com/2021/day/8

use std::{collections::VecDeque, ops};

fn char_to_index(c: char) -> usize {
    c as usize - 'a' as usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SevenSegment(u8);

impl SevenSegment {
    fn new(input: &str) -> SevenSegment {
        let mut value: u8 = 0;

        for c in input.chars() {
            value |= 1 << char_to_index(c);
        }

        SevenSegment(value)
    }

    fn count_lit(&self) -> u32 {
        self.0.count_ones()
    }
}

impl ops::BitAnd for SevenSegment {
    type Output = SevenSegment;

    fn bitand(self, rhs: Self) -> Self::Output {
        SevenSegment(self.0 & rhs.0)
    }
}

impl ops::Not for SevenSegment {
    type Output = SevenSegment;

    fn not(self) -> Self::Output {
        SevenSegment(!self.0)
    }
}

fn parse_input(
    input: impl AsRef<str>,
) -> VecDeque<(VecDeque<SevenSegment>, VecDeque<SevenSegment>)> {
    let mut lines = VecDeque::new();
    for line in input.as_ref().lines() {
        let mut split = line
            .split('|')
            .map(|s| s.trim().split(' ').map(SevenSegment::new).collect());

        let left = split.next().unwrap();
        let right = split.next().unwrap();

        lines.push_back((left, right));
    }

    lines
}

pub fn part1(input: impl AsRef<str>) -> String {
    let parsed_input = parse_input(input);

    let sum: usize = parsed_input
        .iter()
        .map(|(_, out)| {
            out.iter()
                .filter(|a| [2, 3, 4, 7].contains(&a.count_lit()))
                .count()
        })
        .sum();

    sum.to_string()
}

pub fn part2(input: impl AsRef<str>) -> String {
    let parsed_input = parse_input(input);

    let mut sum = 0;

    for (mut in_segments, out_segments) in parsed_input {
        let mut seven_segments: [Option<SevenSegment>; 10] = [None; 10];

        while let Some(segment) = in_segments.pop_front() {
            let segment_count = segment.count_lit();

            let one = seven_segments[1].map(|s| (s & segment).count_lit());
            let four = seven_segments[4].map(|s| (s & segment).count_lit());

            match (segment_count, one, four) {
                (2, _, _) => seven_segments[1] = Some(segment),
                (3, _, _) => seven_segments[7] = Some(segment),
                (4, _, _) => seven_segments[4] = Some(segment),
                (7, _, _) => seven_segments[8] = Some(segment),
                (6, Some(2), Some(4)) => seven_segments[9] = Some(segment),
                (6, Some(2), Some(3)) => seven_segments[0] = Some(segment),
                (6, Some(1), Some(3)) => seven_segments[6] = Some(segment),
                (5, Some(2), Some(3)) => seven_segments[3] = Some(segment),
                (5, Some(1), Some(3)) => seven_segments[5] = Some(segment),
                (5, Some(1), Some(2)) => seven_segments[2] = Some(segment),
                (_, _, _) => in_segments.push_back(segment),
            }
        }

        let mut number = 0;
        for segment in &out_segments {
            for i in 0..10 {
                if let Some(inner_segment) = seven_segments[i] {
                    if *segment == inner_segment {
                        number = number * 10 + i;
                    }
                }
            }
        }

        sum += number;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_real, get_test};
    use test::{black_box, Bencher};

    const DAY_NUM: u32 = 8;

    const TEST_PART_1_RESULT: &'static str = "26";
    const TEST_PART_2_RESULT: &'static str = "61229";
    const REAL_PART_1_RESULT: &'static str = "440";
    const REAL_PART_2_RESULT: &'static str = "1046281";

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
