use std::collections::HashSet;

use advent_of_code::helper::{carte::Pos, iterator::unique_pair};
use itertools::Itertools;

advent_of_code::solution!(11);

type Int = u64;

/* -------------------------------------------------------------------------- */

fn solve(input: &str, expansion_factor: usize) -> Int {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let lines = input.lines().map(|line| line.bytes().map(|b| b == b'#'));

    let galaxies = lines
        .enumerate()
        .flat_map(|(y, line)| {
            line.enumerate()
                .filter_map(move |(x, is_galaxy)| is_galaxy.then_some(Pos { x, y }))
        })
        .collect_vec();

    let empty_rows = {
        let row_with_galaxy = galaxies.iter().map(|pos| pos.y).collect::<HashSet<_>>();
        (0..height)
            .collect::<HashSet<_>>()
            .difference(&row_with_galaxy)
            .copied()
            .sorted()
            .collect_vec()
    };
    let empty_cols = {
        let col_with_galaxy = galaxies.iter().map(|pos| pos.x).collect::<HashSet<_>>();
        (0..width)
            .collect::<HashSet<_>>()
            .difference(&col_with_galaxy)
            .copied()
            .sorted()
            .collect_vec()
    };

    let actual_galaxy_positions = galaxies.into_iter().map(|pos| {
        let Pos { x, y } = pos;
        let dx = empty_cols.binary_search(&x).unwrap_err() * (expansion_factor - 1);
        let dy = empty_rows.binary_search(&y).unwrap_err() * (expansion_factor - 1);
        Pos {
            x: x + dx,
            y: y + dy,
        }
    });

    let result = unique_pair(actual_galaxy_positions)
        .map(|(a, b)| {
            let dx = a.x.abs_diff(b.x);
            let dy = a.y.abs_diff(b.y);
            dx + dy
        })
        .sum::<usize>();

    result as Int
}

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<Int> {
    Some(solve(input, 2))
}

/* -------------------------------------------------------------------------- */

pub fn part_two(input: &str) -> Option<Int> {
    Some(solve(input, 1_000_000))
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let input = &advent_of_code::template::read_file("examples", DAY);

        assert_eq!(solve(input, 10), 1030);
        assert_eq!(solve(input, 100), 8410);
    }
}
