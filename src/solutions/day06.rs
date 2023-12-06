use std::fs;

// Advent of Code 2023 - Day 06

#[derive(Debug)]
struct Stat {
    time: usize,
    distance: usize,
}

impl Stat {
    fn winning_count(&self) -> usize {
        (0..=self.time)
            .map(|t| (self.time - t) * t)
            .filter(|v| v > &self.distance)
            .count()
    }
}

fn parse(data: &str) -> Vec<Stat> {
    let (times, distances) = data.split_once('n').unwrap();
    times
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .zip(distances.split_once(':').unwrap().1.split_whitespace())
        .map(|(t, d)| Stat {
            time: t.parse::<usize>().unwrap(),
            distance: d.parse::<usize>().unwrap(),
        })
        .collect()
}

fn parse_combined(data: &str) -> Stat {
    let (times, distances) = data.split_once('\n').unwrap();
    Stat {
        time: times
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .collect::<String>()
            .trim()
            .parse::<usize>()
            .unwrap(),
        distance: distances
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .collect::<String>()
            .trim()
            .parse::<usize>()
            .unwrap(),
    }
}

pub fn solution_day_06_01(file_path: String) -> Option<usize> {
    let stats = parse(&fs::read_to_string(file_path).expect("Invalid Input."));
    Some(stats.iter().fold(1, |acc, s| acc * s.winning_count()))
}

pub fn solution_day_06_02(file_path: String) -> Option<usize> {
    let stat = parse_combined(&fs::read_to_string(file_path).expect("Invalid Input."));
    Some(stat.winning_count())
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
        let result = solution_day_06_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_06_02() {
        let file_path: String = String::from("src/inputs/day06.txt");
        let result = solution_day_06_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
