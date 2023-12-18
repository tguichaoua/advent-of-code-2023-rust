use std::collections::{HashSet, VecDeque};

use advent_of_code::helper::carte::Direction;

advent_of_code::solution!(18, 1);

/* -------------------------------------------------------------------------- */

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = input.lines().map(|line| {
        let direction = match line.as_bytes()[0] {
            b'U' => Direction::Up,
            b'R' => Direction::Right,
            b'D' => Direction::Down,
            b'L' => Direction::Left,
            _ => unreachable!(),
        };

        let (amount, _) = line[2..].split_once(' ').unwrap();
        let amount: u16 = amount.parse().unwrap();

        (direction, i32::from(amount))
    });

    let mut x_min = 0;
    let mut x_max = 0;

    let mut y_min = 0;
    let mut y_max = 0;

    let mut x = 0;
    let mut y = 0;

    let mut edges = HashSet::new();
    edges.insert((x, y));

    for (direction, amount) in instructions {
        match direction {
            Direction::Up => {
                for dy in 1..=amount {
                    let pos = (x, y - dy);
                    edges.insert(pos);
                }
                y -= amount;
                y_min = y_min.min(y);
            }
            Direction::Down => {
                for dy in 1..=amount {
                    let pos = (x, y + dy);
                    edges.insert(pos);
                }
                y += amount;
                y_max = y_max.max(y);
            }
            Direction::Right => {
                for dx in 1..=amount {
                    let pos = (x + dx, y);
                    edges.insert(pos);
                }
                x += amount;
                x_max = x_max.max(x);
            }

            Direction::Left => {
                for dx in 1..=amount {
                    let pos = (x - dx, y);
                    edges.insert(pos);
                }
                x -= amount;
                x_min = x_min.min(x);
            }
        }
    }

    let mut outsides = HashSet::new();

    let mut queue = {
        let border_positions = (x_min..=x_max)
            .flat_map(|x| [(x, y_min), (x, y_max)])
            .chain((y_min..=y_max).flat_map(|y| [(x_min, y), (x_max, y)]));

        border_positions.collect::<VecDeque<_>>()
    };

    while let Some(pos) = queue.pop_front() {
        if edges.contains(&pos) {
            // This is the edge
            continue;
        }

        if !outsides.insert(pos) {
            // This pos has already been explored
            continue;
        }

        let (x, y) = pos;

        // UP
        {
            let y = y - 1;
            if y >= y_min {
                queue.push_back((x, y));
            }
        };
        // DOWN
        {
            let y = y + 1;
            if y <= y_max {
                queue.push_back((x, y));
            }
        };
        // LEFT
        {
            let x = x - 1;
            if x >= x_min {
                queue.push_back((x, y));
            }
        };
        // RIGHT
        {
            let x = x + 1;
            if x <= x_max {
                queue.push_back((x, y));
            }
        };
    }

    let full_area = u32::try_from((x_max - x_min + 1) * (y_max - y_min + 1)).unwrap();

    let result = full_area - u32::try_from(outsides.len()).unwrap();

    Some(result)
}

/* -------------------------------------------------------------------------- */

pub fn part_two(input: &str) -> Option<u64> {
    let instructions = input.lines().map(|line| {
        let (_, code) = line.split_once('#').unwrap();
        let code = code.strip_suffix(')').unwrap();

        let amount = u32::from_str_radix(&code[0..5], 16).unwrap();
        let direction = match code.as_bytes()[5] {
            b'0' => Direction::Right,
            b'1' => Direction::Down,
            b'2' => Direction::Left,
            b'3' => Direction::Up,
            _ => unreachable!(),
        };

        (direction, i64::from(amount))
    });

    let mut x_min = 0;
    let mut x_max = 0;

    let mut y_min = 0;
    let mut y_max = 0;

    let mut x = 0i64;
    let mut y = 0i64;

    let mut edges = HashSet::new();
    edges.insert((x, y));

    for (direction, amount) in instructions {
        match direction {
            Direction::Up => {
                for dy in 1..=amount {
                    let pos = (x, y - dy);
                    edges.insert(pos);
                }
                y -= amount;
                y_min = y_min.min(y);
            }
            Direction::Down => {
                for dy in 1..=amount {
                    let pos = (x, y + dy);
                    edges.insert(pos);
                }
                y += amount;
                y_max = y_max.max(y);
            }
            Direction::Right => {
                for dx in 1..=amount {
                    let pos = (x + dx, y);
                    edges.insert(pos);
                }
                x += amount;
                x_max = x_max.max(x);
            }

            Direction::Left => {
                for dx in 1..=amount {
                    let pos = (x - dx, y);
                    edges.insert(pos);
                }
                x -= amount;
                x_min = x_min.min(x);
            }
        }
    }

    let mut outsides = HashSet::new();

    let border_positions = (0..=x_max)
        .flat_map(|x| [(x, 0), (x, y_max)])
        .chain((0..=y_max).flat_map(|y| [(0, y), (x_max, y)]));

    let mut queue = VecDeque::new();

    for pos in border_positions {
        if edges.contains(&pos) {
            // This is the edge
            continue;
        }

        if !outsides.insert(pos) {
            // This pos has already been explored
            continue;
        }

        queue.push_back(pos);

        while let Some((x, y)) = queue.pop_front() {
            // UP
            {
                let y = y - 1;
                if y >= y_min {
                    let pos = (x, y);
                    if !edges.contains(&pos) && outsides.insert(pos) {
                        queue.push_back((x, y));
                    }
                }
            };
            // DOWN
            {
                let y = y + 1;
                if y <= y_max {
                    let pos = (x, y);
                    if !edges.contains(&pos) && outsides.insert(pos) {
                        queue.push_back((x, y));
                    }
                }
            };
            // LEFT
            {
                let x = x - 1;
                if x >= x_min {
                    let pos = (x, y);
                    if !edges.contains(&pos) && outsides.insert(pos) {
                        queue.push_back((x, y));
                    }
                }
            };
            // RIGHT
            {
                let x = x + 1;
                if x <= x_max {
                    let pos = (x, y);
                    if !edges.contains(&pos) && outsides.insert(pos) {
                        queue.push_back((x, y));
                    }
                }
            };
        }
    }

    let full_area = u64::try_from((x_max - x_min + 1) * (y_max - y_min + 1)).unwrap();

    let result = full_area - u64::try_from(outsides.len()).unwrap();

    Some(result)
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(952_408_144_115));
    // }
}
