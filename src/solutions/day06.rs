use std::fs;

// Advent of Code 2023 - Day 06

fn winning_count(stat: &(usize, usize)) -> usize {
    let offset = if let 0 = stat.0 % 2 { 1 } else { 0 };
    (0..=stat.0 / 2).fold(0, |acc, t| acc + ((stat.0 - t) * t > stat.1) as usize) * 2 - offset
}

fn parse_line(l: &str) -> impl Iterator<Item = usize> + '_ {
    l.split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
}

fn parse(data: &str) -> Vec<(usize, usize)> {
    let (times, distances) = data.split_once('\n').unwrap();
    parse_line(times).zip(parse_line(distances)).collect()
}

fn parse_combined(data: &str) -> (usize, usize) {
    let mut parts = data.split('\n').map(|v| {
        v.replace(' ', "")
            .split_once(':')
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap()
    });
    (parts.next().unwrap(), parts.next().unwrap())
}

pub fn solution_day_06_01(file_path: String) -> Option<usize> {
    Some(
        parse(&fs::read_to_string(file_path).expect("Invalid Input."))
            .iter()
            .fold(1, |acc, s| acc * winning_count(s)),
    )
}

pub fn solution_day_06_02(file_path: String) -> Option<usize> {
    Some(winning_count(&parse_combined(
        &fs::read_to_string(file_path).expect("Invalid Input."),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_06_01() {
        let file_path: String = String::from("src/inputs/day06e.txt");
        let result = solution_day_06_01(file_path).unwrap();
        assert_eq!(result, 288);
    }

    #[test]
    fn test_day_06_02() {
        let file_path: String = String::from("src/inputs/day06e.txt");
        let result = solution_day_06_02(file_path).unwrap();
        assert_eq!(result, 71503);
    }

    #[test]
    #[ignore]
    fn output_day_06_01() {
        let file_path: String = String::from("src/inputs/day06.txt");
        let result = solution_day_06_01(file_path).unwrap();
        assert_eq!(result, 1312850);
    }

    #[test]
    #[ignore]
    fn output_day_06_02() {
        let file_path: String = String::from("src/inputs/day06.txt");
        let result = solution_day_06_02(file_path).unwrap();
        assert_eq!(result, 36749103);
    }
}
