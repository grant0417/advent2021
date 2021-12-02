// https://adventofcode.com/2021/day/2

pub fn part1() -> String {
    let input = include_str!("../../data/day2.txt");

    let mut depth = 0;
    let mut hor_distance = 0;

    for line in input.lines() {
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

pub fn part2() -> String {
    let input = include_str!("../../data/day2.txt");

    let mut aim = 0;
    let mut hor_distance = 0;
    let mut depth = 0;

    for line in input.lines() {
        let mut data = line.split(' ');

        let direction = data.next().unwrap();
        let x = data.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => {
                hor_distance += x;
                depth += x * aim;
            },
            "up" => aim -= x,
            "down" => aim += x,
            _ => panic!("Unknown direction: {}", direction),
        }        
    }

    format!("{}", hor_distance * depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "1815044");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "1739283308");
    }
}
