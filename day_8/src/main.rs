use num::integer::lcm;
use std::collections::HashMap;
use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();

    // let puzzle_input = "LR

    // 11A = (11B, XXX)
    // 11B = (XXX, 11Z)
    // 11Z = (11B, XXX)
    // 22A = (22B, XXX)
    // 22B = (22C, 22C)
    // 22C = (22Z, 22Z)
    // 22Z = (22B, 22B)
    // XXX = (XXX, XXX)";

    let result_part_1 = part_1(&puzzle_input[..]);

    println!("Res part 1: {result_part_1}");
}

fn part_1(puzzle_input: &str) -> i64 {
    let (directions, map_lines) = puzzle_input.split_once("\n\n").unwrap();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in map_lines.trim().split("\n") {
        // println!("{}", line.trim());
        let (key, left_right) = line.trim().split_once("=").unwrap();
        let l_r = &left_right.trim()[1..left_right.len() - 2];
        let (left, right) = l_r.split_once(',').unwrap();
        map.insert(key.trim(), (left.trim(), right.trim()));
    }

    // println!("{:?}", map);

    let mut solutions: Vec<i64> = vec![];

    let mut placements: Vec<&str> = map
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|p| *p)
        .collect();

    println!("Placements: {:?}", placements);

    // let mut placement = "AAA";

    for node in &mut placements {
        let mut counter: i64 = 0;

        'outer: while !node.ends_with('Z') {
            for dir in directions.chars() {
                counter += 1;

                match dir {
                    'L' => {
                        *node = map[node].0;
                    }
                    'R' => {
                        *node = map[node].1;
                    }
                    _ => {}
                }

                if node.ends_with('Z') {
                    println!("{}", counter);
                    solutions.push(counter);
                    break 'outer;
                }
            }
        }
    }

    println!("{:?}", solutions);
    let mut solution: i64 = 1;
    println!("Sol: {:?}", solution);

    for s in solutions {
        solution = num::integer::lcm(solution, s);
    }

    solution
}

// fn parse_input() -> {}

#[cfg(test)]
mod tests {
    use crate::part_1;

    #[test]
    fn test_part_1() {
        let puzzle_input = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

        let res = part_1(puzzle_input);
        assert_eq!(res, 6);
    }
}
