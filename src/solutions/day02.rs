use std::fs;

// Advent of Code 2022 - Day 02
struct Game {
    id: isize,
    turn_max: (isize, isize, isize)
}

impl Game {
    pub fn from_str(data: &str) -> Self {
        let pair = data.split_once(": ").unwrap();
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        for turn in pair.1.split("; ") {
            for color in turn.split(", ") {
                let pair = color.split_once(' ').unwrap();
                match pair {
                    (num, "red") => {
                        red = red.max(num.parse::<isize>().unwrap());
                    },
                    (num, "blue") => {
                        blue = blue.max(num.parse::<isize>().unwrap());
                    },
                    (num, _) => {
                        green = green.max(num.parse::<isize>().unwrap());
                    }
                };
            }
        }
        Self {
            id: pair.0.split_once(" ").unwrap().1.parse::<isize>().unwrap(),
            turn_max: (red, blue, green)
        }
    }

    pub fn is_valid(&self, caps: (isize, isize, isize)) -> bool {
        self.turn_max.0 <= caps.0 && self.turn_max.1 <= caps.1 && self.turn_max.2 <= caps.2
    }

    pub fn power(&self) -> isize {
        self.turn_max.0 * self.turn_max.1 * self.turn_max.2
    }

}

pub fn solution_day_02_01(file_path: String) -> Option<isize> {
    let result = fs::read_to_string(file_path)
    .expect("Invalid input file.").lines()
    .map(Game::from_str)
    .filter_map(|g| {
        if g.is_valid((12, 14, 13)){
            Some(g.id)
        } else {
            None
        }
    })
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
