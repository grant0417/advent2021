// https://adventofcode.com/2021/day/1

pub fn part1() -> String {
    let input = include_str!("../../data/day1.txt");
    let values = input.lines().map(|x| x.parse::<i32>().unwrap());

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

pub fn part2() -> String {
    let input = include_str!("../../data/day1.txt");
    let values = input.lines().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let window = values.iter()
        .zip(values.iter().skip(1))
        .zip(values.iter().skip(2))
        .map(|((a, b), c)| a + b + c);

    let mut increments = 0;
    let mut previous_value = None;

    for value in window {
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
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "1466");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "1491");
    }
}
