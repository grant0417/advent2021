// https://adventofcode.com/2021/day/3

pub fn part1(input: impl AsRef<str>) -> String {
    let bit_count = input.as_ref().lines().next().unwrap().len();

    let mut zero_count = vec![0; bit_count];
    let mut one_count = vec![0; bit_count];

    for line in input.as_ref().lines() {
        for (i, bit) in line.chars().enumerate() {
            let bit = bit.to_digit(10).unwrap();
            if bit == 0 {
                zero_count[i] += 1;
            } else {
                one_count[i] += 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..bit_count {
        if one_count[bit_count - i - 1] > zero_count[bit_count - i - 1] {
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }

    format!("{}", gamma * epsilon)
}

pub fn part2(input: impl AsRef<str>) -> String {
    let bit_count = input.as_ref().lines().next().unwrap().len();

    // Find oxygen generator rating
    let mut oxygen_ratings = input
        .as_ref()
        .lines()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    for i in 0..bit_count {
        let mut zero_count = 0;
        let mut one_count = 0;

        for line in &oxygen_ratings {
            if line.chars().nth(i).unwrap() == '0' {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }

        let most_common = if zero_count > one_count { '0' } else { '1' };

        // Delete lines that don't have the most common bit
        oxygen_ratings = oxygen_ratings
            .into_iter()
            .filter(|line| line.chars().nth(i).unwrap() == most_common)
            .collect::<Vec<_>>();
    }

    // Find co2 scrubber rating
    let mut co2_ratings = input
        .as_ref()
        .lines()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    for i in 0..bit_count {
        let mut zero_count = 0;
        let mut one_count = 0;

        for line in &co2_ratings {
            if line.chars().nth(i).unwrap() == '0' {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }

        let least_common = if zero_count > one_count { '1' } else { '0' };

        if co2_ratings.len() == 1 {
            break;
        }

        // Delete lines that don't have the least common bit
        co2_ratings = co2_ratings
            .into_iter()
            .filter(|line| line.chars().nth(i).unwrap() == least_common)
            .collect::<Vec<_>>();
    }

    // Convert to bit string to int
    let mut oxygen_rating = 0;
    let mut co2_rating = 0;

    for (j, bit) in oxygen_ratings[0].chars().enumerate() {
        if bit == '1' {
            oxygen_rating += 1 << (bit_count - j - 1);
        }
    }

    for (j, bit) in co2_ratings[0].chars().enumerate() {
        if bit == '1' {
            co2_rating += 1 << (bit_count - j - 1);
        }
    }

    format!("{}", oxygen_rating * co2_rating)
}

#[cfg(test)]
mod tests {
    use crate::{get_input, get_test};
    use test::{black_box, Bencher};

    const DAY_NUM: u32 = 3;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_test(3)), "198");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_test(3)), "230");
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(part1(get_input(3)), "749376");
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2(get_input(3)), "2372923");
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
