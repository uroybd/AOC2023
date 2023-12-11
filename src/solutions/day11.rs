// Advent of Code 2023 - Day 11

use rayon::prelude::*;
use std::fs;

struct Observation {
    data: Vec<Vec<char>>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseObservationError;

impl std::str::FromStr for Observation {
    type Err = ParseObservationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

        let width = data[0].len();
        let height = data.len();
        let empty_cols = (0..width)
            .filter(|i| data.iter().all(|row| row[*i] == '.'))
            .collect();
        let empty_rows = data
            .iter()
            .enumerate()
            .filter_map(|(i, row)| {
                if row.iter().all(|c| *c == '.') {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        Ok(Observation {
            data,
            width,
            height,
            empty_cols,
            empty_rows,
        })
    }
}

impl Observation {
    fn calculate_distance(
        &self,
        p1: &(usize, usize),
        p2: &(usize, usize),
        multiplier: usize,
    ) -> usize {
        let xs: usize = ((p1.0.min(p2.0) + 1)..(p1.0.max(p2.0) + 1))
            .map(|x| {
                if self.empty_cols.contains(&x) {
                    multiplier
                } else {
                    1
                }
            })
            .sum();
        let ys: usize = ((p1.1.min(p2.1) + 1)..(p1.1.max(p2.1) + 1))
            .map(|y| {
                if self.empty_rows.contains(&y) {
                    multiplier
                } else {
                    1
                }
            })
            .sum();
        xs + ys
    }

    fn get_all_galaxy_distances(&self, multiplier: usize) -> Vec<usize> {
        let mut galaxies = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                if self.data[y][x] == '#' {
                    galaxies.push((x, y));
                }
            }
        }
        let mut distances = vec![];
        while let Some(v) = galaxies.pop() {
            let calculated: Vec<usize> = galaxies
                .par_iter()
                .map(|p| self.calculate_distance(p, &v, multiplier))
                .collect();
            distances.extend(calculated);
        }
        distances
    }
}

pub fn solution_day_11(file_path: String, multiplier: usize) -> Option<usize> {
    let data: Observation = fs::read_to_string(file_path)
        .expect("Invalid File")
        .parse()
        .unwrap();
    Some(data.get_all_galaxy_distances(multiplier).iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_11_01() {
        let file_path: String = String::from("src/inputs/day11e.txt");
        let result = solution_day_11(file_path, 2).unwrap();
        assert_eq!(result, 374);
    }

    #[test]
    fn test_day_11_02() {
        let file_path: String = String::from("src/inputs/day11e.txt");
        let result = solution_day_11(file_path, 100).unwrap();
        assert_eq!(result, 8410);
    }

    #[test]
    #[ignore]
    fn output_day_11_01() {
        let file_path: String = String::from("src/inputs/day11.txt");
        let result = solution_day_11(file_path, 2).unwrap();
        assert_eq!(result, 9742154);
    }

    #[test]
    #[ignore]
    fn output_day_11_02() {
        let file_path: String = String::from("src/inputs/day11.txt");
        let result = solution_day_11(file_path, 1000000).unwrap();
        assert_eq!(result, 411142919886);
    }
}
