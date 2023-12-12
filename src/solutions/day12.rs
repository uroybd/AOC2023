// Advent of Code 2023 - Day 12
use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
struct DamageReport {
    map: String,
    report: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseDamageReportError;

impl std::str::FromStr for DamageReport {
    type Err = ParseDamageReportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map, report) = s.split_once(' ').unwrap();
        Ok(DamageReport {
            map: map.to_string(),
            report: report.split(',').map(|x| x.parse().unwrap()).collect(),
        })
    }
}

fn create_key(inp: &str, report: &[usize]) -> String {
    format!(
        "{}-{}",
        inp,
        report
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("/")
    )
}

fn find_combinations(inp: &str, report: &[usize], cache: &mut HashMap<String, usize>) -> usize {
    let key = create_key(inp, report);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }
    let result = match inp.chars().next() {
        Some('.') => find_combinations(inp.strip_prefix('.').unwrap(), report, cache),
        Some('?') => {
            find_combinations(&inp.replacen('?', ".", 1), report, cache)
                + find_combinations(&inp.replacen('?', "#", 1), report, cache)
        }
        Some('#') => {
            if report.is_empty() || inp.len() < report[0] || inp[0..report[0]].contains('.') {
                0
            } else if report.len() > 1 {
                if (inp.len() < report[0] + 1) || (inp.chars().nth(report[0]).unwrap() == '#') {
                    0
                } else {
                    find_combinations(&inp[report[0] + 1..], &report[1..], cache)
                }
            } else {
                find_combinations(&inp[report[0]..], &report[1..], cache)
            }
        }
        None => {
            if report.is_empty() {
                1
            } else {
                0
            }
        }
        _ => unreachable!(),
    };
    cache.insert(key.clone(), result);
    result
}

pub fn solution_day_12_01(file_path: String) -> Option<usize> {
    let data: Vec<DamageReport> = fs::read_to_string(file_path)
        .expect("Invalid File")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut cache = HashMap::new();
    Some(
        data.iter()
            .map(|x| find_combinations(&x.map, &x.report, &mut cache))
            .sum::<usize>(),
    )
    // None
}

pub fn solution_day_12_02(file_path: String) -> Option<usize> {
    let data: Vec<DamageReport> = fs::read_to_string(file_path)
        .expect("Invalid File")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut cache = HashMap::new();
    Some(
        data.iter()
            .map(|x| {
                let map = [x.map.as_str(); 5].join("?");
                let report = x.report.repeat(5);
                find_combinations(&map, &report, &mut cache)
            })
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_12_01() {
        let file_path: String = String::from("src/inputs/day12e.txt");
        let result = solution_day_12_01(file_path).unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn test_day_12_02() {
        let file_path: String = String::from("src/inputs/day12e.txt");
        let result = solution_day_12_02(file_path).unwrap();
        assert_eq!(result, 525152);
    }

    #[test]
    #[ignore]
    fn output_day_12_01() {
        let file_path: String = String::from("src/inputs/day12.txt");
        let result = solution_day_12_01(file_path).unwrap();
        assert_eq!(result, 7753);
    }

    #[test]
    #[ignore]
    fn output_day_12_02() {
        let file_path: String = String::from("src/inputs/day12.txt");
        let result = solution_day_12_02(file_path).unwrap();
        assert_eq!(result, 280382734828319);
    }
}
