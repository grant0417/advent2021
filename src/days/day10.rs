// https://adventofcode.com/2021/day/10

pub fn part1(input: impl AsRef<str>) -> String {
    input
        .as_ref()
        .lines()
        .map(|line| {
            let mut stack = vec![];

            line.chars()
                .map(|c| match c {
                    '(' | '[' | '{' | '<' => {
                        stack.push(c);
                        0
                    }
                    ')' | ']' | '}' | '>' => {
                        let left = stack.pop().unwrap();
                        match (left, c) {
                            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => 0,
                            (_, ')') => 3,
                            (_, ']') => 57,
                            (_, '}') => 1197,
                            (_, '>') => 25137,
                            (_, _) => panic!("Unexpected character"),
                        }
                    }
                    _ => panic!("Unexpected character"),
                })
                .sum::<u64>()
        })
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: impl AsRef<str>) -> String {
    let lines: Vec<_> = input.as_ref().lines().collect();

    let mut line_score: Vec<u64> = vec![];

    'line_loop: for line in lines {
        let mut stack = vec![];
  
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let top = stack.pop().unwrap();

                    match (top, c) {
                        ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => continue,
                        (_, ')') | (_, ']') | (_, '}') | (_, '>') => {
                            continue 'line_loop;
                        }
                        (_, _) => panic!("Unexpected character")
                    }
                }
                _ => panic!("Unexpected character")
            }
        }

        line_score.push(stack.iter().rev().fold(0, |mut acc, c| {
            acc *= 5;
            match c {
                '(' => acc += 1,
                '[' => acc += 2,
                '{' => acc += 3,
                '<' => acc += 4,
                _ => panic!("Unexpected character"),
            }
            acc
        }))
    }

    line_score.sort_unstable();

    line_score[line_score.len() / 2].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_real, get_test};
    use test::{black_box, Bencher};

    const DAY_NUM: u32 = 10;

    const TEST_PART_1_RESULT: &'static str = "26397";
    const TEST_PART_2_RESULT: &'static str = "288957";
    const REAL_PART_1_RESULT: &'static str = "240123";
    const REAL_PART_2_RESULT: &'static str = "3260812321";

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
