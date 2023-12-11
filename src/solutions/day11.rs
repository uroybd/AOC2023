// Advent of Code 2023 - Day 11

use rayon::prelude::*;
use std::fs;

struct Observation {
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseObservationError;

impl std::str::FromStr for Observation {
    type Err = ParseObservationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

        let galaxies: Vec<(usize, usize)> = data
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, c)| if *c == '#' { Some((x, y)) } else { None })
            })
            .collect();
        let empty_cols = (0..data[0].len())
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
            galaxies,
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
        let mut xs = [p1.0, p2.0];
        xs.sort();

        let mut ys = [p1.1, p2.1];
        ys.sort();

        let expanded = self
            .empty_cols
            .iter()
            .filter(|x| x >= &&(xs[0] + 1) && x <= &&xs[1])
            .count()
            + self
                .empty_rows
                .iter()
                .filter(|y| y >= &&(ys[0] + 1) && y <= &&ys[1])
                .count();

        xs[1].abs_diff(xs[0]) + ys[1].abs_diff(ys[0]) + (expanded * multiplier) - expanded
    }

    fn get_all_galaxy_distances(&self, multiplier: usize) -> usize {
        let mut galaxies = self.galaxies.clone();

        let mut distances = 0;
        while let Some(v) = galaxies.pop() {
            distances += galaxies
                .par_iter()
                .map(|p| self.calculate_distance(p, &v, multiplier))
                .sum::<usize>();
        }
        distances
    }
}

pub fn solution_day_11(file_path: String, multiplier: usize) -> Option<usize> {
    let data: Observation = fs::read_to_string(file_path)
        .expect("Invalid File")
        .parse()
        .unwrap();
    Some(data.get_all_galaxy_distances(multiplier))
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
