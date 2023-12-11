use std::collections::HashMap;
use std::fs;
use std::ops::Range;

fn main() {
    // let puzzle_input = "50 98 2
    // 52 50 48";
    // part_1(puzzle_input);

    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    part_2(&puzzle_input[..]);
}

fn part_2(almanac: &str) {
    let paragraphs: Vec<&str> = almanac.split("\n\n").collect();

    let mut almanac: Almanac = Almanac::new();

    for p in paragraphs {
        let (p_title, table_values) = p.split_once(":").unwrap();

        if p_title.starts_with("seeds") {
            let mut line_splits: Vec<i64> = table_values
                .trim()
                .split_whitespace()
                .enumerate()
                .map(|(_, s)| s.parse::<i64>().unwrap())
                .collect();

            for seed_split in line_splits.chunks(2) {
                let seed_start: i64 = seed_split[0];
                let seed_span: i64 = seed_split[1];

                almanac.seeds.push(seed_start..(seed_start + seed_span - 1));
            }
        } else {
            let mut convert_table: ConvertTable = ConvertTable::new();

            let (table_name, _) = p_title.split_once(" ").unwrap();
            for line_values in table_values.trim().lines() {
                let mut line_splits = line_values.trim().split_whitespace();

                let destination = line_splits.next().unwrap().trim().parse::<i64>().unwrap();
                let source = line_splits.next().unwrap().trim().parse::<i64>().unwrap();
                let span = line_splits.next().unwrap().trim().parse::<i64>().unwrap();

                convert_table.convert_lines.push(ConvertLine {
                    from: (source..source + span - 1),
                    difference: (destination - source),
                });
            }

            almanac
                .convert_tables
                .insert(table_name.into(), convert_table);
        }
    }
    println!("Length: {}", almanac.seeds.len());

    let soil = almanac.convert_tables["seed-to-soil"].convert(almanac.seeds);

    println!("{:?}", soil);
    let fertilizer = almanac.convert_tables["soil-to-fertilizer"].convert(soil);
    let water = almanac.convert_tables["fertilizer-to-water"].convert(fertilizer);
    let light = almanac.convert_tables["water-to-light"].convert(water);
    let temperature = almanac.convert_tables["light-to-temperature"].convert(light);
    let humidity = almanac.convert_tables["temperature-to-humidity"].convert(temperature);
    let mut locations = almanac.convert_tables["humidity-to-location"].convert(humidity);

    println!("Num of locs: {:?}", locations.len());
    println!("The locs: {:?}", locations);

    locations.sort_by_key(|l| l.start);

    // println!("The location: {:?}", locations[0].start);
    // println!("The loc: {}", location);
}

// fn part_1(almanac: &str) {
//     let paragraphs: Vec<&str> = almanac.split("\n\n").collect();

//     let mut table_dict: HashMap<&str, ConvertTable> = HashMap::new();

//     let mut seed_vec: Vec<i64> = vec![];

//     for p in paragraphs {
//         let (p_title, table_values) = p.split_once(":").unwrap();

//         let mut convert_table: ConvertTable = ConvertTable::new();

//         if p_title.starts_with("seeds") {
//             seed_vec = table_values
//                 .trim()
//                 .split_whitespace()
//                 .map(|v| v.trim().parse::<i64>().unwrap())
//                 .collect();
//         } else {
//             let (table_name, _) = p_title.split_once(" ").unwrap();
//             for line_values in table_values.trim().lines() {
//                 let mut line_splits = line_values.trim().split_whitespace();

//                 let destination = line_splits
//                     .next()
//                     .unwrap()
//                     .trim()
//                     .parse::<i64>()
//                     .expect("Something did not parse");
//                 let source = line_splits.next().unwrap().trim().parse::<i64>().unwrap();
//                 let span = line_splits.next().unwrap().trim().parse::<i64>().unwrap();

//                 convert_table.convert_lines.push(ConvertLine {
//                     from: (source..(source + span)),
//                     to: (destination..(destination + span)),
//                     difference: (destination - source),
//                 });
//             }

//             table_dict.insert(table_name, convert_table);
//         }
//     }

//     // println!("The tables --> {:?}", table_dict);

//     let mut locations = vec![];
//     for seed in seed_vec {
//         // println!("The seed: {}", seed);
//         let soil = table_dict["seed-to-soil"].convert(seed);
//         let fertilizer = table_dict["soil-to-fertilizer"].convert(soil);
//         let water = table_dict["fertilizer-to-water"].convert(fertilizer);
//         let light = table_dict["water-to-light"].convert(water);
//         let temperature = table_dict["light-to-temperature"].convert(light);
//         let humidity = table_dict["temperature-to-humidity"].convert(temperature);
//         let location = table_dict["humidity-to-location"].convert(humidity);
//         // println!("The loc: {}", location);
//         locations.push(location);
//     }

//     locations.sort();
//     println!("The locations: {:?}", locations);
// }

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Range<i64>>,
    convert_tables: HashMap<String, ConvertTable>,
}

impl Almanac {
    fn new() -> Self {
        Self {
            seeds: vec![],
            convert_tables: HashMap::new(),
        }
    }
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

    fn convert(&self, input_ranges: Vec<Range<i64>>) -> Vec<Range<i64>> {
        let mut conformed_ranges: Vec<Range<i64>> = vec![];

        println!("{:?}", &self.convert_lines);
        for conv_line in &self.convert_lines {
            conformed_ranges = ConvertTable::conform_ranges(input_ranges.clone(), &conv_line.from);
        }

        let mut converted_ranges: Vec<Range<i64>> = vec![];
        println!("{:?}", conformed_ranges);

        for i_range in &conformed_ranges {
            for conv_line in &self.convert_lines {
                if conv_line.from.contains(&i_range.start) {
                    converted_ranges.push(
                        (&i_range.start + conv_line.difference)
                            ..(&i_range.end + conv_line.difference),
                    );
                    converted_ranges.push(i_range.clone());
                } else {
                }
            }
        }
        println!("{:?}", converted_ranges);

        converted_ranges
    }

    fn conform_ranges(
        input_ranges: Vec<Range<i64>>,
        reference_range: &Range<i64>,
    ) -> Vec<Range<i64>> {
        let mut revised_ranges: Vec<Range<i64>> = vec![];

        for range in input_ranges {
            match (
                range.start < reference_range.start,
                range.end <= reference_range.end,
                (range.end <= reference_range.start || range.start >= reference_range.end),
            ) {
                (_, _, true) => {
                    println!("Outside");
                    revised_ranges.push(range);
                }
                (false, true, _) => {
                    println!("Inside");
                    revised_ranges.push(range);
                }
                (true, true, _) => {
                    println!("Start-Overflow");
                    let split_ranges = &[
                        range.start..reference_range.start,
                        reference_range.start..range.end,
                    ];
                    revised_ranges.extend_from_slice(split_ranges);
                }
                (false, false, _) => {
                    println!("End-Overflow");
                    let split_ranges = &[
                        range.start..reference_range.end,
                        reference_range.end..range.end,
                    ];
                    revised_ranges.extend_from_slice(split_ranges);
                }
                (true, false, _) => {
                    println!("Full-Overflow");
                    let split_ranges = &[
                        range.start..reference_range.start,
                        reference_range.start..reference_range.end,
                        reference_range.end..range.end,
                    ];
                    revised_ranges.extend_from_slice(split_ranges);
                }
            }
        }
        revised_ranges
    }
}

#[derive(Debug)]
struct ConvertLine {
    from: Range<i64>,
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
