use itertools::Itertools;

advent_of_code::solution!(5);

/* -------------------------------------------------------------------------- */

type Range = std::ops::Range<u64>;

struct MapRange {
    dst_start: u64,
    source_range: Range,
}

impl MapRange {
    fn map(&self, value: u64) -> u64 {
        value - self.source_range.start + self.dst_start
    }

    fn map_range(&self, range: Range) -> Range {
        self.map(range.start)..self.map(range.end)
    }
}

struct ResourceMap {
    // INVARIANT: ranges are sorted by `source_range.start`
    ranges: Vec<MapRange>,
}

impl ResourceMap {
    fn new(mut ranges: Vec<MapRange>) -> Self {
        ranges.sort_by_key(|range| range.source_range.start);
        Self { ranges }
    }

    fn map(&self, value: u64) -> u64 {
        for map in self.ranges.iter() {
            if map.source_range.contains(&value) {
                return map.map(value);
            }
        }
        value
    }

    fn map_range(&self, mut range: Range) -> Vec<Range> {
        // Okay, this function looks horrible but look how the rest
        // of code look elegant thanks to this function :)

        let mut ranges = Vec::new();

        for map_range @ MapRange {
            dst_start: _,
            source_range: map,
        } in self.ranges.iter()
        {
            if range.start < map.start {
                // `range` start is before `map`.
                if range.end <= map.start {
                    // `range` is *completely* before `map`, also `map`s are sorted so we'll never match another range.
                    ranges.push(range);
                    return ranges;
                } else {
                    // `range` first part is before `map`
                    ranges.push(range.start..map.start);

                    if range.end <= map.end {
                        // the second part of `range` is *completely* inside `map`
                        ranges.push(map_range.map_range(map.start..range.end));

                        return ranges;
                    } else {
                        // The second part of `range` covers `map`,
                        ranges.push(map_range.map_range(map.start..map.end));

                        // and there is a third part.
                        range.start = map.end;

                        // Let's map the third part.
                        continue;
                    }
                }
            } else if range.start >= map.end {
                // `range` is totally after `map`, let's check for another map range.
                continue;
            } else if range.end <= map.end {
                // `range` is totally included inside `map` range.
                ranges.push(map_range.map_range(range));

                return ranges;
            } else {
                // The first part of `range` is *completely* inside `map`.
                ranges.push(map_range.map_range(range.start..map.end));

                // Let's map the second part.
                range.start = map.end;

                continue;
            }
        }

        ranges.push(range);

        ranges
    }
}

struct Maps {
    seed_to_soil: ResourceMap,
    soil_to_fertilizer: ResourceMap,
    fertilizer_to_water: ResourceMap,
    water_to_light: ResourceMap,
    light_to_temperature: ResourceMap,
    temperature_to_humidity: ResourceMap,
    humidity_to_location: ResourceMap,
}

fn parse_input(input: &str) -> (impl Iterator<Item = u64> + '_, Maps) {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds:")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap());
    let _ = lines.next(); // empty line

    macro_rules! parse_resource_map {
        (
            $( $name:ident => $header:expr; )*
        ) => {
            Maps {
                $(
                    $name: {
                        let header = lines.next();
                        debug_assert_eq!(header, Some($header));
                        let ranges = lines
                            .by_ref()
                            .take_while(|line| !line.is_empty())
                            .map(parse_map_range)
                            .collect();
                        ResourceMap::new(ranges)
                    },
                )*
            }

        };
    }

    let maps = parse_resource_map! {
        seed_to_soil            => "seed-to-soil map:";
        soil_to_fertilizer      => "soil-to-fertilizer map:";
        fertilizer_to_water     => "fertilizer-to-water map:";
        water_to_light          => "water-to-light map:";
        light_to_temperature    => "light-to-temperature map:";
        temperature_to_humidity => "temperature-to-humidity map:";
        humidity_to_location    => "humidity-to-location map:";
    };

    (seeds, maps)
}

fn parse_map_range(str: &str) -> MapRange {
    let (a, rest) = str.split_once(' ').unwrap();
    let (b, c) = rest.split_once(' ').unwrap();

    let dst_start = a.parse().unwrap();
    let src_start = b.parse().unwrap();
    let len: u64 = c.parse().unwrap();

    let source_range = src_start..(src_start + len);

    MapRange {
        dst_start,
        source_range,
    }
}

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, maps) = parse_input(input);

    let locations = seeds
        .map(|seed| maps.seed_to_soil.map(seed))
        .map(|soil| maps.soil_to_fertilizer.map(soil))
        .map(|fertilizer| maps.fertilizer_to_water.map(fertilizer))
        .map(|water| maps.water_to_light.map(water))
        .map(|light| maps.light_to_temperature.map(light))
        .map(|temperature| maps.temperature_to_humidity.map(temperature))
        .map(|humidity| maps.humidity_to_location.map(humidity));

    let result = locations.min().unwrap();

    Some(result)
}

/* -------------------------------------------------------------------------- */

// pub fn part_two(input: &str) -> Option<u64> {
//     let (seeds, maps) = parse_input(input);

//     let seeds = seeds.tuples().map(|(start, len)| Range { start, len });

//     let locations = seeds
//         .flat_map(|seeds| maps.seed_to_soil.map_range(seeds))
//         .flat_map(|soils| maps.soil_to_fertilizer.map_range(soils))
//         .flat_map(|fertilizers| maps.fertilizer_to_water.map_range(fertilizers))
//         .flat_map(|waters| maps.water_to_light.map_range(waters))
//         .flat_map(|lights| maps.light_to_temperature.map_range(lights))
//         .flat_map(|temperatures| maps.temperature_to_humidity.map_range(temperatures))
//         .flat_map(|humidities| maps.humidity_to_location.map_range(humidities));

//     let result = locations
//         .map(|localizations| localizations.start)
//         .min()
//         .unwrap();

//     Some(result)
// }

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, maps) = parse_input(input);

    let seeds = seeds.tuples().map(|(start, len)| start..(start + len));

    let locations = seeds
        .flat_map(|seeds| maps.seed_to_soil.map_range(seeds))
        .flat_map(|soils| maps.soil_to_fertilizer.map_range(soils))
        .flat_map(|fertilizers| maps.fertilizer_to_water.map_range(fertilizers))
        .flat_map(|waters| maps.water_to_light.map_range(waters))
        .flat_map(|lights| maps.light_to_temperature.map_range(lights))
        .flat_map(|temperatures| maps.temperature_to_humidity.map_range(temperatures))
        .map(|humidities| humidities.start)
        .map(|humidity| maps.humidity_to_location.map(humidity));

    let result = locations.min().unwrap();

    Some(result)
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
