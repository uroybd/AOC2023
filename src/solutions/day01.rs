use std::fs;
use regex::Regex;

// Advent of Code 2022 - Day 01

// An regex-based solution is possible but too slow
fn get_calibration_value(val: &str) -> u32 {
    let mut digits = vec![];

    for c in val.chars() {
        if c.is_ascii_digit() {
            digits.push(c.to_digit(10).unwrap())
        }
    }
    (digits.first().unwrap() * 10) + digits.last().unwrap()
}

fn get_calibration_value_extended(val: &str) -> i32 {
    let pattern = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").expect("Unable to compile regex");
    let mut digits = vec![];
    let mut idx = 0;
    let length = val.len();
    while idx < length {
        if let Some(m) = pattern.find(&val[idx..]) {
            let v: i32 = match m.as_str() {
                "one" | "1" => 1,
                "two" | "2" => 2,
                "three" | "3" => 3,
                "four" | "4" => 4,
                "five" | "5" => 5,
                "six" | "6" => 6,
                "seven" | "7" => 7,
                "eight" | "8" => 8,
                "nine" | "9" => 9,
                _ => todo!()
            };
            digits.push(v);
            idx += m.start() + 1;
        } else {
            idx = length;
        }
    }
    
    (digits.first().unwrap() * 10) + digits.last().unwrap()
}

pub fn solution_day_01_01(file_path: String) -> Option<u32> {
    Some(fs::read_to_string(file_path).expect("Invalid File").trim().lines().map(get_calibration_value).sum())
}

pub fn solution_day_01_02(file_path: String) -> Option<i32> {
    Some(fs::read_to_string(file_path).expect("Invalid File").trim().lines().map(get_calibration_value_extended).sum())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01_01() {
        let file_path: String = String::from("src/inputs/day01e.txt");
        let result = solution_day_01_01(file_path).unwrap();
        assert_eq!(result, 142);
    }

    #[test]
    fn test_day_01_02() {
        let file_path: String = String::from("src/inputs/day01e2.txt");
        let result = solution_day_01_02(file_path).unwrap();
        assert_eq!(result, 281);
    }

    #[test]
    #[ignore]
    fn output_day_01_01() {
        let file_path: String = String::from("src/inputs/day01.txt");
        let result = solution_day_01_01(file_path).unwrap();
        assert_eq!(result, 53974);
    }

    #[test]
    #[ignore]
    fn output_day_01_02() {
        let file_path: String = String::from("src/inputs/day01.txt");
        let result = solution_day_01_02(file_path).unwrap();
        assert_eq!(result, 52840);
    }
}
