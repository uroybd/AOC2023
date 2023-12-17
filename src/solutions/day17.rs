// Advent of Code 2023 - Day 17

use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Hash)]
struct Graph {
    nodes: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGraphError;

impl std::str::FromStr for Graph {
    type Err = ParseGraphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nodes: Vec<Vec<usize>> = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        let width: usize = nodes[0].len();
        let height: usize = nodes.len();
        Ok(Graph {
            nodes,
            width,
            height,
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Cursor {
    pos: (usize, usize),
    dir: (isize, isize),
    con_moves: usize,
}

#[derive(Debug)]
struct Record(usize, Cursor);

impl Ord for Record {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Record {}

impl Graph {
    fn find_neighbors(&self, cursor: &Cursor, min_moves: usize, max_moves: usize) -> Vec<Cursor> {
        let (x, y) = cursor.pos;
        let mut result = vec![];
        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        for dir in directions {
            let mut moves_so_far = 1;
            if dir == cursor.dir {
                moves_so_far += cursor.con_moves;
            }
            if moves_so_far > max_moves {
                continue;
            }
            if -dir.0 == cursor.dir.0 && -dir.1 == cursor.dir.1 {
                continue;
            }
            let (nx, ny) = (x as isize + dir.0, y as isize + dir.1);
            if nx < 0 || ny < 0 || nx as usize >= self.width || ny as usize >= self.height {
                continue;
            }
            if dir != cursor.dir && cursor.con_moves < min_moves {
                continue;
            }
            result.push(Cursor {
                pos: (nx as usize, ny as usize),
                dir,
                con_moves: moves_so_far,
            });
        }
        result
    }

    fn find_shortest(&self, min_moves: usize, max_moves: usize) -> usize {
        let mut bests = HashMap::new();
        let mut boundary = BinaryHeap::new();

        for dir in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let cursor = Cursor {
                pos: (0, 0),
                dir,
                con_moves: 0,
            };
            boundary.push(Record(0, cursor));
        }
        while let Some(Record(val, cursor)) = boundary.pop() {
            if bests.contains_key(&cursor) {
                continue;
            }
            bests.insert(cursor.clone(), val);
            for next in self.find_neighbors(&cursor, min_moves, max_moves) {
                if !bests.contains_key(&next) {
                    let val = bests.get(&cursor).unwrap() + self.nodes[next.pos.1][next.pos.0];
                    boundary.push(Record(val, next));
                }
            }
        }

        let res = bests
            .iter()
            .filter_map(|(c, v)| {
                if c.pos.0 == self.width - 1
                    && c.pos.1 == self.height - 1
                    && c.con_moves >= min_moves
                {
                    Some(v)
                } else {
                    None
                }
            })
            .min();
        *res.unwrap()
    }
}

pub fn solution_day_17_01(file_path: String) -> Option<usize> {
    let graph: Graph = std::fs::read_to_string(file_path).unwrap().parse().unwrap();
    Some(graph.find_shortest(1, 3))
}

pub fn solution_day_17_02(file_path: String) -> Option<usize> {
    let graph: Graph = std::fs::read_to_string(file_path).unwrap().parse().unwrap();
    Some(graph.find_shortest(4, 10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_17_01() {
        let file_path: String = String::from("src/inputs/day17e.txt");
        let result = solution_day_17_01(file_path).unwrap();
        assert_eq!(result, 102);
    }

    #[test]
    fn test_day_17_02() {
        let file_path: String = String::from("src/inputs/day17e.txt");
        let result = solution_day_17_02(file_path).unwrap();
        assert_eq!(result, 94);
        let file_path: String = String::from("src/inputs/day17e2.txt");
        let result = solution_day_17_02(file_path).unwrap();
        assert_eq!(result, 71);
    }

    #[test]
    #[ignore]
    fn output_day_17_01() {
        let file_path: String = String::from("src/inputs/day17.txt");
        let result = solution_day_17_01(file_path).unwrap();
        assert_eq!(result, 724);
    }

    #[test]
    #[ignore]
    fn output_day_17_02() {
        let file_path: String = String::from("src/inputs/day17.txt");
        let result = solution_day_17_02(file_path).unwrap();
        assert_eq!(result, 877);
    }
}
