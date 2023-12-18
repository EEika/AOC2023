fn main() {
    let puzzle_input = &std::fs::read_to_string("puzzle_input.txt").unwrap()[..];

    let solution_part_1 = christmas_machine(puzzle_input, 1);
    println!("{solution_part_1}");

    let solution_part_2 = christmas_machine(puzzle_input, 1000000 - 1);
    println!("{solution_part_2}");
}

fn christmas_machine(input: &str, expantion_factor: i64) -> i64 {
    let mut galaxy_chart = GalaxyChart::new();

    galaxy_chart.chart(input, expantion_factor);

    let mut galaxy_pairs: Vec<(&Galaxy, &Galaxy)> = vec![];

    for (i, g) in galaxy_chart.chart.iter().enumerate() {
        for h in galaxy_chart.chart[i + 1..].iter() {
            galaxy_pairs.push((g, h));
        }
    }

    let mut solution = 0;
    for (g_1, g_2) in galaxy_pairs {
        // println!(
        //     "{:?} - {:?} : {}",
        //     g_1,
        //     g_2,
        //     (g_2.coord.0 - g_1.coord.0).abs() + (g_2.coord.1 - g_1.coord.1).abs()
        // );
        solution += (g_2.coord.0 - g_1.coord.0).abs() + (g_2.coord.1 - g_1.coord.1).abs()
    }

    solution
}
#[derive(Debug)]
struct GalaxyChart {
    chart: Vec<Galaxy>,
}

impl GalaxyChart {
    fn new() -> Self {
        Self { chart: vec![] }
    }

    fn chart(&mut self, observations: &str, expantion_factor: i64) {
        let mut row_expantion_counter = 0;

        for (r, i) in observations.split("\n").zip(0..) {
            // println!("{r}");
            if r.trim().chars().all(|x| x == '.') {
                row_expantion_counter += expantion_factor;
            } else {
                for (c, j) in r.trim().chars().zip(0..) {
                    if c == '#' {
                        self.chart.push(Galaxy {
                            coord: (i + row_expantion_counter, j),
                        })
                    }
                }
            }
        }

        let mut column_expantion_counter = 0;

        let max_column_num = self.chart.iter().max_by_key(|f| f.coord.1).unwrap().coord.1;

        self.chart.sort_by_key(|f| f.coord.0);

        for c in (0..max_column_num).rev() {
            if !self.chart.iter().any(|f| f.coord.1 == c) {
                self.chart
                    .iter_mut()
                    .filter(|f| f.coord.1 > c)
                    .for_each(|g| g.coord.1 += expantion_factor);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Galaxy {
    coord: (i64, i64),
}

impl Galaxy {
    fn new(x: i64, y: i64) -> Self {
        Self { coord: (x, y) }
    }
}

#[cfg(test)]
mod tests {
    use crate::Galaxy;
    use crate::GalaxyChart;

    #[test]
    fn test_part_1() {
        let input: &str = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";
        let solu = crate::christmas_machine(input, 1);

        assert_eq!(solu, 374);
    }
    #[test]
    fn test_part_2_10_times() {
        let input: &str = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";
        let solu = crate::christmas_machine(input, 9);

        assert_eq!(solu, 1030);
    }
    #[test]
    fn test_part_2_100_times() {
        let input: &str = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";
        let solu = crate::christmas_machine(input, 99);

        assert_eq!(solu, 8410);
    }

    #[test]
    fn test_expantion_row() {
        let input: &str = "#
        .
        #";

        let mut gc = crate::GalaxyChart::new();
        gc.chart(input, 1);

        assert_eq!(
            gc.chart,
            [Galaxy { coord: (0, 0) }, Galaxy { coord: (3, 0) }]
        );
    }

    #[test]
    fn test_expantion_col() {
        let input: &str = "#.#.#.#";

        let mut gc = crate::GalaxyChart::new();
        gc.chart(input, 1);

        assert_eq!(
            gc.chart,
            [
                Galaxy { coord: (0, 0) },
                Galaxy { coord: (0, 3) },
                Galaxy { coord: (0, 6) },
                Galaxy { coord: (0, 9) },
            ]
        );
    }

    #[test]
    fn test_part_1_simple() {
        let input: &str = "#..
        ...
        ..#";
        let solu = crate::christmas_machine(input, 1);

        assert_eq!(solu, 6);
    }

    #[test]
    fn test_part_1_simple_row() {
        let input: &str = "#
        .
        #";

        let solu = crate::christmas_machine(input, 1);

        assert_eq!(solu, 3);
    }

    #[test]
    fn test_part_1_simple_col() {
        let input: &str = "#..#";
        let solu = crate::christmas_machine(input, 1);

        assert_eq!(solu, 5);
    }

    #[test]
    fn test_part_1_multi_col() {
        let input: &str = "#..#..#";
        let solu = crate::christmas_machine(input, 1);

        assert_eq!(solu, 20);
    }

    #[test]
    fn test_part_many() {
        let input: &str = "#.#.#.#.#
        .........
        #.#.#.#.#
        ......... 
        #.#.#.#.#
        .........
        #.#.#.#.#";
        let solu = crate::christmas_machine(input, 1);

        assert_eq!(solu, 1710);
    }
}
