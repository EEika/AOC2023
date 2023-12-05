use std::fs;

fn main() {
    let file_path = "puzzle_input.txt";
    let puzzle_string = fs::read_to_string(file_path).unwrap();
    let sum_part_1 = let_it_scratch(puzzle_string);

    let puzzle_string = fs::read_to_string(file_path).unwrap();
    let sum_part_2 = more_scratches(puzzle_string);
    println!("The sum part 1: {sum_part_1}");
    println!("The sum part 2: {sum_part_2}");
}

fn let_it_scratch(scratch_pile: String) -> i32 {
    let mut summer = 0;

    for line in scratch_pile.lines() {
        let mut matches = 0;

        let (_card_no, numbers) = line.split_once(":").unwrap();
        let (winning_numbers_str, scratched_numbers_str) = numbers.split_once("|").unwrap();

        let winning_numbers: Vec<i32> = winning_numbers_str
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        let scratched_numbers: Vec<i32> = scratched_numbers_str
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        // println!("{:?}", winning_numbers);
        // println!("{:?}", scratched_numbers);

        for winning_number in winning_numbers {
            if scratched_numbers.contains(&winning_number) {
                matches += 1;
                continue;
            }
        }

        let base: i32 = 2;
        summer += if matches > 0 {
            base.pow(matches - 1)
        } else {
            0
        };
    }

    summer
}

fn more_scratches(scratch_pile: String) -> i32 {
    let num_of_lines = scratch_pile.lines().count();
    // println!("Num of lines: {num_of_lines}");

    let mut line_counter = vec![1; num_of_lines];

    // println!("{:?}", line_counter);

    for line in scratch_pile.lines() {
        let mut matches = 0;

        let (card_str, numbers) = line.split_once(":").unwrap();
        let (_, card_no_str) = card_str.split_once(" ").unwrap();

        let card_no: usize = card_no_str.trim().parse::<usize>().unwrap();

        let (winning_numbers_str, scratched_numbers_str) = numbers.split_once("|").unwrap();

        let winning_numbers: Vec<i32> = winning_numbers_str
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        let scratched_numbers: Vec<i32> = scratched_numbers_str
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        // println!("{:?}", winning_numbers);
        // println!("{:?}", scratched_numbers);

        for winning_number in winning_numbers {
            if scratched_numbers.contains(&winning_number) {
                matches += 1;
                continue;
            }
        }

        for _ in 0..line_counter[card_no - 1] {
            for j in 0..matches {
                line_counter[card_no + j] += 1;
            }
        }

        // let base: i32 = 2;
        // summer += if matches > 0 {
        //     base.pow(matches - 1)
        // } else {
        //     0
        // };
    }

    line_counter.iter().sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part_1() {
        let input: String = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string();
        assert_eq!(let_it_scratch(input), 13);
    }

    #[test]
    fn test_part_2() {
        let input: String = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string();
        assert_eq!(more_scratches(input), 30);
    }
}
