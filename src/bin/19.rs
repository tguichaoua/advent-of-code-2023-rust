use std::{
    collections::{HashMap, VecDeque},
    ops::RangeInclusive,
};

use itertools::Itertools;

advent_of_code::solution!(19);

type Int = u32;

/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Copy)]
enum Category {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Debug, Clone, Copy)]
enum Cmp {
    GreaterThan,
    LessThan,
}

enum Target {
    Workflow(Box<str>),
    Accept,
    Reject,
}

struct Instruction {
    category: Category,
    cmp: Cmp,
    threshold: Int,
    target: Target,
}

struct Workflow {
    instructions: Vec<Instruction>,
    fallback: Target,
}

struct Scrap {
    x: Int,
    m: Int,
    a: Int,
    s: Int,
}

impl Target {
    fn from_str(s: &str) -> Self {
        match s {
            "A" => Target::Accept,
            "R" => Target::Reject,
            target => Target::Workflow(target.into()),
        }
    }
}

impl Workflow {
    fn process(&self, scrap: &Scrap) -> &Target {
        for Instruction {
            category,
            cmp,
            threshold,
            target,
        } in &self.instructions
        {
            let value = scrap.get(*category);
            let pass = match cmp {
                Cmp::GreaterThan => value > *threshold,
                Cmp::LessThan => value < *threshold,
            };
            if pass {
                return target;
            }
        }

        &self.fallback
    }
}

impl Scrap {
    fn get(&self, category: Category) -> Int {
        match category {
            Category::ExtremelyCoolLooking => self.x,
            Category::Musical => self.m,
            Category::Aerodynamic => self.a,
            Category::Shiny => self.s,
        }
    }

    fn total(&self) -> Int {
        self.x + self.m + self.a + self.s
    }
}

fn parse_input(
    input: &str,
) -> (
    HashMap<Box<str>, Workflow>,
    impl Iterator<Item = Scrap> + '_,
) {
    let mut lines = input.lines();

    let workflows = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (name, rest) = line.split_once('{').unwrap();
            let mut rest = rest.strip_suffix('}').unwrap().split(',');

            let fallback = rest.next_back().unwrap();
            debug_assert!(!fallback.contains(['>', '<', ':']));
            let fallback = Target::from_str(fallback);

            let instructions = rest
                .map(|instruction| {
                    let category = match instruction.as_bytes()[0] {
                        b'x' => Category::ExtremelyCoolLooking,
                        b'm' => Category::Musical,
                        b'a' => Category::Aerodynamic,
                        b's' => Category::Shiny,
                        _ => unreachable!(),
                    };

                    let cmp = match instruction.as_bytes()[1] {
                        b'<' => Cmp::LessThan,
                        b'>' => Cmp::GreaterThan,
                        _ => unreachable!(),
                    };

                    let (threshold, target) = instruction[2..].split_once(':').unwrap();

                    let threshold = threshold.parse().unwrap();
                    let target = Target::from_str(target);

                    Instruction {
                        category,
                        cmp,
                        threshold,
                        target,
                    }
                })
                .collect_vec();

            (
                Box::<str>::from(name),
                Workflow {
                    fallback,
                    instructions,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let scraps = lines.map(|line| {
        let line = line.strip_prefix('{').unwrap();
        let line = line.strip_suffix('}').unwrap();

        // Let's just assume values are always ordered x, m, a, s
        let [x, m, a, s] = line
            .splitn(4, ',')
            .map(|item| {
                let (_, value) = item.split_once('=').unwrap();
                value.parse::<Int>().unwrap()
            })
            .collect_vec()
            .try_into()
            .unwrap();

        Scrap { x, m, a, s }
    });

    (workflows, scraps)
}

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<Int> {
    let (workflows, scraps) = parse_input(input);

    let result = scraps
        .map(|scrap| {
            let mut current_workflow = workflows.get("in").unwrap();
            loop {
                match current_workflow.process(&scrap) {
                    Target::Accept => break scrap.total(),
                    Target::Reject => break 0,
                    Target::Workflow(next_workflow) => {
                        current_workflow = workflows.get(next_workflow).unwrap()
                    }
                }
            }
        })
        .sum();

    Some(result)
}

/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone)]
struct ScrapRange {
    x: RangeInclusive<Int>,
    m: RangeInclusive<Int>,
    a: RangeInclusive<Int>,
    s: RangeInclusive<Int>,
}

impl ScrapRange {
    fn get(&self, category: Category) -> RangeInclusive<Int> {
        match category {
            Category::ExtremelyCoolLooking => self.x.clone(),
            Category::Musical => self.m.clone(),
            Category::Aerodynamic => self.a.clone(),
            Category::Shiny => self.s.clone(),
        }
    }

    fn set(&mut self, category: Category, range: RangeInclusive<Int>) {
        match category {
            Category::ExtremelyCoolLooking => self.x = range,
            Category::Musical => self.m = range,
            Category::Aerodynamic => self.a = range,
            Category::Shiny => self.s = range,
        }
    }

    fn combination(self) -> u64 {
        self.x.count() as u64
            * self.m.count() as u64
            * self.a.count() as u64
            * self.s.count() as u64
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (workflows, _) = parse_input(input);

    let mut queue = VecDeque::new();
    queue.push_back((
        workflows.get("in").unwrap(),
        ScrapRange {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        },
    ));

    let mut accepted = 0u64;

    'queue: while let Some((workflow, mut scrap)) = queue.pop_front() {
        for Instruction {
            category,
            cmp,
            threshold,
            target,
        } in &workflow.instructions
        {
            let range = scrap.get(*category);

            let (pass_range, next_range) = match cmp {
                Cmp::GreaterThan => {
                    if range.start() > threshold {
                        (Some(range), None)
                    } else if range.end() <= threshold {
                        (None, Some(range))
                    } else {
                        let pass_range = Some((threshold + 1)..=(*range.end()));
                        let next_range = Some((*range.start())..=(*threshold));
                        (pass_range, next_range)
                    }
                }
                Cmp::LessThan => {
                    if range.start() >= threshold {
                        (None, Some(range))
                    } else if range.end() < threshold {
                        (Some(range), None)
                    } else {
                        let pass_range = Some((*range.start())..=(threshold - 1));
                        let next_range = Some((*threshold)..=(*range.end()));
                        (pass_range, next_range)
                    }
                }
            };

            if let Some(pass_range) = pass_range {
                debug_assert!(!pass_range.is_empty());

                let mut scrap = scrap.clone();
                scrap.set(*category, pass_range);

                match target {
                    Target::Workflow(name) => {
                        queue.push_back((workflows.get(name).unwrap(), scrap))
                    }
                    Target::Accept => accepted += scrap.combination(),
                    Target::Reject => { /* nothing */ }
                }
            }

            let Some(next_range) = next_range else {
                continue 'queue;
            };
            debug_assert!(!next_range.is_empty());

            scrap.set(*category, next_range);
        }

        match &workflow.fallback {
            Target::Workflow(name) => queue.push_back((workflows.get(name).unwrap(), scrap)),
            Target::Accept => accepted += scrap.combination(),
            Target::Reject => { /* nothing */ }
        }
    }

    Some(accepted)
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167_409_079_868_000));
    }
}
