// Advent of Code 2023 - Day 16
use rayon::prelude::*;
use std::sync::mpsc::channel;
use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum MovementDirection {
    Rightward,
    LeftWard,
    Upward,
    Downward,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum RoomTile {
    Empty,
    RightTiltedMirror,
    LeftTiltedMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl std::convert::From<char> for RoomTile {
    fn from(s: char) -> Self {
        match s {
            '.' => Self::Empty,
            '/' => Self::RightTiltedMirror,
            '\\' => Self::LeftTiltedMirror,
            '|' => Self::VerticalSplitter,
            '-' => Self::HorizontalSplitter,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Photon {
    position: (isize, isize),
    direction: MovementDirection,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct MirrorRoom {
    room: Vec<Vec<RoomTile>>,
    width: isize,
    height: isize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMirrorRoomError;

impl std::str::FromStr for MirrorRoom {
    type Err = ParseMirrorRoomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let room: Vec<Vec<RoomTile>> = s
            .lines()
            .map(|l| l.chars().map(|c| RoomTile::from(c)).collect())
            .collect();
        let width = room[0].len() as isize;
        let height = room.len() as isize;
        Ok(Self {
            room,
            width,
            height,
        })
    }
}

impl MirrorRoom {
    fn get_next_photon(&self, photon: &Photon) -> Option<Vec<Photon>> {
        let (x, y) = photon.position;
        let new_pos;
        match photon.direction {
            MovementDirection::Rightward => {
                if x + 1 == self.width {
                    return None;
                }
                new_pos = (x + 1, y);
            }
            MovementDirection::LeftWard => {
                if x == 0 {
                    return None;
                }
                new_pos = (x - 1, y);
            }
            MovementDirection::Upward => {
                if y == 0 {
                    return None;
                }
                new_pos = (x, y - 1);
            }
            MovementDirection::Downward => {
                if y + 1 == self.height {
                    return None;
                }
                new_pos = (x, y + 1);
            }
        }
        let tile = &self.room[new_pos.1 as usize][new_pos.0 as usize];

        match tile {
            RoomTile::Empty => Some(vec![Photon {
                position: new_pos,
                direction: photon.direction.clone(),
            }]),
            RoomTile::RightTiltedMirror => match photon.direction {
                MovementDirection::Rightward => Some(vec![Photon {
                    position: new_pos,
                    direction: MovementDirection::Upward,
                }]),
                MovementDirection::LeftWard => Some(vec![Photon {
                    position: new_pos,
                    direction: MovementDirection::Downward,
                }]),
                MovementDirection::Upward => Some(vec![Photon {
                    position: new_pos,
                    direction: MovementDirection::Rightward,
                }]),
                MovementDirection::Downward => Some(vec![Photon {
                    position: new_pos,
                    direction: MovementDirection::LeftWard,
                }]),
            },
            RoomTile::LeftTiltedMirror => match photon.direction {
                MovementDirection::Rightward => Some(vec![Photon {
                    position: new_pos,
                    direction: MovementDirection::Downward,
                }]),
                MovementDirection::LeftWard => Some(vec![Photon {
                    position: new_pos,
                    direction: MovementDirection::Upward,
                }]),
                MovementDirection::Upward => Some(vec![Photon {
                    position: new_pos,
                    direction: MovementDirection::LeftWard,
                }]),
                MovementDirection::Downward => Some(vec![Photon {
                    position: new_pos,
                    direction: MovementDirection::Rightward,
                }]),
            },
            RoomTile::VerticalSplitter => match photon.direction {
                MovementDirection::Rightward => Some(vec![
                    Photon {
                        position: new_pos,
                        direction: MovementDirection::Upward,
                    },
                    Photon {
                        position: new_pos,
                        direction: MovementDirection::Downward,
                    },
                ]),
                MovementDirection::LeftWard => Some(vec![
                    Photon {
                        position: new_pos,
                        direction: MovementDirection::Upward,
                    },
                    Photon {
                        position: new_pos,
                        direction: MovementDirection::Downward,
                    },
                ]),
                MovementDirection::Upward => Some(vec![Photon {
                    position: new_pos,
                    direction: MovementDirection::Upward,
                }]),
                MovementDirection::Downward => Some(vec![Photon {
                    position: new_pos,
                    direction: MovementDirection::Downward,
                }]),
            },
            RoomTile::HorizontalSplitter => match photon.direction {
                MovementDirection::Rightward => Some(vec![Photon {
                    position: new_pos,
                    direction: MovementDirection::Rightward,
                }]),
                MovementDirection::LeftWard => Some(vec![Photon {
                    position: new_pos,
                    direction: MovementDirection::LeftWard,
                }]),
                MovementDirection::Upward => Some(vec![
                    Photon {
                        position: new_pos,
                        direction: MovementDirection::LeftWard,
                    },
                    Photon {
                        position: new_pos,
                        direction: MovementDirection::Rightward,
                    },
                ]),
                MovementDirection::Downward => Some(vec![
                    Photon {
                        position: new_pos,
                        direction: MovementDirection::LeftWard,
                    },
                    Photon {
                        position: new_pos,
                        direction: MovementDirection::Rightward,
                    },
                ]),
            },
        }
    }

    fn print_photons(&self, photons: &Vec<Photon>) {
        let mut room: Vec<Vec<char>> = self
            .room
            .iter()
            .map(|l| {
                l.iter()
                    .map(|t| match t {
                        RoomTile::Empty => '.',
                        RoomTile::RightTiltedMirror => '/',
                        RoomTile::LeftTiltedMirror => '\\',
                        RoomTile::VerticalSplitter => '|',
                        RoomTile::HorizontalSplitter => '-',
                    })
                    .collect()
            })
            .collect();
        for photon in photons {
            room[photon.position.1 as usize][photon.position.0 as usize] = match photon.direction {
                MovementDirection::Rightward => '>',
                MovementDirection::LeftWard => '<',
                MovementDirection::Upward => '^',
                MovementDirection::Downward => 'v',
            }
        }
        for l in room {
            println!("{}", l.iter().collect::<String>());
        }
    }

    fn find_photons(&self, starter: &Photon) -> usize {
        let mut available = vec![];
        let mut seen = HashSet::new();
        let mut processed = vec![];
        if let Some(next_photons) = self.get_next_photon(starter) {
            available.extend(next_photons);
        }
        while let Some(photon) = available.pop() {
            if seen.contains(&photon) {
                continue;
            }
            seen.insert(photon.clone());
            processed.push(photon.clone());
            if let Some(next_photons) = self.get_next_photon(&photon) {
                available.extend(next_photons);
            }
        }
        let pos_counter: HashSet<(isize, isize)> =
            HashSet::from_iter(seen.iter().map(|p| p.position));

        pos_counter.len()
    }
}

pub fn solution_day_16_01(file_path: String) -> Option<usize> {
    let mirror_room: MirrorRoom = fs::read_to_string(file_path).unwrap().parse().unwrap();
    Some(mirror_room.find_photons(&Photon {
        position: (-1, 0),
        direction: MovementDirection::Rightward,
    }))
}

pub fn solution_day_16_02(file_path: String) -> Option<usize> {
    let mirror_room: MirrorRoom = fs::read_to_string(file_path).unwrap().parse().unwrap();

    let (sender, receiver) = channel();
    (0..mirror_room.width)
        .into_par_iter()
        .for_each_with(sender, |s, x| {
            s.send(mirror_room.find_photons(&Photon {
                position: (x, -1),
                direction: MovementDirection::Downward,
            }))
            .unwrap();
            s.send(mirror_room.find_photons(&Photon {
                position: (x, mirror_room.height),
                direction: MovementDirection::Upward,
            }))
            .unwrap();
        });
    let (sender2, receiver2) = channel();
    (0..mirror_room.height)
        .into_par_iter()
        .for_each_with(sender2, |s, y| {
            s.send(mirror_room.find_photons(&Photon {
                position: (-1, y),
                direction: MovementDirection::Rightward,
            }))
            .unwrap();
            s.send(mirror_room.find_photons(&Photon {
                position: (mirror_room.width, y),
                direction: MovementDirection::LeftWard,
            }))
            .unwrap();
        });

    Some(
        receiver
            .iter()
            .max()
            .unwrap()
            .max(receiver2.iter().max().unwrap()),
    )

    // for x in (0..mirror_room.width).into_par_iter() {
    //     vals.push(mirror_room.find_photons(&Photon {
    //         position: (x, -1),
    //         direction: MovementDirection::Downward,
    //     }));
    //     vals.push(mirror_room.find_photons(&Photon {
    //         position: (x, mirror_room.height),
    //         direction: MovementDirection::Upward,
    //     }));
    // }
    // for y in 0..mirror_room.height {
    //     vals.push(mirror_room.find_photons(&Photon {
    //         position: (-1, y),
    //         direction: MovementDirection::Rightward,
    //     }));
    //     vals.push(mirror_room.find_photons(&Photon {
    //         position: (mirror_room.width, y),
    //         direction: MovementDirection::LeftWard,
    //     }));
    // }
    // Some(*vals.iter().max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_16_01() {
        let file_path: String = String::from("src/inputs/day16e.txt");
        let result = solution_day_16_01(file_path).unwrap();
        assert_eq!(result, 46);
    }

    #[test]
    fn test_day_16_02() {
        let file_path: String = String::from("src/inputs/day16e.txt");
        let result = solution_day_16_02(file_path).unwrap();
        assert_eq!(result, 51);
    }

    #[test]
    #[ignore]
    fn output_day_16_01() {
        let file_path: String = String::from("src/inputs/day16.txt");
        let result = solution_day_16_01(file_path).unwrap();
        assert_eq!(result, 8098);
    }

    #[test]
    #[ignore]
    fn output_day_16_02() {
        let file_path: String = String::from("src/inputs/day16.txt");
        let result = solution_day_16_02(file_path).unwrap();
        assert_eq!(result, 8335);
    }
}
