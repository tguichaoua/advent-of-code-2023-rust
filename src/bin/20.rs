use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(20, 1);

type Int = u32;

/* -------------------------------------------------------------------------- */

struct Module {
    destinations: Box<[Box<str>]>,
    kind: ModuleKind,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ModuleKind {
    Broadcaster,
    FlipFlop { state: bool },
    Conjunction { memory: Vec<(Box<str>, Pulse)> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Pulse {
    Low,
    High,
}

/* -------------------------------------------------------------------------- */

fn parse_input(input: &str) -> HashMap<Box<str>, Module> {
    let mut input_connections: HashMap<Box<str>, Vec<Box<str>>> = HashMap::new();

    let mut modules = input
        .lines()
        .map(|line| {
            let (name, destinations) = line.split_once(" -> ").unwrap();

            let destinations = destinations
                .split(", ")
                .map(|s| s.to_owned().into_boxed_str())
                .collect_vec()
                .into_boxed_slice();

            let (name, kind) = if let Some(name) = name.strip_prefix('%') {
                let name = name.to_owned().into_boxed_str();
                let kind = ModuleKind::FlipFlop { state: false };
                (name, kind)
            } else if let Some(name) = name.strip_prefix('&') {
                let name = name.to_owned().into_boxed_str();
                let kind = ModuleKind::Conjunction { memory: Vec::new() };
                (name, kind)
            } else if name == "broadcaster" {
                let name = name.to_owned().into_boxed_str();
                let kind = ModuleKind::Broadcaster;
                (name, kind)
            } else {
                unreachable!();
            };

            for destination in destinations.iter() {
                input_connections
                    .entry(destination.clone())
                    .or_default()
                    .push(name.clone());
            }

            (name, Module { destinations, kind })
        })
        .collect::<HashMap<_, _>>();

    for (name, module) in &mut modules {
        if let ModuleKind::Conjunction { memory } = &mut module.kind {
            *memory = input_connections
                .remove(name)
                .unwrap()
                .into_iter()
                .map(|name| (name, Pulse::Low))
                .collect();
        }
    }

    modules
}

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<Int> {
    let mut modules = parse_input(input);

    let mut queue = VecDeque::new();

    let mut low_pulse = 0;
    let mut high_pulse = 0;

    for _ in 0..1000 {
        // push the button
        queue.push_back((
            "button".to_owned().into_boxed_str(),
            Pulse::Low,
            "broadcaster".to_owned().into_boxed_str(),
        ));
        low_pulse += 1;

        while let Some((from, pulse, to)) = queue.pop_front() {
            let Some(Module { destinations, kind }) = modules.get_mut(&to) else {
                continue;
            };

            match kind {
                ModuleKind::Broadcaster => {
                    match pulse {
                        Pulse::Low => low_pulse += destinations.len(),
                        Pulse::High => high_pulse += destinations.len(),
                    }

                    for destination in destinations.iter() {
                        queue.push_back((to.clone(), pulse, destination.clone()));
                    }
                }
                ModuleKind::FlipFlop { state } => {
                    if pulse == Pulse::Low {
                        if *state {
                            *state = false;
                            low_pulse += destinations.len();
                            for destination in destinations.iter() {
                                queue.push_back((to.clone(), Pulse::Low, destination.clone()));
                            }
                        } else {
                            *state = true;
                            high_pulse += destinations.len();
                            for destination in destinations.iter() {
                                queue.push_back((to.clone(), Pulse::High, destination.clone()));
                            }
                        }
                    }
                }
                ModuleKind::Conjunction { memory } => {
                    let (_, memorised_pulse) =
                        memory.iter_mut().find(|(name, _)| name == &from).unwrap();
                    *memorised_pulse = pulse;

                    let pulse = if memory.iter().all(|(_, pulse)| *pulse == Pulse::High) {
                        low_pulse += destinations.len();
                        Pulse::Low
                    } else {
                        high_pulse += destinations.len();
                        Pulse::High
                    };

                    for destination in destinations.iter() {
                        queue.push_back((to.clone(), pulse, destination.clone()));
                    }
                }
            }
        }
    }

    let result = low_pulse * high_pulse;

    Some(Int::try_from(result).unwrap())
}

/* -------------------------------------------------------------------------- */

pub fn part_two(input: &str) -> Option<Int> {
    let mut modules = parse_input(input);

    let mut queue = VecDeque::new();

    let mut button_pressed_count = 0;

    'outer: loop {
        // push the button
        button_pressed_count += 1;
        queue.push_back((
            "button".to_owned().into_boxed_str(),
            Pulse::Low,
            "broadcaster".to_owned().into_boxed_str(),
        ));

        while let Some((from, pulse, to)) = queue.pop_front() {
            if &*to == "rx" && pulse == Pulse::Low {
                break 'outer;
            }

            let Some(Module { destinations, kind }) = modules.get_mut(&to) else {
                continue;
            };

            match kind {
                ModuleKind::Broadcaster => {
                    for destination in destinations.iter() {
                        queue.push_back((to.clone(), pulse, destination.clone()));
                    }
                }
                ModuleKind::FlipFlop { state } => {
                    if pulse == Pulse::Low {
                        if *state {
                            *state = false;

                            for destination in destinations.iter() {
                                queue.push_back((to.clone(), Pulse::Low, destination.clone()));
                            }
                        } else {
                            *state = true;

                            for destination in destinations.iter() {
                                queue.push_back((to.clone(), Pulse::High, destination.clone()));
                            }
                        }
                    }
                }
                ModuleKind::Conjunction { memory } => {
                    let (_, memorised_pulse) =
                        memory.iter_mut().find(|(name, _)| name == &from).unwrap();
                    *memorised_pulse = pulse;

                    let pulse = if memory.iter().all(|(_, pulse)| *pulse == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };

                    for destination in destinations.iter() {
                        queue.push_back((to.clone(), pulse, destination.clone()));
                    }
                }
            }
        }
    }

    Some(button_pressed_count)
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 10,
        ));
        assert_eq!(result, Some(32_000_000));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 11,
        ));
        assert_eq!(result, Some(11_687_500));
    }
}
