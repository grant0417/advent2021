// https://adventofcode.com/2021/day/13

#[derive(Debug, Clone, PartialEq, Eq)]
enum FoldAxis {
    X,
    Y,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FoldCommand {
    coord: i32,
    axis: FoldAxis,
}

struct FoldablePaper {
    points: Vec<bool>,
    width: i32,
    height: i32,
}

impl FoldablePaper {
    fn new(width: i32, height: i32) -> FoldablePaper {
        FoldablePaper {
            points: vec![false; (width * height).try_into().unwrap()],
            width,
            height,
        }
    }

    fn mark_point(&mut self, x: i32, y: i32) {
        let index: usize = (self.width * y + x).try_into().unwrap();
        self.points[index] = true;
    }

    fn is_marked(&self, x: i32, y: i32) -> bool {
        let index: usize = (self.width * y + x).try_into().unwrap();
        *self.points.get(index).unwrap()
    }

    fn count_marked(&self) -> i32 {
        self.points
            .iter()
            .filter(|v| **v)
            .count()
            .try_into()
            .unwrap()
    }

    fn fold(&mut self, fold: &FoldCommand) {
        let (new_width, new_height) = match fold.axis {
            FoldAxis::X => (self.width / 2, self.height),
            FoldAxis::Y => (self.width, self.height / 2),
        };

        let mut new_points = vec![false; (new_height * new_width) as usize];

        match fold.axis {
            FoldAxis::X => {
                for y in 0..self.height {
                    for x in 0..new_width {
                        if self.is_marked(x, y) {
                            new_points[(new_width * y + x) as usize] = true;
                        }
                        if self.is_marked(self.width - 1 - x, y) {
                            new_points[(new_width * y + x) as usize] = true;
                        }
                    }
                }
            }
            FoldAxis::Y => {
                for y in 0..new_height {
                    for x in 0..self.width {
                        if self.is_marked(x, y) {
                            new_points[(new_width * y + x) as usize] = true;
                        }
                        if self.is_marked(x, self.height - 1 - y) {
                            new_points[(new_width * y + x) as usize] = true;
                        }
                    }
                }
            }
        }

        self.width = new_width;
        self.height = new_height;
        self.points = new_points;
    }
}

fn parse_input(input: impl AsRef<str>) -> (FoldablePaper, Vec<FoldCommand>) {
    let mut split = input.as_ref().split("\n\n");

    let mut max_x = 0;
    let mut max_y = 0;

    let coords: Vec<_> = split
        .next()
        .unwrap()
        .lines()
        .map(|s| {
            let mut line_split = s.split(',');
            let x: i32 = line_split.next().unwrap().parse().unwrap();
            let y: i32 = line_split.next().unwrap().parse().unwrap();

            max_x = max_x.max(x);
            max_y = max_y.max(y);

            (x, y)
        })
        .collect();

    let mut foldable_paper = FoldablePaper::new(max_x + 1, max_y + 1);

    for (x, y) in &coords {
        foldable_paper.mark_point(*x, *y);
    }

    let commands: Vec<_> = split
        .next()
        .unwrap()
        .lines()
        .map(|s| {
            let mut split = s.strip_prefix("fold along ").unwrap().split('=');

            let axis = match split.next().unwrap() {
                "x" => FoldAxis::X,
                "y" => FoldAxis::Y,
                _ => panic!("Unexpected fold axis"),
            };

            let coord = split.next().unwrap().parse().unwrap();

            FoldCommand { coord, axis }
        })
        .collect();

    (foldable_paper, commands)
}

pub fn part1(input: impl AsRef<str>) -> String {
    let (mut foldable_paper, fold_commands) = parse_input(input);

    let cmd = &fold_commands[0];
    foldable_paper.fold(cmd);

    foldable_paper.count_marked().to_string()
}

pub fn part2(input: impl AsRef<str>) -> String {
    let (mut foldable_paper, fold_commands) = parse_input(input);

    for cmd in fold_commands {
        foldable_paper.fold(&cmd);
    }

    let mut s = String::new();

    for y in 0..foldable_paper.height {
        for x in 0..foldable_paper.width {
            if foldable_paper.is_marked(x, y) {
                s.push('#');
            } else {
                s.push(' ');
            }
        }
        s.push('\n');
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_real, get_test};
    use test::{black_box, Bencher};

    const DAY_NUM: u32 = 13;

    const TEST_PART_1_RESULT: &'static str = "17";
    const TEST_PART_2_RESULT: &'static str = "#####
#   #
#   #
#   #
#####
     
     
";
    const REAL_PART_1_RESULT: &'static str = "664";
    const REAL_PART_2_RESULT: &'static str = "#### ####   ## #  # #### #    ###  #    
#    #       # # #     # #    #  # #    
###  ###     # ##     #  #    ###  #    
#    #       # # #   #   #    #  # #    
#    #    #  # # #  #    #    #  # #    
#### #     ##  #  # #### #### ###  #### 
";

    #[test]
    fn test_part1_test() {
        let result = part1(get_test(DAY_NUM));
        assert_eq!(result, TEST_PART_1_RESULT);
        println!("Day {} Part 1 test result: {}", DAY_NUM, result);
    }

    #[test]
    fn test_part2() {
        let result = part2(get_test(DAY_NUM));
        assert_eq!(result, TEST_PART_2_RESULT);
        println!("Day {} Part 2 test result:\n{}", DAY_NUM, result);
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
        println!("Day {} Part 2 real result:\n{}", DAY_NUM, result);
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
