// https://adventofcode.com/2021/day/4

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BingoBoard {
    board: [u32; 25],
    checked: [bool; 25],
}

impl BingoBoard {
    fn from_string(input: &str) -> BingoBoard {
        let mut board = [0; 25];
        let checked = [false; 25];

        let mut i = 0;
        for line in input.lines() {
            for num in line.split_whitespace() {
                board[i] = num.parse().unwrap();
                i += 1;
            }
        }
        BingoBoard { board, checked }
    }

    fn check(&mut self, num: u32) {
        for i in 0..25 {
            if self.board[i] == num {
                self.checked[i] = true;
            }
        }
    }

    fn check_row(&self, row: usize) -> bool {
        for i in 0..5 {
            if !self.checked[row * 5 + i] {
                return false;
            }
        }
        true
    }

    fn check_col(&self, col: usize) -> bool {
        for i in 0..5 {
            if !self.checked[col + i * 5] {
                return false;
            }
        }
        true
    }

    fn check_bingo(&self) -> bool {
        for i in 0..5 {
            if self.check_row(i) {
                return true;
            }
            if self.check_col(i) {
                return true;
            }
        }
        false
    }

    fn sum_unchecked(&self) -> u32 {
        let mut sum = 0;
        for i in 0..25 {
            if !self.checked[i] {
                sum += self.board[i];
            }
        }
        sum
    }
}

pub fn part1(input: impl AsRef<str>) -> String {
    let draw_numbers = input
        .as_ref()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();

    let boards_input = input.as_ref().lines().skip(2).collect::<Vec<&str>>();
    let mut boards: Vec<_> = boards_input
        .chunks(6)
        .into_iter()
        .map(|chunck| {
            let b_str = chunck.join("\n");
            BingoBoard::from_string(&b_str)
        })
        .collect();

    for num in draw_numbers {
        // Mark boards
        for board in &mut boards {
            board.check(num);
        }

        // Check if any board is bingo
        for board in &boards {
            if board.check_bingo() {
                return (board.sum_unchecked() * num).to_string();
            }
        }
    }

    return String::from("No winning board found");
}

pub fn part2(input: impl AsRef<str>) -> String {
    let draw_numbers = input
        .as_ref()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();

    let boards_input = input.as_ref().lines().skip(2).collect::<Vec<&str>>();
    let mut boards: Vec<_> = boards_input
        .chunks(6)
        .into_iter()
        .map(|chunck| {
            let b_str = chunck.join("\n");
            BingoBoard::from_string(&b_str)
        })
        .collect();

    for num in draw_numbers {
        // Mark boards
        for board in &mut boards {
            board.check(num);
        }

        // Check if a single board remains
        if boards.len() == 1 && boards[0].check_bingo() {
            return (boards[0].sum_unchecked() * num).to_string();
        }

        // Remove board if it is bingo
        boards.retain(|board| !board.check_bingo());
    }

    return String::from("No winning board found");
}

#[cfg(test)]
mod tests {
    use crate::{get_input, get_test};

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_test(4)), "4512");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_test(4)), "1924");
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(part1(get_input(4)), "4662");
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2(get_input(4)), "12080");
    }
}
