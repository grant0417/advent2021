use std::vec;

// https://adventofcode.com/2021/day/11

struct OctopusGrid {
    points: Vec<u32>,
    width: i32,
    height: i32,
}

impl OctopusGrid {
    fn step(&mut self) -> usize {
        for point in self.points.iter_mut() {
            *point += 1;
        }

        let mut flashed = vec![false; (self.width * self.height).try_into().unwrap()];

        'outer_loop: loop {
            for y in 0..self.height {
                for x in 0..self.width {
                    let index: usize = (self.height * y + x).try_into().unwrap();

                    if !flashed[index] && self.points[index] > 9 {
                        flashed[index] = true;
                        self.flash_point(x, y);
                        continue 'outer_loop;
                    }
                }
            }

            break;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let index: usize = (self.height * y + x).try_into().unwrap();

                if self.points[index] >= 10 {
                    self.points[index] = 0;
                }
            }
        }

        flashed.into_iter().filter(|b| *b).count()
    }

    fn flash_point(&mut self, x: i32, y: i32) {
        let x_min = 0.max(x - 1);
        let x_max = (self.width - 1).min(x + 1);

        let y_min = 0.max(y - 1);
        let y_max = (self.height - 1).min(y + 1);

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let index: usize = (self.height * y + x).try_into().unwrap();
                self.points[index] += 1;
            }
        }
    }
}

fn parse_input(input: impl AsRef<str>) -> OctopusGrid {
    let lines: Vec<_> = input.as_ref().lines().collect();

    let height = lines.len();
    let width = lines[0].chars().count();

    let points = lines
        .into_iter()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();

    OctopusGrid {
        width: width.try_into().unwrap(),
        height: height.try_into().unwrap(),
        points,
    }
}

pub fn part1(input: impl AsRef<str>) -> String {
    let mut octopus_grid = parse_input(input);

    let flash_count: usize = (0..100).map(|_| octopus_grid.step()).sum();

    flash_count.to_string()
}

pub fn part2(input: impl AsRef<str>) -> String {
    let mut octopus_grid = parse_input(input);

    let mut i = 1;

    loop {
        let flashed = octopus_grid.step();

        if flashed
            == (octopus_grid.width * octopus_grid.height)
                .try_into()
                .unwrap()
        {
            break;
        }

        i += 1;
    }

    i.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_real, get_test};
    use test::{black_box, Bencher};

    const DAY_NUM: u32 = 11;

    const TEST_PART_1_RESULT: &'static str = "1656";
    const TEST_PART_2_RESULT: &'static str = "195";
    const REAL_PART_1_RESULT: &'static str = "1773";
    const REAL_PART_2_RESULT: &'static str = "494";

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
