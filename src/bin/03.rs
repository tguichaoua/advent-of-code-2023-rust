use itertools::Itertools;

advent_of_code::solution!(3);

/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Copy)]
struct Number {
    line: usize,
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, Copy)]
struct Gear {
    line: usize,
    col: usize,
    number: Number,
}

/* -------------------------------------------------------------------------- */

struct Input {
    data: Vec<Vec<u8>>,
    numbers: Vec<Number>,
    line_len: usize,
}

fn parse_input(input: &str) -> Input {
    debug_assert!(
        input.chars().all(|c| c.is_ascii()),
        "expected input to be an ascii string"
    );

    // NOTE: we assume the input is an ascii string
    let data = input
        .lines()
        .map(|line| line.bytes().collect_vec())
        .collect_vec();

    let line_len = data.first().unwrap().len();

    debug_assert!(
        data.iter().all(|line| line.len() == line_len),
        "expected all line to have the same len"
    );

    let numbers = {
        let mut numbers = Vec::new();
        for (line, data) in data.iter().enumerate() {
            let mut data = data.iter().enumerate().peekable();
            while let Some((start, c)) = data.next() {
                if c.is_ascii_digit() {
                    let end = loop {
                        match data.peek() {
                            Some((_, c)) if c.is_ascii_digit() => {
                                data.next();
                                continue;
                            }
                            Some((end, _)) => break *end,
                            None => break line_len,
                        }
                    };
                    numbers.push(Number { line, start, end });
                }
            }
        }
        numbers
    };

    Input {
        data,
        numbers,
        line_len,
    }
}

fn parse_number(data: &[Vec<u8>], number: Number) -> u32 {
    let Number { line, start, end } = number;
    let str = data[line].get(start..end).unwrap();
    let str = std::str::from_utf8(str).unwrap();
    str.parse::<u32>().unwrap()
}

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<u32> {
    let Input {
        data,
        numbers,
        line_len,
    } = parse_input(input);

    fn is_symbol(c: u8) -> bool {
        !c.is_ascii_digit() && c != b'.'
    }

    let result = numbers
        .into_iter()
        .filter(|Number { line, start, end }| {
            let range = {
                let start = start.saturating_sub(1);
                let end = if *end == line_len { line_len } else { end + 1 };
                start..end
            };

            let is_there_symbol = |line: usize| {
                if let Some(data) = data.get(line) {
                    let symbols = data.get(range.clone()).unwrap();
                    symbols.iter().copied().any(is_symbol)
                } else {
                    false
                }
            };

            if let Some(line) = line.checked_sub(1) {
                if is_there_symbol(line) {
                    return true;
                }
            }

            if is_there_symbol(*line) {
                return true;
            }

            if is_there_symbol(line + 1) {
                return true;
            }

            false
        })
        .map(|number| parse_number(&data, number))
        .sum();

    Some(result)
}

/* -------------------------------------------------------------------------- */

pub fn part_two(input: &str) -> Option<u32> {
    let Input {
        data,
        numbers,
        line_len,
    } = parse_input(input);

    let mut gears = Vec::new();
    for number @ Number { line, start, end } in numbers {
        let range = {
            let start = start.saturating_sub(1);
            let end = if end == line_len { line_len } else { end + 1 };
            start..end
        };

        let mut get_gears = |line: usize| {
            if let Some(data) = data.get(line) {
                for col in range.clone() {
                    if data[col] == b'*' {
                        gears.push(Gear { line, col, number });
                    }
                }
            }
        };

        if let Some(line) = line.checked_sub(1) {
            get_gears(line);
        }

        get_gears(line);

        get_gears(line + 1);
    }

    let gears = gears
        .into_iter()
        .into_group_map_by(|Gear { line, col, .. }| (*line, *col));

    let result = gears
        .into_values()
        .filter_map(|values| -> Option<[_; 2]> { values.try_into().ok() })
        .map(|[a, b]| {
            let a = parse_number(&data, a.number);
            let b = parse_number(&data, b.number);

            a * b
        })
        .sum();

    Some(result)
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
