use std::fs;

// Advent of Code 2023 - Day 18

struct Instruction {
    dir: u8,
    len: u64,
    c: u32,
}

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.splitn(3, ' ');
        let dir = match s.next().unwrap() {
            "R" => 0,
            "D" => 1,
            "L" => 2,
            "U" => 3,
            _ => unreachable!(),
        };
        let len = s.next().unwrap().parse::<u64>().unwrap();
        let c = u32::from_str_radix(
            s.next()
                .unwrap()
                .trim_matches(|s| s == '(' || s == ')' || s == '#'),
            16,
        )
        .unwrap();

        Ok(Self { dir, len, c })
    }
}

fn get_end(start: (i64, i64), dir: u8, len: u64) -> (i64, i64) {
    match dir {
        0 => (start.0.wrapping_add_unsigned(len), start.1),
        1 => (start.0, start.1.wrapping_add_unsigned(len)),
        2 => (start.0.wrapping_sub_unsigned(len), start.1),
        3 => (start.0, start.1.wrapping_sub_unsigned(len)),
        _ => unreachable!(),
    }
}

impl Instruction {
    fn get_vector(&self) -> (u8, u64) {
        (self.dir, self.len)
    }

    fn get_vector_from_color(&self) -> (u8, u64) {
        ((self.c & 3) as u8, (self.c >> 4) as u64)
    }
}

fn shoelace_area(instructions: &[Instruction], vector_func: fn(&Instruction) -> (u8, u64)) -> u64 {
    let mut perimeter = 0;
    let mut sum = 0;
    let mut prev = (0, 0);
    for i in instructions.iter() {
        let (dir, len) = vector_func(i);

        let next = get_end(prev, dir, len);
        sum += (prev.1 + next.1) * (prev.0 - next.0);
        perimeter += len;
        prev = next;
    }
    perimeter.wrapping_add_signed(sum) / 2 + 1
}
pub fn solution_day_18_01(file_path: String) -> Option<u64> {
    let plan: Vec<Instruction> = fs::read_to_string(file_path)
        .expect("Invalid Input File.")
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect();
    Some(shoelace_area(&plan, |i| i.get_vector()))
}

pub fn solution_day_18_02(file_path: String) -> Option<u64> {
    let plan: Vec<Instruction> = fs::read_to_string(file_path)
        .expect("Invalid Input File.")
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect();
    Some(shoelace_area(&plan, |i| i.get_vector_from_color()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_18_01() {
        let file_path: String = String::from("src/inputs/day18e.txt");
        let result = solution_day_18_01(file_path).unwrap();
        assert_eq!(result, 62);
    }

    #[test]
    fn test_day_18_02() {
        let file_path: String = String::from("src/inputs/day18e.txt");
        let result = solution_day_18_02(file_path).unwrap();
        assert_eq!(result, 952408144115);
    }

    #[test]
    #[ignore]
    fn output_day_18_01() {
        let file_path: String = String::from("src/inputs/day18.txt");
        let result = solution_day_18_01(file_path).unwrap();
        assert_eq!(result, 36807);
    }

    #[test]
    #[ignore]
    fn output_day_18_02() {
        let file_path: String = String::from("src/inputs/day18.txt");
        let result = solution_day_18_02(file_path).unwrap();
        assert_eq!(result, 48797603984357);
    }
}
