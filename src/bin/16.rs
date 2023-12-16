use std::collections::HashSet;

use advent_of_code::helper::{
    array_2d::Array2D,
    carte::{Direction, Pos},
};

advent_of_code::solution!(16);

/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    position: Pos,
    direction: Direction,
}

fn parse_input(input: &str) -> Array2D<u8> {
    Array2D::from_iter(input.lines().map(|line| line.bytes()))
}

fn process(grid: &Array2D<u8>, start: Beam) -> usize {
    let mut beams = vec![start];

    let mut energized = HashSet::new();
    energized.insert(start);

    while !beams.is_empty() {
        beams = beams
            .drain(..)
            .flat_map(
                |Beam {
                     position,
                     direction,
                 }| {
                    let new_beam = |direction| {
                        position.move_to(direction).map(|position| Beam {
                            position,
                            direction,
                        })
                    };

                    let mirror = grid.get(position.x, position.y).copied().unwrap();

                    match mirror {
                        b'|' => match direction {
                            Direction::Up | Direction::Down => [new_beam(direction), None],
                            Direction::Right | Direction::Left => {
                                [new_beam(Direction::Up), new_beam(Direction::Down)]
                            }
                        },
                        b'-' => match direction {
                            Direction::Right | Direction::Left => [new_beam(direction), None],
                            Direction::Up | Direction::Down => {
                                [new_beam(Direction::Left), new_beam(Direction::Right)]
                            }
                        },
                        b'\\' => match direction {
                            Direction::Right => [new_beam(Direction::Down), None],
                            Direction::Left => [new_beam(Direction::Up), None],
                            Direction::Up => [new_beam(Direction::Left), None],
                            Direction::Down => [new_beam(Direction::Right), None],
                        },
                        b'/' => match direction {
                            Direction::Right => [new_beam(Direction::Up), None],
                            Direction::Left => [new_beam(Direction::Down), None],
                            Direction::Up => [new_beam(Direction::Right), None],
                            Direction::Down => [new_beam(Direction::Left), None],
                        },
                        b'.' => [new_beam(direction), None],
                        _ => unreachable!(),
                    }
                },
            )
            .flatten()
            .filter(|beam| {
                grid.get(beam.position.x, beam.position.y).is_ok() && energized.insert(*beam)
            })
            .collect();
    }

    energized
        .into_iter()
        .map(|beam| beam.position)
        .collect::<HashSet<_>>()
        .len()
}

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input);

    let result = process(
        &grid,
        Beam {
            position: Pos { x: 0, y: 0 },
            direction: Direction::Right,
        },
    );

    Some(result)
}

/* -------------------------------------------------------------------------- */

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_input(input);

    let width = grid.width();
    let height = grid.height();

    let result = (0..width)
        .flat_map(|x| {
            [
                process(
                    &grid,
                    Beam {
                        position: Pos { x, y: 0 },
                        direction: Direction::Down,
                    },
                ),
                process(
                    &grid,
                    Beam {
                        position: Pos { x, y: height - 1 },
                        direction: Direction::Up,
                    },
                ),
            ]
        })
        .chain((0..height).flat_map(|y| {
            [
                process(
                    &grid,
                    Beam {
                        position: Pos { x: 0, y },
                        direction: Direction::Right,
                    },
                ),
                process(
                    &grid,
                    Beam {
                        position: Pos { x: width - 1, y },
                        direction: Direction::Left,
                    },
                ),
            ]
        }))
        .max()
        .unwrap();

    Some(result)
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
