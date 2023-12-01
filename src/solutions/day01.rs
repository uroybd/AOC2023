use std::fs;
use regex::Regex;

// Advent of Code 2022 - Day 01
fn get_calibration_value(val: &str) -> u32 {
    const RADIX: u32 = 10;
    let mut first_val: Option<u32> = None;
    let mut last_val: Option<u32> = None;
    
    for c in val.chars() {
        if let Some(v) = c.to_digit(RADIX) {
            if first_val.is_none() {
                first_val = Some(v);
                last_val = Some(v);
            } else {
                last_val = Some(v);
            }
        };
    }
    if last_val.is_some() && first_val.is_some() {
        (first_val.unwrap() * 10) + last_val.unwrap()
    } else {
         0
    }
}

fn get_calibration_value_extended(val: &str) -> u32 {
    let pattern = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[1-9]").expect("Unable to compile regex");
    let mut first_val: Option<u32> = None;
    let mut last_val: Option<u32> = None;
    for i in 0..val.len() {
        if let Some(m) = pattern.find(&val[i..]) {
            let v: u32 = match m.as_str() {
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
            if first_val.is_none() {
                first_val = Some(v);
                last_val = Some(v);
            } else {
                last_val = Some(v);
            }
            
        }
    }
    
    if last_val.is_some() && first_val.is_some() {
        (first_val.unwrap() * 10) + last_val.unwrap()
    } else {
         0
    }
}

pub fn solution_day_01_01(file_path: String) -> Option<u32> {
    Some(fs::read_to_string(file_path).expect("Invalid File").trim().lines().map(get_calibration_value).sum())
}

pub fn solution_day_01_02(file_path: String) -> Option<u32> {
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
        let result = solution_day_01_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_01_02() {
        let file_path: String = String::from("src/inputs/day01.txt");
        let result = solution_day_01_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
