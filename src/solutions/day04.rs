use std::fs;

// Advent of Code 2023 - Day 04

#[derive(Clone, Debug)]
struct Card {
    winning: Vec<usize>,
    available: Vec<usize>,
}

impl Card {
    pub fn parse(inp: &str) -> Self {
        let (_, numbers) = inp.split_once(':').unwrap();
        let mut card = Self {
            winning: vec![],
            available: vec![],
        };
        let (winning, available) = numbers.split_once(" | ").unwrap();
        card.winning = winning
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect();
        card.available = available
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect();
        card
    }

    pub fn win_count(&self) -> usize {
        self.available
            .iter()
            .filter(|a| self.winning.contains(a))
            .count()
    }

    pub fn points(&self) -> usize {
        let total = self.win_count();
        if total < 3 {
            return total;
        }
        2_usize.pow((total - 1).try_into().unwrap())
    }
}

fn total_won(cards: &Vec<Card>) -> usize {
    let mut count_cache = vec![1; cards.len()];
    for (index, card) in cards.iter().enumerate() {
        let won_count = card.win_count();
        if won_count > 0 {
            for x in index + 1..=(index + won_count) {
                count_cache[x] += count_cache[index]
            }
        }
    }
    count_cache.iter().sum()
}

pub fn solution_day_04_01(file_path: String) -> Option<usize> {
    Some(
        fs::read_to_string(file_path)
            .expect("Invalid File")
            .lines()
            .map(|l| Card::parse(l).points())
            .sum(),
    )
}

pub fn solution_day_04_02(file_path: String) -> Option<usize> {
    let cards: Vec<Card> = fs::read_to_string(file_path)
        .expect("Invalid File")
        .lines()
        .map(Card::parse)
        .collect();
    Some(total_won(&cards))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_04_01() {
        let file_path: String = String::from("src/inputs/day04e.txt");
        let result = solution_day_04_01(file_path).unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn test_day_04_02() {
        let file_path: String = String::from("src/inputs/day04e.txt");
        let result = solution_day_04_02(file_path).unwrap();
        assert_eq!(result, 30);
    }

    #[test]
    #[ignore]
    fn output_day_04_01() {
        let file_path: String = String::from("src/inputs/day04.txt");
        let result = solution_day_04_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_04_02() {
        let file_path: String = String::from("src/inputs/day04.txt");
        let result = solution_day_04_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
