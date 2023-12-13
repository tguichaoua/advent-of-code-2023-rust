use itertools::Itertools;

advent_of_code::solution!(12);

type Int = u32;

/* -------------------------------------------------------------------------- */

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<Int> {
    let result = input
        .lines()
        .map(|line| {
            let (data, groups) = line.split_once(' ').unwrap();

            let free = data.bytes().filter(|&b| b == b'?').count();

            let groups = groups
                .split(',')
                .map(|x| x.parse::<Int>().unwrap())
                .collect_vec();

            let mut valid_pattern_count = 0;

            'pattern: for pattern in 0u32..=((1 << free) - 1) {
                let mut groups = groups.iter().peekable();
                let mut x = 0;
                let mut cur_group_count = 0;
                let mut in_group = false;

                for mut c in data.bytes() {
                    if c == b'?' {
                        c = if (pattern & (1 << x)) == 0 {
                            b'.'
                        } else {
                            b'#'
                        };
                        x += 1;
                    }

                    if !in_group && c == b'.' {
                        continue;
                    }

                    in_group = true;

                    match c {
                        b'#' => {
                            cur_group_count += 1;
                            let Some(&&expected) = groups.peek() else {
                                continue 'pattern;
                            };
                            if cur_group_count > expected {
                                continue 'pattern;
                            }
                        }

                        b'.' => {
                            if groups.next().copied() == Some(cur_group_count) {
                                cur_group_count = 0;
                                in_group = false;
                            } else {
                                continue 'pattern;
                            }
                        }

                        _ => unreachable!(),
                    }
                }

                if (!in_group || (groups.next().copied() == Some(cur_group_count)))
                    && groups.next().is_none()
                {
                    valid_pattern_count += 1;
                }
            }

            valid_pattern_count
        })
        .sum::<Int>();

    Some(result)
}

/* -------------------------------------------------------------------------- */

pub fn part_two(_input: &str) -> Option<Int> {
    None
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
        assert_eq!(result, None);
    }
}
