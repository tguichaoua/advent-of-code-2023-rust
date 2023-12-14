use advent_of_code::helper::{array_2d::Array2D, loop_detector};

advent_of_code::solution!(14);

type Int = u32;

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<Int> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let lines = input.lines().map(|line| line.bytes());

    let mut where_to_stop_roll = vec![0; width];
    let mut round_rock_per_row = vec![0; height];

    for (y, line) in lines.enumerate() {
        for (x, data) in line.enumerate() {
            match data {
                b'#' => where_to_stop_roll[x] = y + 1,
                b'O' => {
                    let new_y = where_to_stop_roll[x];
                    round_rock_per_row[new_y] += 1;
                    where_to_stop_roll[x] = new_y + 1;
                }
                b'.' => { /* nothing */ }
                _ => unreachable!(),
            }
        }
    }

    let result = round_rock_per_row
        .into_iter()
        .enumerate()
        .map(|(i, load)| load * (height - i))
        .sum::<usize>();

    Some(Int::try_from(result).unwrap())
}

/* -------------------------------------------------------------------------- */

fn do_cycle(square_rocks: &Array2D<bool>, round_rocks: &mut Array2D<bool>) {
    #![allow(clippy::needless_range_loop)]

    let width = round_rocks.width();
    let height = round_rocks.height();

    // NORTH
    {
        let mut where_to_stop_roll = vec![0; width];
        for y in 0..height {
            for x in 0..width {
                if *square_rocks.get(x, y).unwrap() {
                    where_to_stop_roll[x] = y + 1;
                } else if *round_rocks.get(x, y).unwrap() {
                    let new_y = where_to_stop_roll[x];
                    round_rocks.set(x, y, false);
                    round_rocks.set(x, new_y, true);
                    where_to_stop_roll[x] = new_y + 1;
                }
            }
        }
    }

    // WEST
    {
        let mut where_to_stop_roll = vec![0; height];
        for x in 0..width {
            for y in 0..height {
                if *square_rocks.get(x, y).unwrap() {
                    where_to_stop_roll[y] = x + 1;
                } else if *round_rocks.get(x, y).unwrap() {
                    let new_x = where_to_stop_roll[y];
                    round_rocks.set(x, y, false);
                    round_rocks.set(new_x, y, true);
                    where_to_stop_roll[y] = new_x + 1;
                }
            }
        }
    }

    // SOUTH
    {
        let mut where_to_stop_roll = vec![height - 1; width];
        for y in (0..height).rev() {
            for x in 0..width {
                if *square_rocks.get(x, y).unwrap() {
                    if let Some(y) = y.checked_sub(1) {
                        where_to_stop_roll[x] = y;
                    }
                } else if *round_rocks.get(x, y).unwrap() {
                    let new_y = where_to_stop_roll[x];
                    round_rocks.set(x, y, false);
                    round_rocks.set(x, new_y, true);
                    if let Some(y) = new_y.checked_sub(1) {
                        where_to_stop_roll[x] = y;
                    }
                }
            }
        }
    }

    // EAST
    {
        let mut where_to_stop_roll = vec![width - 1; height];
        for x in (0..width).rev() {
            for y in 0..height {
                if *square_rocks.get(x, y).unwrap() {
                    if let Some(x) = x.checked_sub(1) {
                        where_to_stop_roll[y] = x;
                    }
                } else if *round_rocks.get(x, y).unwrap() {
                    let new_x = where_to_stop_roll[y];
                    round_rocks.set(x, y, false);
                    round_rocks.set(new_x, y, true);
                    if let Some(x) = new_x.checked_sub(1) {
                        where_to_stop_roll[y] = x;
                    }
                }
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<Int> {
    let square_rocks = Array2D::from_iter(
        input
            .lines()
            .map(|line: &str| line.bytes().map(|b| b == b'#')),
    );

    let round_rocks = Array2D::from_iter(
        input
            .lines()
            .map(|line: &str| line.bytes().map(|b| b == b'O')),
    );

    let round_rocks =
        loop_detector::compute_last_state(1_000_000_000, round_rocks, |round_rocks| {
            let mut round_rocks = round_rocks.clone();
            do_cycle(&square_rocks, &mut round_rocks);
            round_rocks
        });

    let height = round_rocks.height();
    let result = round_rocks
        .per_line()
        .enumerate()
        .map(|(i, line)| line.iter().filter(|x| **x).count() * (height - i))
        .sum::<usize>();

    Some(Int::try_from(result).unwrap())
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
