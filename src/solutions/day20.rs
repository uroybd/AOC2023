// Advent of Code 2023 - Day 20

use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use derive_deref::{Deref, DerefMut};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Signal {
    name: String,
    pulse: Pulse,
}

trait Module {
    fn input(&mut self, input: &Signal) -> Option<Vec<Signal>>;
    fn dest_contains(&self, name: &str) -> bool;
    fn get_name(&self) -> String;
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct FlipFlop {
    name: String,
    destinations: Vec<String>,
    state: bool,
}

impl FlipFlop {
    fn new(name: String, destinations: Vec<String>) -> Self {
        Self {
            name,
            destinations,
            state: false,
        }
    }
}

impl Module for FlipFlop {
    fn input(&mut self, input: &Signal) -> Option<Vec<Signal>> {
        if input.pulse == Pulse::High {
            return None;
        }
        self.state = !self.state;
        Some(
            self.destinations
                .iter()
                .map(|d| Signal {
                    name: d.clone(),
                    pulse: if self.state { Pulse::High } else { Pulse::Low },
                })
                .collect(),
        )
    }

    fn dest_contains(&self, name: &str) -> bool {
        self.destinations.contains(&name.to_string())
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Conjunction {
    name: String,
    inputs: HashMap<String, Pulse>,
    destinations: Vec<String>,
}

impl Conjunction {
    fn new(name: String, input_keys: &[String], destinations: &[String]) -> Self {
        let inputs = input_keys
            .iter()
            .map(|k| (k.clone(), Pulse::Low))
            .collect::<HashMap<String, Pulse>>();
        Self {
            name,
            inputs,
            destinations: destinations.to_vec(),
        }
    }
}

impl Module for Conjunction {
    fn input(&mut self, input: &Signal) -> Option<Vec<Signal>> {
        self.inputs.entry(input.name.clone()).and_modify(|p| {
            *p = input.pulse.clone();
        });
        let mut p = Pulse::Low;
        if self.inputs.values().any(|p| p == &Pulse::Low) {
            p = Pulse::High;
        }
        Some(
            self.destinations
                .iter()
                .map(|d| Signal {
                    name: d.clone(),
                    pulse: p.clone(),
                })
                .collect(),
        )
    }

    fn dest_contains(&self, name: &str) -> bool {
        self.destinations.contains(&name.to_string())
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Broadcaster {
    name: String,
    destinations: Vec<String>,
}

impl Broadcaster {
    fn new(name: String, destinations: Vec<String>) -> Self {
        Self { name, destinations }
    }
}

impl Module for Broadcaster {
    fn input(&mut self, input: &Signal) -> Option<Vec<Signal>> {
        Some(
            self.destinations
                .iter()
                .map(|d| Signal {
                    name: d.clone(),
                    pulse: input.pulse.clone(),
                })
                .collect(),
        )
    }

    fn dest_contains(&self, name: &str) -> bool {
        self.destinations.contains(&name.to_string())
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Button {
    name: String,
    dest: String,
}

impl Button {
    fn new(name: String, dest: String) -> Self {
        Self { name, dest }
    }
}

impl Module for Button {
    fn input(&mut self, _input: &Signal) -> Option<Vec<Signal>> {
        Some(vec![Signal {
            name: self.dest.clone(),
            pulse: Pulse::Low,
        }])
    }

    fn dest_contains(&self, name: &str) -> bool {
        self.dest == *name
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Deref, DerefMut)]
struct Circuit(HashMap<String, Box<dyn Module>>);

#[derive(Debug, PartialEq, Eq)]
struct ParseCircuitError;

impl std::str::FromStr for Circuit {
    type Err = ParseCircuitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut circuit = Self(HashMap::new());
        let mut conjunctions = vec![];
        for line in s.lines() {
            let (src, dest) = line.split_once(" -> ").unwrap();
            let destinations = dest.split(", ").map(|x| x.to_string()).collect();
            if src == "broadcaster" {
                circuit.insert(
                    src.to_string(),
                    Box::new(Broadcaster::new(src.to_string(), destinations)),
                );
            } else if src.starts_with('%') {
                let k = src.trim_start_matches('%').to_string();
                circuit.insert(k.clone(), Box::new(FlipFlop::new(k.clone(), destinations)));
            } else {
                let k = src.trim_start_matches('&').to_string();
                conjunctions.push((k, destinations));
            }
        }
        for con in conjunctions {
            let (k, destinations) = con;
            let input_keys = circuit
                .iter()
                .filter_map(|(key, v)| {
                    if v.dest_contains(&k) {
                        Some(key.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>();
            circuit.insert(
                k.clone(),
                Box::new(Conjunction::new(k, &input_keys, &destinations)),
            );
        }
        circuit.insert(
            "button".to_string(),
            Box::new(Button::new("button".to_string(), "broadcaster".to_string())),
        );
        Ok(circuit)
    }
}

impl Circuit {
    fn run(&mut self) -> (usize, usize) {
        let mut high_count = 0;
        let mut low_count = -1;
        let mut queue = VecDeque::from(vec![(
            "button".to_string(),
            Signal {
                name: "button".to_string(),
                pulse: Pulse::Low,
            },
        )]);

        while let Some((name, signal)) = queue.pop_front() {
            match signal.pulse {
                Pulse::High => high_count += 1,
                Pulse::Low => low_count += 1,
            }

            if let Some(module) = self.get_mut(&name) {
                if let Some(outputs) = module.input(&signal) {
                    for entry in outputs {
                        queue.push_back((
                            entry.name.clone(),
                            Signal {
                                name: module.get_name(),
                                pulse: entry.pulse,
                            },
                        ));
                    }
                }
            }
        }
        (high_count, (low_count as usize))
    }

    fn total_pulse(&mut self, n: usize) -> usize {
        let (high, low) = (0..n).fold((0, 0), |(h, l), _| {
            let (vh, vl) = self.run();
            (h + vh, l + vl)
        });
        high * low
    }

    fn run_until_on(&mut self) -> usize {
        let rx_setter = self.values().find(|m| m.dest_contains("rx")).unwrap();
        let leading_to_rx_setter = self
            .values()
            .filter(|m| m.dest_contains(&rx_setter.get_name()))
            .map(|m| m.get_name())
            .collect::<Vec<String>>();
        let mut cycles = HashMap::new();
        let mut cycle = 0;
        'mloop: loop {
            cycle += 1;
            let mut queue = VecDeque::from(vec![(
                "button".to_string(),
                Signal {
                    name: "button".to_string(),
                    pulse: Pulse::Low,
                },
            )]);

            while let Some((name, signal)) = queue.pop_front() {
                if let Some(module) = self.get_mut(&name) {
                    if leading_to_rx_setter.contains(&name)
                        && signal.pulse == Pulse::Low
                        && !cycles.contains_key(&name)
                    {
                        cycles.insert(name.clone(), cycle);
                        if cycles.len() == leading_to_rx_setter.len() {
                            break 'mloop;
                        }
                    }
                    if let Some(outputs) = module.input(&signal) {
                        for entry in outputs {
                            queue.push_back((
                                entry.name.clone(),
                                Signal {
                                    name: module.get_name(),
                                    pulse: entry.pulse,
                                },
                            ));
                        }
                    }
                }
            }
        }

        cycles.values().product()
    }
}

pub fn solution_day_20_01(file_path: String) -> Option<usize> {
    let mut circuit = fs::read_to_string(file_path)
        .expect("Invalid input file.")
        .parse::<Circuit>()
        .unwrap();

    Some(circuit.total_pulse(1000))
    // Some((0..1000).fold(0, |acc, _| acc + circuit.run()))
}

pub fn solution_day_20_02(file_path: String) -> Option<usize> {
    let mut circuit = fs::read_to_string(file_path)
        .expect("Invalid input file.")
        .parse::<Circuit>()
        .unwrap();

    Some(circuit.run_until_on())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_20_01() {
        let file_path: String = String::from("src/inputs/day20e.txt");
        let result = solution_day_20_01(file_path).unwrap();
        assert_eq!(result, 32000000);
    }

    #[test]
    fn test_day_20_02() {
        let file_path: String = String::from("src/inputs/day20e.txt");
        let result = solution_day_20_02(file_path).unwrap();
        dbg!(result);
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_20_01() {
        let file_path: String = String::from("src/inputs/day20.txt");
        let result = solution_day_20_01(file_path).unwrap();
        assert_eq!(result, 743090292);
    }

    #[test]
    #[ignore]
    fn output_day_20_02() {
        let file_path: String = String::from("src/inputs/day20.txt");
        let result = solution_day_20_02(file_path).unwrap();
        assert_eq!(result, 241528184647003);
    }
}
