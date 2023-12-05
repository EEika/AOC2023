use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
#[derive(Debug, PartialEq, Clone, Copy)]
struct EnginePart {
    part_no: i32,
    line: usize,
    start: usize,
    end: usize,
}
#[derive(Debug, PartialEq)]

struct ValidMark {
    line: usize,
    start: usize,
}
struct Gear {
    valid_mark: ValidMark,
    connected_parts: Vec<EnginePart>,
}

fn main() {
    let mut summer = 0;
    let mut gear_summer = 0;
    let file_path = "src/puzzle_input_simple.txt";
    let file = File::open(file_path).unwrap();
    let buf = BufReader::new(file);
    let mut engine_parts: Vec<EnginePart> = Vec::new();
    let mut markers: Vec<ValidMark> = vec![];
    let mut gears: Vec<Gear> = vec![];

    let re = Regex::new(r"\d+").unwrap();
    let va = Regex::new(r"[^\d|\.|\n]").unwrap();
    let ge = Regex::new(r"\*").unwrap();

    for (index, line) in buf.lines().enumerate() {
        let line_str = &line.unwrap()[..];
        let captures = re.captures_iter(line_str);

        for capture in captures.into_iter() {
            for cap in capture.iter() {
                if let Some(m) = cap {
                    engine_parts.push(EnginePart {
                        part_no: (m.as_str().parse::<i32>().unwrap()),
                        line: (index),
                        start: (m.start()),
                        end: (m.end()),
                    });
                }
            }
        }

        let cap_markers = va.captures_iter(line_str);
        for cap_mark in cap_markers.into_iter() {
            for cap in cap_mark.iter() {
                if let Some(m) = cap {
                    markers.push(ValidMark {
                        line: (index),
                        start: (m.start()),
                    });
                }
            }
        }

        let gear_marks = ge.captures_iter(line_str);
        for cap_mark in gear_marks {
            for cap in cap_mark.iter() {
                if let Some(m) = cap {
                    gears.push(Gear {
                        valid_mark: ValidMark {
                            line: (index),
                            start: (m.start()),
                        },
                        connected_parts: vec![],
                    });
                }
            }
        }
    }

    // for mark in &markers {
    //     // println!("{:?}", mark);
    // }
    for part in &engine_parts {
        if check_if_valid_part(part.start, part.end, part.line, &markers) {
            summer += part.part_no;
        }
        // println!("{:?}", part);
    }

    for part in &engine_parts {
        check_connected_gears(part, part.start, part.end, part.line, &mut gears);
    }

    for gear in gears {
        if gear.connected_parts.len() == 2 {
            let g_r: i32 = gear
                .connected_parts
                .into_iter()
                .map(|p| p.part_no)
                .collect::<Vec<i32>>()
                .into_iter()
                .product();

            gear_summer += g_r;
        }
    }
    println!("Part1: {}", summer);
    println!("Part2: {}", gear_summer);
}

fn check_if_valid_part(
    part_start: usize,
    part_end: usize,
    part_line: usize,
    validation_vector: &Vec<ValidMark>,
) -> bool {
    for l in (if part_line > 0 { part_line - 1 } else { 0 })..(part_line + 2) {
        for i in (if part_start > 0 { part_start - 1 } else { 0 })..(part_end + 1) {
            let mark = ValidMark { start: i, line: l };
            if validation_vector.contains(&mark) {
                return true;
            }
        }
    }

    false
}
fn check_connected_gears(
    engine_part: &EnginePart,
    part_start: usize,
    part_end: usize,
    part_line: usize,
    gears_vector: &mut Vec<Gear>,
) {
    for l in (if part_line > 0 { part_line - 1 } else { 0 })..(part_line + 2) {
        for i in (if part_start > 0 { part_start - 1 } else { 0 })..(part_end + 1) {
            let mark: ValidMark = ValidMark { start: i, line: l };
            for m in &mut *gears_vector {
                if m.valid_mark == mark && (!m.connected_parts.contains(engine_part)) {
                    m.connected_parts.push(*engine_part);
                }
            }
        }
    }
}
