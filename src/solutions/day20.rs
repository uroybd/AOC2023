// Advent of Code 2023 - Day 20


pub fn solution_day_20_01(file_path: String) -> Option<usize> {
    None
}

pub fn solution_day_20_02(file_path: String) -> Option<usize> {
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_20_01() {
        let file_path: String = String::from("src/inputs/day20e.txt");
        let result = solution_day_20_01(file_path).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_day_20_02() {
        let file_path: String = String::from("src/inputs/day20e.txt");
        let result = solution_day_20_02(file_path).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    #[ignore]
    fn output_day_20_01() {
        let file_path: String = String::from("src/inputs/day20.txt");
        let result = solution_day_20_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_20_02() {
        let file_path: String = String::from("src/inputs/day20.txt");
        let result = solution_day_20_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
