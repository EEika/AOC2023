use std::fs;

fn main() {
    if let Ok(s) = fs::read_to_string("puzzle_input.txt") {
        let puzzle_input = s;
        let (solution_part_1, solution_part_2) = part_the_first_and_second(&puzzle_input[..]);

        println!("Solution part 1: {}", solution_part_1);
        println!("Solution part 2: {}", solution_part_2);
    };
}

fn part_the_first_and_second(input: &str) -> (i32, i32) {
    let mut results_past: Vec<i32> = vec![];
    let mut results_post: Vec<i32> = vec![];
    for line in input.split("\n") {
        let mut record_reductios: Vec<Vec<i32>> = vec![];
        let line_splits = line.split_whitespace();
        let line_recordings: Vec<i32> = line_splits
            .into_iter()
            .map(|l| l.trim().parse::<i32>().unwrap())
            .collect();

        record_reductios.push(line_recordings);
        while !record_reductios.last().unwrap().iter().all(|r| *r == 0) {
            let last_record_reduction = record_reductios.last().unwrap();
            let mut next_record_reduction: Vec<i32> = vec![];
            for i in 0..(record_reductios.last().unwrap().len() - 1) {
                next_record_reduction.push(last_record_reduction[i + 1] - last_record_reduction[i]);
            }

            record_reductios.push(next_record_reduction);
        }

        let mut last_element_summer = 0;
        let mut first_element_summer = 0;

        for record in record_reductios.iter().rev() {
            last_element_summer += record.last().unwrap();
            first_element_summer = record.first().unwrap() - first_element_summer;
        }

        results_post.push(last_element_summer);
        results_past.push(first_element_summer);
    }

    // println!("{:?}", record_reductios);
    (
        results_post.into_iter().sum(),
        results_past.into_iter().sum(),
    )
}
