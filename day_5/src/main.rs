use std::collections::HashMap;
use std::fs;
use std::ops::Range;
use std::panic::Location;

fn main() {
    // let puzzle_input = "50 98 2
    // 52 50 48";
    // part_1(puzzle_input);

    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    part_2(&puzzle_input[..]);
}
fn part_2(almanac: &str) {
    let paragraphs: Vec<&str> = almanac.split("\n\n").collect();

    let mut table_dict: HashMap<&str, ConvertTable> = HashMap::new();

    let mut seed_vec: Vec<i64> = vec![];

    for p in paragraphs {
        let (p_title, table_values) = p.split_once(":").unwrap();

        let mut convert_table: ConvertTable = ConvertTable::new();

        if p_title.starts_with("seeds") {
            println!("Start of seeds");
            let mut line_splits: Vec<i64> = table_values
                .trim()
                .split_whitespace()
                .enumerate()
                .map(|(_, s)| s.parse::<i64>().unwrap())
                .collect();

            for seed_split in line_splits.chunks(2) {
                let seed_start: i64 = seed_split[0];
                let seed_range: i64 = seed_split[1];

                println!("{seed_range}");
                println!("{seed_start}");

                // let mut r = seed_start..seed_range;

                // r.for_each(|s| seed_vec.push(s));

                // seed_vec.extend_from_slice(&r);

                // println!("{:?}", r);

                let mut seed: Vec<i64> = (seed_start..seed_range).collect::<Vec<i64>>();
                println!("{:?}", seed);

                seed_vec.append(&mut seed);
            }
            // println!("{:?}", seed);
        } else {
            let (table_name, _) = p_title.split_once(" ").unwrap();
            for line_values in table_values.trim().lines() {
                let mut line_splits = line_values.trim().split_whitespace();

                let destination = line_splits.next().unwrap().trim().parse::<i64>().unwrap();
                let source = line_splits.next().unwrap().trim().parse::<i64>().unwrap();
                let span = line_splits.next().unwrap().trim().parse::<i64>().unwrap();

                convert_table.convert_lines.push(ConvertLine {
                    from: (source..(source + span)),
                    to: (destination..(destination + span)),
                    difference: (destination - source),
                });
            }

            table_dict.insert(table_name, convert_table);
        }
    }

    println!("The seeds: {:?}", seed_vec);

    // println!("The tables --> {:?}", table_dict);

    let mut locations = vec![];
    for seed in seed_vec {
        // println!("The seed: {}", seed);
        let soil = table_dict["seed-to-soil"].convert(seed);
        let fertilizer = table_dict["soil-to-fertilizer"].convert(soil);
        let water = table_dict["fertilizer-to-water"].convert(fertilizer);
        let light = table_dict["water-to-light"].convert(water);
        let temperature = table_dict["light-to-temperature"].convert(light);
        let humidity = table_dict["temperature-to-humidity"].convert(temperature);
        let location = table_dict["humidity-to-location"].convert(humidity);
        // println!("The loc: {}", location);
        locations.push(location);
    }

    locations.sort();
    println!("The locations: {:?}", locations);
}

fn part_1(almanac: &str) {
    let paragraphs: Vec<&str> = almanac.split("\n\n").collect();

    let mut table_dict: HashMap<&str, ConvertTable> = HashMap::new();

    let mut seed_vec: Vec<i64> = vec![];

    for p in paragraphs {
        let (p_title, table_values) = p.split_once(":").unwrap();

        let mut convert_table: ConvertTable = ConvertTable::new();

        if p_title.starts_with("seeds") {
            seed_vec = table_values
                .trim()
                .split_whitespace()
                .map(|v| v.trim().parse::<i64>().unwrap())
                .collect();
        } else {
            let (table_name, _) = p_title.split_once(" ").unwrap();
            for line_values in table_values.trim().lines() {
                let mut line_splits = line_values.trim().split_whitespace();

                let destination = line_splits
                    .next()
                    .unwrap()
                    .trim()
                    .parse::<i64>()
                    .expect("Something did not parse");
                let source = line_splits.next().unwrap().trim().parse::<i64>().unwrap();
                let span = line_splits.next().unwrap().trim().parse::<i64>().unwrap();

                convert_table.convert_lines.push(ConvertLine {
                    from: (source..(source + span)),
                    to: (destination..(destination + span)),
                    difference: (destination - source),
                });
            }

            table_dict.insert(table_name, convert_table);
        }
    }

    // println!("The tables --> {:?}", table_dict);

    let mut locations = vec![];
    for seed in seed_vec {
        // println!("The seed: {}", seed);
        let soil = table_dict["seed-to-soil"].convert(seed);
        let fertilizer = table_dict["soil-to-fertilizer"].convert(soil);
        let water = table_dict["fertilizer-to-water"].convert(fertilizer);
        let light = table_dict["water-to-light"].convert(water);
        let temperature = table_dict["light-to-temperature"].convert(light);
        let humidity = table_dict["temperature-to-humidity"].convert(temperature);
        let location = table_dict["humidity-to-location"].convert(humidity);
        // println!("The loc: {}", location);
        locations.push(location);
    }

    locations.sort();
    println!("The locations: {:?}", locations);
}

#[derive(Debug)]
struct ConvertTable {
    convert_lines: Vec<ConvertLine>,
}

impl ConvertTable {
    fn new() -> Self {
        ConvertTable {
            convert_lines: vec![],
        }
    }

    fn convert(&self, value: i64) -> i64 {
        for line in self.convert_lines.iter() {
            if line.from.contains(&value) {
                return value + line.difference;
            };
        }
        value
    }
}

#[derive(Debug)]
struct ConvertLine {
    from: Range<i64>,
    to: Range<i64>,
    difference: i64,
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part_1() {
        let input = "50 98 2
        52 50 48";
    }
}
