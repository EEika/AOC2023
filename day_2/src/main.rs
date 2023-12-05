use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct game_set {
    red: i32,
    green: i32,
    blue: i32,
}

fn extract_number(number_string: &str) -> i32 {
    let number_regex = Regex::new(r"\d+").unwrap();
    let number_value = number_regex
        .find(number_string)
        .map(|x| x.as_str())
        .unwrap_or("")
        .parse::<i32>()
        .unwrap_or(0);
    number_value
}

fn part_1() {
    let mut summer = 0;

    let file_path = "src/puzzle_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let red_ref = 12;
    let green_ref = 13;
    let blue_ref = 14;

    'outer: for line in reader.lines() {
        let line_string = line.unwrap();
        let game_num_strs = line_string.split(":");

        let game_num = extract_number(&line_string);

        for rounds_raw in game_num_strs.into_iter() {
            let rounds = rounds_raw.split(";");

            for round in rounds {
                let cubes = round.split(",");

                for cube in cubes {
                    if cube.contains("red") {
                        let red_val = extract_number(cube);
                        if red_val > red_ref {
                            continue 'outer;
                        };
                    } else if cube.contains("green") {
                        let green_val = extract_number(cube);
                        if green_val > green_ref {
                            continue 'outer;
                        };
                    } else if cube.contains("blue") {
                        let blue_val = extract_number(cube);
                        if blue_val > blue_ref {
                            continue 'outer;
                        };
                    }
                }
            }
        }

        summer += game_num;
    }
    println!("{summer}");
}
fn main() {
    let mut summer = 0;

    let file_path = "src/puzzle_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let ref_game_set = game_set {
        red: 12,
        green: 13,
        blue: 14,
    };

    'outer: for line in reader.lines() {
        let mut counter_game_set = game_set {
            red: 0,
            green: 0,
            blue: 0,
        };

        let line_string = line.unwrap();
        let game_num_strs = line_string.split(":");

        let game_num = extract_number(&line_string);

        for rounds_raw in game_num_strs.into_iter() {
            let rounds = rounds_raw.split(";");

            for round in rounds {
                let cubes = round.split(",");

                for cube in cubes {
                    print!("{} of {}", val, color);

                    if cube.contains("red") {
                        let red_val = extract_number(cube);
                        if red_val > counter_game_set(red) {
                            red_counter = red_val;
                        };
                    } else if cube.contains("green") {
                        let green_val = extract_number(cube);
                        if green_val > green_counter {
                            green_counter = green_val;
                        };
                    } else if cube.contains("blue") {
                        let blue_val = extract_number(cube);
                        if blue_val > blue_counter {
                            blue_counter = blue_val;
                        };
                    }
                }
            }
        }

        let game_power = blue_counter * green_counter * red_counter;

        summer += game_power;
    }
    println!("{summer}");
}
