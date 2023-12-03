use std::fs;

// Advent of Code 2022 - Day 02
struct Game {
    id: usize,
    turn_max: [usize; 3]
}

impl Game {
    pub fn from_str(data: &str) -> Self {
        let (game, turns) = data.split_once(": ").unwrap();
        let mut turn_max = [0; 3];
        for color_val in turns.split([';', ',']) {
            let (val, color) = color_val.trim().split_once(' ').unwrap();
            match color {
                "red" => turn_max[0] = turn_max[0].max(val.parse::<usize>().unwrap()),
                "blue" => turn_max[1] = turn_max[1].max(val.parse::<usize>().unwrap()),
                _ => turn_max[2] = turn_max[2].max(val.parse::<usize>().unwrap()),
            }
        };
        Self {
            id: game.split_once(' ').unwrap().1.parse::<usize>().unwrap(),
            turn_max
        }
    }

    pub fn is_valid(&self, caps: &[usize; 3]) -> bool {
        for (idx, val) in self.turn_max.iter().enumerate() {
            if val > &caps[idx] {
                return false;
            }
        }
        true
    }

    pub fn power(&self) -> usize {
        self.turn_max.iter().copied().reduce(|acc, a| acc * a).unwrap()
    }

}

pub fn solution_day_02_01(file_path: String) -> Option<usize> {
    let result = fs::read_to_string(file_path)
    .expect("Invalid input file.").lines()
    .filter_map(|l| {
        let g = Game::from_str(l);
        if g.is_valid(&[12, 14, 13]){ Some(g.id) } else { None }
    })
    .sum();
    Some(result)
}

pub fn solution_day_02_02(file_path: String) -> Option<usize> {
    let result = fs::read_to_string(file_path)
    .expect("Invalid input file.")
    .lines()
    .map(|l| Game::from_str(l).power())
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
