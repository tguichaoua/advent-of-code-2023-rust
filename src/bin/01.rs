use core::panic;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let first = line.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
            let num = format!("{first}{last}");
            num.parse::<u32>().unwrap()
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    debug_assert!(
        input.chars().all(|c| c.is_ascii()),
        "expected input to be an ascii string"
    );

    const DIGITS: [(&str, char); 10] = [
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let result = input
        .lines()
        .map(|line| {
            // NOTE: we assume the input is an ascii string

            let first = (|| {
                for i in 0..line.len() {
                    let Some(x) = line.get(i..) else {
                        break;
                    };
                    let first = x.chars().next().unwrap();
                    if first.is_ascii_digit() {
                        return first;
                    }

                    for (digit_str, value) in DIGITS {
                        if x.starts_with(digit_str) {
                            return value;
                        }
                    }
                }

                panic!("not found");
            })();

            let last = (|| {
                for i in 0..line.len() {
                    let Some(x) = line.get(0..(line.len() - i)) else {
                        break;
                    };
                    let last = x.chars().next_back().unwrap();
                    if last.is_ascii_digit() {
                        return last;
                    }

                    for (digit_str, value) in DIGITS {
                        if x.ends_with(digit_str) {
                            return value;
                        }
                    }
                }

                panic!("not found");
            })();

            let num = format!("{first}{last}");
            num.parse::<u32>().unwrap()
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string(
            advent_of_code::template::data_path()
                .join("examples")
                .join("01-2.txt"),
        )
        .expect("could not read the file");

        let result = part_two(&input);
        assert_eq!(result, Some(281));
    }
}
