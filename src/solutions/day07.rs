// Advent of Code 2023 - Day 07

use std::{collections::HashMap, fs};

#[derive(PartialEq, Eq)]
struct Hand {
    value: usize,
    bid: usize,
    cards: String,
}

impl Hand {
    fn get_value(hand: &str) -> usize {
        let mut counts: HashMap<char, usize> =
            hand.chars().fold(HashMap::new(), |mut counter, card| {
                let v = counter.entry(card).or_insert(0);
                *v += 1;
                counter
            });

        let j_val = counts.remove(&'1').unwrap_or(0);
        if j_val == 5 {
            return 7;
        }
        let mut counts: Vec<usize> = counts.values().cloned().collect();
        counts.sort_by(|a, b| b.cmp(a));
        if !counts.is_empty() {
            counts[0] += j_val
        }
        match &counts[..] {
            [5] => 7,
            [4, 1] => 6,
            [3, 2] => 5,
            [3, ..] => 4,
            [2, 2, 1] => 3,
            [2, ..] => 2,
            _ => 1,
        }
    }

    fn from_str(val: &str, wild: bool) -> Self {
        let j = if wild { '1' } else { 'U' };
        let (h, b) = val.split_once(' ').unwrap();
        let cards: String = h
            .chars()
            .map(|c| match c {
                'J' => j,
                'Q' => 'V',
                'K' => 'W',
                'A' => 'X',
                o => o,
            })
            .collect();
        let value = Hand::get_value(&cards);
        Self {
            cards,
            value,
            bid: b.trim().parse().unwrap(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.value.cmp(&other.value) {
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            o => o,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn get_total(file_path: String, wild: bool) -> Option<usize> {
    let mut hands: Vec<Hand> = fs::read_to_string(file_path)
        .expect("Invalid Input File.")
        .lines()
        .map(|l| Hand::from_str(l, wild))
        .collect();
    hands.sort();
    Some(
        hands
            .iter()
            .enumerate()
            .fold(0, |total, (idx, hand)| total + (hand.bid * (idx + 1))),
    )
}

pub fn solution_day_07_01(file_path: String) -> Option<usize> {
    get_total(file_path, false)
}

pub fn solution_day_07_02(file_path: String) -> Option<usize> {
    get_total(file_path, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_07_01() {
        let file_path: String = String::from("src/inputs/day07e.txt");
        let result = solution_day_07_01(file_path).unwrap();
        assert_eq!(result, 6440);
    }

    #[test]
    fn test_day_07_02() {
        let file_path: String = String::from("src/inputs/day07e.txt");
        let result = solution_day_07_02(file_path).unwrap();
        assert_eq!(result, 5905);
    }

    #[test]
    #[ignore]
    fn output_day_07_01() {
        let file_path: String = String::from("src/inputs/day07.txt");
        let result = solution_day_07_01(file_path).unwrap();
        assert_eq!(result, 241344943);
    }

    #[test]
    #[ignore]
    fn output_day_07_02() {
        let file_path: String = String::from("src/inputs/day07.txt");
        let result = solution_day_07_02(file_path).unwrap();
        assert_eq!(result, 243101568);
    }
}
