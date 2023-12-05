use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let mut summer = 0;

    let file_path = "src/puzzle_input_1.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    // let re = Regex::new(r"\d").unwrap();

    for line in reader.lines() {
        let patterns: Vec<&str> = vec![
            r"one", r"two", r"three", r"four", r"five", r"six", r"seven", r"eight", r"nine", r"\d",
        ];
        // let patterns: Vec<&str> = vec![r"\d"];
        let mut first_num_index: usize = 9999;
        let mut last_num_index: usize = 0;
        let mut first_num: &str = "";
        let mut last_num: &str = "";

        let mut temp_num_str: String = "".to_owned();
        let line_str: &str = &line.unwrap()[..];

        for p in patterns.into_iter() {
            let re = Regex::new(p).unwrap();

            let col_count = re.find_iter(line_str).count();
            let cols = re.find_iter(line_str);

            if col_count > 0 {
                for (i, c) in cols.into_iter().enumerate() {
                    print!("{:?}", c);
                    if i == 0 && c.start() <= first_num_index {
                        first_num_index = c.start();
                        match c.as_str() {
                            "one" => first_num = "1",
                            "two" => first_num = "2",
                            "three" => first_num = "3",
                            "four" => first_num = "4",
                            "five" => first_num = "5",
                            "six" => first_num = "6",
                            "seven" => first_num = "7",
                            "eight" => first_num = "8",
                            "nine" => first_num = "9",
                            _ => first_num = c.as_str(),
                        }
                    };
                    if i == col_count - 1 && c.start() >= last_num_index {
                        last_num_index = c.start();
                        match c.as_str() {
                            "one" => last_num = "1",
                            "two" => last_num = "2",
                            "three" => last_num = "3",
                            "four" => last_num = "4",
                            "five" => last_num = "5",
                            "six" => last_num = "6",
                            "seven" => last_num = "7",
                            "eight" => last_num = "8",
                            "nine" => last_num = "9",
                            _ => last_num = c.as_str(),
                        }
                    }
                }
            }
        }

        temp_num_str.push_str(first_num);
        temp_num_str.push_str(last_num);

        if !temp_num_str.is_empty() {
            println!("The temp sum: {temp_num_str}");
            summer += temp_num_str.parse::<i32>().unwrap();
        }
    }

    println!("{summer}");
}
