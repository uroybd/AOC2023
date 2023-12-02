use std::fs;

// Advent of Code 2022 - Day 02

struct Turn(isize, isize, isize);

impl Turn {
    pub fn from_str(data: &str) -> Self {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        data.split(", ").for_each(|f| {
            let pair = f.split_once(' ').unwrap();
            match pair {
                (num, "red") => {
                    red = num.parse::<isize>().unwrap();
                },
                (num, "blue") => {
                    blue = num.parse::<isize>().unwrap();
                },
                (num, _) => {
                    green = num.parse::<isize>().unwrap();
                }
            };
        });
        Turn ( red, blue, green )
    }
}

struct Game {
    id: isize,
    turns: Vec<Turn>
}

impl Game {
    pub fn from_str(data: &str) -> Self {
        let pair = data.split_once(": ").unwrap();
        Self {
            id: pair.0.split_once(" ").unwrap().1.parse::<isize>().unwrap(),
            turns: pair.1.split("; ").map(Turn::from_str).collect()
        }
    }

    pub fn is_valid(&self, caps: (isize, isize, isize)) -> bool {
        for t in self.turns.iter() {
            if t.0 > caps.0 || t.1 > caps.1 || t.2 > caps.2 {
                return false;
            }
        }
        true
    }

    pub fn power(&self) -> isize {
        let val = self.turns.iter()
        .fold((0, 0, 0), |acc, t| (t.0.max(acc.0), t.1.max(acc.1), t.2.max(acc.2)));
        val.0 * val.1 * val.2
    }

}

pub fn solution_day_02_01(file_path: String) -> Option<isize> {
    let result = fs::read_to_string(file_path)
    .expect("Invalid input file.").lines()
    .map(Game::from_str)
    .filter(|g| g.is_valid((12, 14, 13)))
    .map(|g| g.id)
    .sum();
    Some(result)
}

pub fn solution_day_02_02(file_path: String) -> Option<isize> {
    let result = fs::read_to_string(file_path)
    .expect("Invalid input file.")
    .lines()
    .map(Game::from_str)
    .map(|g| g.power())
    .sum();
    Some(result)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_02_01() {
        let file_path: String = String::from("src/inputs/day02e.txt");
        let result = solution_day_02_01(file_path).unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn test_day_02_02() {
        let file_path: String = String::from("src/inputs/day02e.txt");
        let result = solution_day_02_02(file_path).unwrap();
        assert_eq!(result, 2286);
    }

    #[test]
    #[ignore]
    fn output_day_02_01() {
        let file_path: String = String::from("src/inputs/day02.txt");
        let result = solution_day_02_01(file_path).unwrap();
        assert_eq!(result, 2727);
    }

    #[test]
    #[ignore]
    fn output_day_02_02() {
        let file_path: String = String::from("src/inputs/day02.txt");
        let result = solution_day_02_02(file_path).unwrap();
        assert_eq!(result, 56580);
    }
}
