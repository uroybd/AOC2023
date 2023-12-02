use std::fs;

// Advent of Code 2022 - Day 02

#[derive(Debug)]
struct Turn {
    red: isize,
    blue: isize,
    green: isize
}

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
        Self { red, blue, green }
    }
}

#[derive(Debug)]
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

    pub fn find_cube_more_than(&self, color: &str, cap: isize) -> Option<&Turn> {
        match color {
            "red" => self.turns.iter().find(|t| t.red > cap),
            "blue" => self.turns.iter().find(|t| t.blue > cap),
            _ => self.turns.iter().find(|t| t.green > cap),
        }
    }

    pub fn power(&self) -> isize {
        // TODO: optimize
        let red = self.turns.iter().map(|t| t.red).max().unwrap();
        let green = self.turns.iter().map(|t| t.green).max().unwrap();
        let blue = self.turns.iter().map(|t| t.blue).max().unwrap();
        red * green * blue
    }

}

pub fn solution_day_02_01(file_path: String) -> Option<isize> {
    let games: Vec<Game> = fs::read_to_string(file_path).expect("Invalid input file.").lines().map(Game::from_str).collect();

    let result: isize = games.iter().filter(|g| {
        g.find_cube_more_than("red", 12).is_none() && g.find_cube_more_than("green", 13).is_none() && g.find_cube_more_than("blue", 14).is_none()
    }).map(|g| g.id).sum();
    Some(result)
}

pub fn solution_day_02_02(file_path: String) -> Option<isize> {
    let games: Vec<Game> = fs::read_to_string(file_path).expect("Invalid input file.").lines().map(Game::from_str).collect();
    let result = games.iter().map(|g| g.power()).sum();
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
        let result = solution_day_02_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_02_02() {
        let file_path: String = String::from("src/inputs/day02.txt");
        let result = solution_day_02_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
