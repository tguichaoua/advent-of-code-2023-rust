use itertools::Itertools;

advent_of_code::solution!(13);

type Int = u32;

/* -------------------------------------------------------------------------- */

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<Int> {
    let result = input
        .lines()
        .batching(|it| {
            let mut lines = Vec::new();

            for line in it {
                if line.is_empty() {
                    break;
                }
                lines.push(line);
            }

            (!lines.is_empty()).then_some(lines)
        })
        .map(|pattern| {
            // Vertical

            let width = pattern.first().unwrap().len();
            'index: for x in 0..(width - 1) {
                //     l >= 0    and          r < width
                // x - i >= 0    and  x + i + 1 < width
                //    -i >= -x   and          i < width - x - 1
                //     i <= x
                //     i < x + 1

                let end = usize::min(x + 1, width - x - 1);
                for i in 0..end {
                    let l = x - i;
                    let r = x + i + 1;
                    for line in &pattern {
                        if line.as_bytes()[l] != line.as_bytes()[r] {
                            continue 'index;
                        }
                    }
                }

                return x + 1;
            }

            // Horizontal

            let height = pattern.len();
            'index: for y in 0..(height - 1) {
                let end = usize::min(y + 1, height - y - 1);
                for i in 0..end {
                    let t = y - i;
                    let b = y + i + 1;

                    let t = pattern[t];
                    let b = pattern[b];

                    if t.bytes().zip_eq(b.bytes()).any(|(t, b)| t != b) {
                        continue 'index;
                    }
                }

                return (y + 1) * 100;
            }

            unreachable!();
        })
        .sum::<usize>();

    Some(Int::try_from(result).unwrap())
}

/* -------------------------------------------------------------------------- */

pub fn part_two(input: &str) -> Option<Int> {
    let result = input
        .lines()
        .batching(|it| {
            let mut lines = Vec::new();

            for line in it {
                if line.is_empty() {
                    break;
                }
                lines.push(line);
            }

            (!lines.is_empty()).then_some(lines)
        })
        .map(|pattern| {
            // Vertical

            let width = pattern.first().unwrap().len();
            'index: for x in 0..(width - 1) {
                let mut used_joker = false;
                //     l >= 0    and          r < width
                // x - i >= 0    and  x + i + 1 < width
                //    -i >= -x   and          i < width - x - 1
                //     i <= x
                //     i < x + 1

                let end = usize::min(x + 1, width - x - 1);
                for i in 0..end {
                    let l = x - i;
                    let r = x + i + 1;
                    for line in &pattern {
                        if line.as_bytes()[l] != line.as_bytes()[r] {
                            if used_joker {
                                continue 'index;
                            }
                            used_joker = true;
                        }
                    }
                }

                if used_joker {
                    return x + 1;
                }
            }

            // Horizontal

            let height = pattern.len();
            'index: for y in 0..(height - 1) {
                let mut used_joker = false;
                let end = usize::min(y + 1, height - y - 1);
                for i in 0..end {
                    let t = y - i;
                    let b = y + i + 1;

                    let t = pattern[t];
                    let b = pattern[b];

                    for (t, b) in t.bytes().zip_eq(b.bytes()) {
                        if t != b {
                            if used_joker {
                                continue 'index;
                            }
                            used_joker = true;
                        }
                    }
                }

                if used_joker {
                    return (y + 1) * 100;
                }
            }

            unreachable!();
        })
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
