// https://adventofcode.com/2021/day/12

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
}

impl Cave {
    fn new(name: impl AsRef<str>) -> Cave {
        let is_upper = name.as_ref().chars().all(|c| c.is_uppercase());

        match (is_upper, name.as_ref()) {
            (_, "start") => Cave::Start,
            (_, "end") => Cave::End,
            (true, _) => Cave::Big(name.as_ref().to_string()),
            (false, _) => Cave::Small(name.as_ref().to_string()),
        }
    }
}

#[derive(Debug, Clone)]
struct CaveSystem {
    adjacency_list: HashMap<Cave, Vec<Cave>>,
}

impl CaveSystem {
    fn new() -> CaveSystem {
        CaveSystem {
            adjacency_list: HashMap::new(),
        }
    }

    fn insert_edge(&mut self, n1: &Cave, n2: &Cave) {
        for (a, b) in [(n1, n2), (n2, n1)] {
            match self.adjacency_list.get_mut(a) {
                Some(node) => node.push(b.clone()),
                None => {
                    self.adjacency_list.insert(a.clone(), vec![b.clone()]);
                }
            }
        }
    }
}

fn find_paths(
    current: &Cave,
    cave_system: &CaveSystem,
    visits: &HashMap<&Cave, usize>,
    mut can_double_visit: bool,
) -> Vec<Vec<Cave>> {
    let mut all_paths = vec![];

    let mut visits = visits.clone();
    let current_visits = match visits.get_mut(current) {
        Some(v) => {
            *v += 1;
            *v
        }
        None => {
            visits.insert(current, 1);
            1
        }
    };

    if let Cave::Small(_) = current {
        if current_visits > 1 {
            can_double_visit = false;
        }
    }

    if let Some(caves) = cave_system.adjacency_list.get(current) {
        for cave in caves.iter().filter(|c| *c != &Cave::Start) {
            let can_visit_small = *visits.get(cave).unwrap_or(&0) >= 1 && !can_double_visit;
            match (cave, can_visit_small) {
                (Cave::Small(_), true) => {}
                (Cave::End, _) => all_paths.push(vec![cave.clone(), current.clone()]),
                (Cave::Small(_) | Cave::Big(_), _) => {
                    let mut paths = find_paths(cave, cave_system, &visits, can_double_visit);

                    for path in paths.iter_mut() {
                        path.push(current.clone())
                    }

                    all_paths.extend(paths);
                }
                _ => unreachable!(),
            }
        }
    }

    all_paths
}

fn parse_input(input: impl AsRef<str>) -> CaveSystem {
    let mut graph = CaveSystem::new();

    for line in input.as_ref().lines() {
        let mut split = line.split('-');
        let n1 = Cave::new(split.next().unwrap().to_owned());
        let n2 = Cave::new(split.next().unwrap().to_owned());

        graph.insert_edge(&n1, &n2);
    }

    graph
}

pub fn part1(input: impl AsRef<str>) -> String {
    let cave_system = parse_input(input);

    let paths: Vec<_> = find_paths(&Cave::Start, &cave_system, &HashMap::new(), false);

    paths.len().to_string()
}

pub fn part2(input: impl AsRef<str>) -> String {
    let cave_system = parse_input(input);

    let paths: Vec<_> = find_paths(&Cave::Start, &cave_system, &HashMap::new(), true);

    paths.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_real, get_test};
    use test::{black_box, Bencher};

    const DAY_NUM: u32 = 12;

    const TEST_PART_1_RESULT: &'static str = "226";
    const TEST_PART_2_RESULT: &'static str = "3509";
    const REAL_PART_1_RESULT: &'static str = "3761";
    const REAL_PART_2_RESULT: &'static str = "99138";

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
