use std::iter;

use itertools::Itertools;

advent_of_code::solution!(12, 1);

/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone)]
struct Universe<'a> {
    future_fix_dashes: usize,
    expected_groups: &'a [usize],
    current_group_len: usize,
}

enum FutureUniverse<'a> {
    NotFix(Universe<'a>),
    Fix,
    Invalid,
}

impl<'a> Universe<'a> {
    fn push_dot(mut self) -> FutureUniverse<'a> {
        if self.current_group_len == 0 {
            FutureUniverse::NotFix(self)
        } else {
            let is_valid = self.expected_groups[0] == self.current_group_len;
            if !is_valid {
                return FutureUniverse::Invalid;
            }

            self.expected_groups = &self.expected_groups[1..];
            self.current_group_len = 0;

            if self.expected_groups.is_empty() {
                if self.future_fix_dashes == 0 {
                    FutureUniverse::Fix
                } else {
                    FutureUniverse::Invalid
                }
            } else {
                FutureUniverse::NotFix(self)
            }
        }
    }

    fn push_dash(mut self) -> FutureUniverse<'a> {
        let expected = self.expected_groups[0];
        self.current_group_len += 1;
        if self.current_group_len > expected {
            FutureUniverse::Invalid
        } else {
            FutureUniverse::NotFix(self)
        }
    }

    fn push_fixed_dash(self) -> FutureUniverse<'a> {
        let this = self.push_dash();
        if let FutureUniverse::NotFix(mut this) = this {
            this.future_fix_dashes -= 1;
            FutureUniverse::NotFix(this)
        } else {
            this
        }
    }

    fn push_unknown(self) -> [FutureUniverse<'a>; 2] {
        let other = self.clone();

        let this = self.push_dot();
        let other = other.push_dash();

        [this, other]
    }
}

fn computes(
    fixed_dashes: usize,
    instructions: impl Iterator<Item = u8>,
    expected_groups: &[usize],
) -> usize {
    let mut universes = vec![Universe {
        future_fix_dashes: fixed_dashes,
        expected_groups,
        current_group_len: 0,
    }];

    let mut fix_count = 0;

    for inst in instructions {
        match inst {
            b'.' => {
                universes = universes
                    .drain(..)
                    .map(|universe| universe.push_dot())
                    .flat_map(|future| match future {
                        FutureUniverse::NotFix(universe) => Some(universe),
                        FutureUniverse::Fix => {
                            fix_count += 1;
                            None
                        }
                        FutureUniverse::Invalid => None,
                    })
                    .collect();
            }
            b'#' => {
                universes = universes
                    .drain(..)
                    .map(|universe| universe.push_fixed_dash())
                    .flat_map(|future| match future {
                        FutureUniverse::NotFix(universe) => Some(universe),
                        FutureUniverse::Fix => {
                            fix_count += 1;
                            None
                        }
                        FutureUniverse::Invalid => None,
                    })
                    .collect();
            }
            b'?' => {
                universes = universes
                    .drain(..)
                    .flat_map(|universe| universe.push_unknown())
                    .flat_map(|future| match future {
                        FutureUniverse::NotFix(universe) => Some(universe),
                        FutureUniverse::Fix => {
                            fix_count += 1;
                            None
                        }
                        FutureUniverse::Invalid => None,
                    })
                    .collect();
            }
            _ => unreachable!(),
        }
    }

    universes
        .into_iter()
        .filter(|u| u.expected_groups.is_empty())
        .count()
        + fix_count
}

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<usize> {
    let result = input
        .lines()
        .map(|line| {
            let (data, expected_groups) = line.split_once(' ').unwrap();

            let expected_groups = expected_groups
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec();

            let instructions = data.bytes().chain(iter::once(b'.'));
            let fixed_dashes = instructions.clone().filter(|&b| b == b'#').count();

            computes(fixed_dashes, instructions, &expected_groups)
        })
        .sum::<usize>();

    Some(result)
}

/* -------------------------------------------------------------------------- */

pub fn part_two(input: &str) -> Option<usize> {
    let result = input
        .lines()
        .map(|line| {
            let (data, expected_groups) = line.split_once(' ').unwrap();

            if data.ends_with('#') {
                let instructions = data.bytes().chain(iter::once(b'.'));

                let fixed_dashes = instructions.clone().filter(|&b| b == b'#').count();

                let expected_groups = expected_groups
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_vec();

                let n = computes(fixed_dashes, instructions, &expected_groups);

                return n.pow(5);
            }

            let expected_groups = {
                let v = expected_groups
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_vec();

                iter::repeat(v.into_iter()).take(5).flatten().collect_vec()
            };

            let instructions = iter::repeat(data.bytes())
                .take(5)
                .intersperse("?".bytes())
                .flatten()
                .chain(iter::once(b'.'));

            let fixed_dashes = instructions.clone().filter(|&b| b == b'#').count();

            computes(fixed_dashes, instructions, &expected_groups)
        })
        .sum::<usize>();

    Some(result)
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
