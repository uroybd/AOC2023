use std::fs;

// Advent of Code 2023 - Day 06

struct Stat {
    time: usize,
    distance: usize,
}

impl Stat {
    fn winning_count(&self) -> usize {
        let offset = if let 0 = self.time % 2 { 1 } else { 0 };
        (0..=self.time / 2).fold(0, |acc, t| {
            acc + ((self.time - t) * t > self.distance) as usize
        }) * 2
            - offset
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
            .replace(' ', "")
            .split_once(':')
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap(),
        distance: distances
            .replace(' ', "")
            .split_once(':')
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap(),
    }
}

pub fn solution_day_06_01(file_path: String) -> Option<usize> {
    Some(
        parse(&fs::read_to_string(file_path).expect("Invalid Input."))
            .iter()
            .fold(1, |acc, s| acc * s.winning_count()),
    )
}

pub fn solution_day_06_02(file_path: String) -> Option<usize> {
    Some(parse_combined(&fs::read_to_string(file_path).expect("Invalid Input.")).winning_count())
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
