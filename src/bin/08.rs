use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(8);

type Int = u32;

/* -------------------------------------------------------------------------- */

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

struct Outcomes<'a> {
    left: &'a str,
    right: &'a str,
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<&str, Outcomes>) {
    let mut lines = input.lines();

    let instructions = lines
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|b| match b {
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect_vec();

    let empty = lines.next();
    debug_assert!(empty.unwrap().is_empty());

    let paths = lines
        .map(|line| {
            let (name, outcomes) = line.split_once('=').unwrap();

            let name = name.trim();

            let (left, right) = outcomes
                .trim()
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(',')
                .unwrap();

            let left = left.trim();
            let right = right.trim();

            (name, Outcomes { left, right })
        })
        .collect::<HashMap<_, _>>();

    (instructions, paths)
}

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<Int> {
    let (instructions, paths) = parse_input(input);

    let mut location = "AAA";
    let mut instructions = instructions.iter().copied().cycle();
    let mut step = 0;
    loop {
        step += 1;
        let path = paths.get(location).unwrap();
        location = match instructions.next().unwrap() {
            Direction::Left => path.left,
            Direction::Right => path.right,
        };
        if location == "ZZZ" {
            break;
        }
    }

    Some(step)
}

/* -------------------------------------------------------------------------- */

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, paths) = parse_input(input);

    let start_locations = paths.keys().filter(|name| name.ends_with('A')).copied();

    let result = start_locations
        .map(|mut location| {
            let mut step = 0;
            let mut instructions = instructions.clone().into_iter().cycle();

            loop {
                let instruction = instructions.next().unwrap();

                step += 1;
                let path = paths.get(location).unwrap();
                location = match instruction {
                    Direction::Left => path.left,
                    Direction::Right => path.right,
                };

                if location.ends_with('Z') {
                    break step;
                }
            }
        })
        .map(|x| x as u64)
        .reduce(num::integer::lcm)
        .unwrap();

    Some(result)
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
