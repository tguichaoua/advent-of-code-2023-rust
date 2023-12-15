use std::array;

advent_of_code::solution!(15);

type Int = u32;

/* -------------------------------------------------------------------------- */

fn holiday_ascii_string_helper(s: &str) -> Int {
    let mut x = 0;

    for c in s.bytes() {
        x += Int::from(c);
        x *= 17;
        x %= 256;
    }

    x
}

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<Int> {
    let result = input
        .trim_end()
        .split(',')
        .map(holiday_ascii_string_helper)
        .sum();
    Some(result)
}

/* -------------------------------------------------------------------------- */

pub fn part_two(input: &str) -> Option<usize> {
    let instructions = input.trim_end().split(',').map(|inst| {
        if let Some((label, focal)) = inst.split_once('=') {
            (
                holiday_ascii_string_helper(label),
                label,
                Some(focal.parse::<Int>().unwrap()),
            )
        } else if let Some((label, empty)) = inst.split_once('-') {
            debug_assert!(empty.is_empty());
            (holiday_ascii_string_helper(label), label, None)
        } else {
            unreachable!();
        }
    });

    let mut boxes: [_; 256] = array::from_fn(|_| Vec::new());

    for (box_, label, focal) in instructions {
        let box_ = &mut boxes[box_ as usize];
        if let Some(focal) = focal {
            if let Some((_, existing_focal)) = box_.iter_mut().find(|(x, _)| *x == label) {
                *existing_focal = focal;
            } else {
                box_.push((label, focal));
            }
        } else {
            box_.retain(|(x, _)| *x != label);
        }
    }

    let result = boxes
        .into_iter()
        .enumerate()
        .flat_map(|(i_box, lens)| {
            let i = i_box + 1;
            lens.into_iter()
                .enumerate()
                .map(move |(j, (_, focal))| i * (j + 1) * focal as usize)
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
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
