use std::{collections::HashSet, mem};

use advent_of_code::helper::{
    array_2d::Array2D,
    carte::{IPos, Pos},
};

advent_of_code::solution!(21, 1);

type Int = u32;

/* -------------------------------------------------------------------------- */

fn parse_input(input: &str) -> (Array2D<bool>, Pos) {
    let start_pos = input
        .lines()
        .enumerate()
        .find_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .find_map(|(x, b)| (b == b'S').then_some(Pos { x, y }))
        })
        .unwrap();

    let grid = Array2D::from_iter(input.lines().map(|line| line.bytes().map(|b| b != b'#')));

    (grid, start_pos)
}

/* -------------------------------------------------------------------------- */

fn process(grid: &Array2D<bool>, start_pos: Pos, step: usize) -> usize {
    debug_assert!(step % 2 == 0);

    let width = grid.width();
    let height = grid.height();

    let mut cur_positions = vec![start_pos];
    let mut next_positions = Vec::new();

    let mut positions_visited_at_even = HashSet::new();
    positions_visited_at_even.insert(start_pos);

    for _ in 0..(step / 2) {
        for pos in cur_positions.drain(..) {
            for pos in pos.neighbors_clamped(width, height).into_iter().flatten() {
                // odd step
                if !*grid.get(pos.x, pos.y).unwrap() {
                    continue;
                }

                for pos in pos.neighbors_clamped(width, height).into_iter().flatten() {
                    // event step
                    if !*grid.get(pos.x, pos.y).unwrap() {
                        continue;
                    }

                    if positions_visited_at_even.insert(pos) {
                        next_positions.push(pos);
                    }
                }
            }
        }

        mem::swap(&mut cur_positions, &mut next_positions);
    }

    positions_visited_at_even.len()
}

fn do_one(input: &str, step: usize) -> Int {
    let (grid, start_pos) = parse_input(input);
    let result = process(&grid, start_pos, step);
    Int::try_from(result).unwrap()
}

pub fn part_one(input: &str) -> Option<Int> {
    Some(do_one(input, 64))
}

/* -------------------------------------------------------------------------- */

fn do_two(input: &str, step: usize) -> Int {
    let (grid, start_pos) = parse_input(input);

    let width = grid.width();
    let height = grid.height();

    let start_pos = IPos {
        x: isize::try_from(start_pos.x).unwrap(),
        y: isize::try_from(start_pos.y).unwrap(),
    };

    {
        let mut cur_positions = vec![start_pos];
        let mut next_positions = Vec::new();

        let mut positions_visited_at_even = HashSet::new();
        positions_visited_at_even.insert(start_pos);

        for _ in 0..(step / 2) {
            for pos in cur_positions.drain(..) {
                for pos in pos.neighbors().into_iter() {
                    // odd step
                    let p = pos.wrapped(width, height);
                    if !*grid.get(p.x, p.y).unwrap() {
                        continue;
                    }

                    for pos in pos.neighbors().into_iter() {
                        // event step
                        let p = pos.wrapped(width, height);
                        if !*grid.get(p.x, p.y).unwrap() {
                            continue;
                        }

                        if positions_visited_at_even.insert(pos) {
                            next_positions.push(pos);
                        }
                    }
                }
            }

            mem::swap(&mut cur_positions, &mut next_positions);
        }

        Int::try_from(positions_visited_at_even.len()).unwrap()
    }
}

pub fn part_two(input: &str) -> Option<Int> {
    Some(do_two(input, 26_501_365))
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = do_one(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);

        assert_eq!(do_two(&input, 6), 16);
        assert_eq!(do_two(&input, 10), 50);
        assert_eq!(do_two(&input, 50), 1_594);
        assert_eq!(do_two(&input, 100), 6_536);
        assert_eq!(do_two(&input, 500), 167_004);
        assert_eq!(do_two(&input, 1000), 668_697);
        // assert_eq!(do_two(&input, 5000), 16_733_044);
    }
}
