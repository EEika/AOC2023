use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();

    let pipe_chart = PipeChart::new(&puzzle_input);

    let chart_dimentions: (i32, i32) = (
        pipe_chart.chart.len() as i32,
        pipe_chart.chart.first().unwrap().len() as i32,
    );

    let mut next_node: &PipeNode = pipe_chart.get_start_node().unwrap();
    let mut prev_node: &PipeNode = pipe_chart.get_start_node().unwrap();

    let mut step_counter: i32 = 0;

    let mut pipes_in_the_loop: Vec<&PipeNode> = vec![];

    'outer: while step_counter < chart_dimentions.0 * chart_dimentions.1 {
        step_counter += 1;

        //Check north
        if next_node.pipe_coord.0 > 0 {
            let neighbor = &pipe_chart.chart[next_node.pipe_coord.0 as usize - 1]
                [next_node.pipe_coord.1 as usize];
            if [
                PipeType::NorthSouth,
                PipeType::SouthEast,
                PipeType::SouthWest,
                PipeType::Start,
            ]
            .contains(&neighbor.pipe_type)
                && [
                    PipeType::NorthSouth,
                    PipeType::NorthEast,
                    PipeType::NorthWest,
                    PipeType::Start,
                ]
                .contains(&next_node.pipe_type)
                && neighbor != prev_node
            {
                // print!("North");
                prev_node = next_node;
                pipes_in_the_loop.push(prev_node);
                next_node = neighbor;
                continue;
            }
        }

        //Check south
        if next_node.pipe_coord.0 < chart_dimentions.0 - 1 {
            let neighbor = &pipe_chart.chart[next_node.pipe_coord.0 as usize + 1]
                [next_node.pipe_coord.1 as usize];
            if [
                PipeType::NorthSouth,
                PipeType::NorthEast,
                PipeType::NorthWest,
                PipeType::Start,
            ]
            .contains(&neighbor.pipe_type)
                && [
                    PipeType::NorthSouth,
                    PipeType::SouthEast,
                    PipeType::SouthWest,
                    PipeType::Start,
                ]
                .contains(&next_node.pipe_type)
                && neighbor != prev_node
            {
                // print!("South");

                prev_node = next_node;
                pipes_in_the_loop.push(prev_node);
                next_node = neighbor;
                continue;
            }
        }

        //Check east
        if next_node.pipe_coord.1 < chart_dimentions.1 - 1 {
            let neighbor = &pipe_chart.chart[next_node.pipe_coord.0 as usize]
                [next_node.pipe_coord.1 as usize + 1];
            if [
                PipeType::NorthWest,
                PipeType::WestEast,
                PipeType::SouthWest,
                PipeType::Start,
            ]
            .contains(&neighbor.pipe_type)
                && [
                    PipeType::NorthEast,
                    PipeType::SouthEast,
                    PipeType::WestEast,
                    PipeType::Start,
                ]
                .contains(&next_node.pipe_type)
                && neighbor != prev_node
            {
                // print!("East");

                prev_node = next_node;
                pipes_in_the_loop.push(prev_node);
                next_node = neighbor;
                continue;
            }
        }

        //Check west
        if next_node.pipe_coord.1 > 0 {
            let neighbor = &pipe_chart.chart[next_node.pipe_coord.0 as usize]
                [next_node.pipe_coord.1 as usize - 1];
            if [
                PipeType::NorthEast,
                PipeType::SouthEast,
                PipeType::WestEast,
                PipeType::Start,
            ]
            .contains(&neighbor.pipe_type)
                && [
                    PipeType::NorthWest,
                    PipeType::WestEast,
                    PipeType::SouthWest,
                    PipeType::Start,
                ]
                .contains(&next_node.pipe_type)
                && neighbor != prev_node
            {
                // print!("West");

                prev_node = next_node;
                pipes_in_the_loop.push(prev_node);
                next_node = neighbor;
                continue;
            }
        }

        if next_node.pipe_type != PipeType::Start {
            break 'outer;
        };
    }

    let mut left_hand_loop = true;
    let mut right_hand_loop = true;
    let mut right_hand_nodes: Vec<&PipeNode> = vec![];
    let mut left_hand_nodes: Vec<&PipeNode> = vec![];

    for i in 0..pipes_in_the_loop.len() - 2 {
        let node = pipes_in_the_loop[i];
        let pipe_direction = (
            pipes_in_the_loop[i + 1].pipe_coord.0 - pipes_in_the_loop[i].pipe_coord.0,
            pipes_in_the_loop[i + 1].pipe_coord.1 - pipes_in_the_loop[i].pipe_coord.1,
        );

        let right_hand_direciton = (pipe_direction.1, 0 - pipe_direction.0);
        let left_hand_direciton = (0 - pipe_direction.1, pipe_direction.0);

        let node_coord = node.pipe_coord;

        // check right hand
        if right_hand_loop {
            for i in 1.. {
                let coord = (
                    node_coord.0 + right_hand_direciton.0 * i,
                    node_coord.1 + right_hand_direciton.1 * i,
                );

                if coord.0 < 0
                    || coord.1 < 0
                    || coord.0 >= chart_dimentions.0
                    || coord.1 >= chart_dimentions.1
                {
                    right_hand_loop = false;
                    break;
                }

                let neighbor_pipe = &pipe_chart.chart[coord.0 as usize][coord.1 as usize];

                if pipes_in_the_loop.contains(&neighbor_pipe) {
                    break;
                }

                //check surrounding
                for x in 0..3 {
                    for y in 0..3 {
                        if !right_hand_nodes.contains(
                            &&pipe_chart.chart[if coord.0 > 0 {
                                (coord.0 as usize + x - 1).min(chart_dimentions.0 as usize - 1)
                            } else {
                                0
                            }][if coord.1 > 0 {
                                (coord.1 as usize + y - 1).min(chart_dimentions.1 as usize - 1)
                            } else {
                                0
                            }],
                        ) && !pipes_in_the_loop.contains(
                            &&pipe_chart.chart[if coord.0 > 0 {
                                (coord.0 as usize + x - 1).min(chart_dimentions.0 as usize - 1)
                            } else {
                                0
                            }][if coord.1 > 0 {
                                (coord.1 as usize + y - 1).min(chart_dimentions.1 as usize - 1)
                            } else {
                                0
                            }],
                        ) {
                            right_hand_nodes.push(
                                &pipe_chart.chart[if coord.0 > 0 {
                                    (coord.0 as usize + x - 1).min(chart_dimentions.0 as usize - 1)
                                } else {
                                    0
                                }][if coord.1 > 0 {
                                    (coord.1 as usize + y - 1).min(chart_dimentions.1 as usize - 1)
                                } else {
                                    0
                                }],
                            );
                        }
                    }
                }
            }
        }

        // check right hand
        if left_hand_loop {
            for i in 1.. {
                let coord = (
                    node_coord.0 + left_hand_direciton.0 * i,
                    node_coord.1 + left_hand_direciton.1 * i,
                );

                if coord.0 < 0
                    || coord.1 < 0
                    || coord.0 >= chart_dimentions.0
                    || coord.1 >= chart_dimentions.1
                {
                    left_hand_loop = false;
                    break;
                }

                let neighbor_pipe = &pipe_chart.chart[coord.0 as usize][coord.1 as usize];

                if pipes_in_the_loop.contains(&neighbor_pipe) {
                    break;
                }

                //check surrounding
                for x in 0..3 {
                    for y in 0..3 {
                        if !left_hand_nodes.contains(
                            &&pipe_chart.chart[if coord.0 > 0 {
                                (coord.0 as usize + x - 1).min(chart_dimentions.0 as usize - 1)
                            } else {
                                0
                            }][if coord.1 > 0 {
                                (coord.1 as usize + y - 1).min(chart_dimentions.1 as usize - 1)
                            } else {
                                0
                            }],
                        ) && !pipes_in_the_loop.contains(
                            &&pipe_chart.chart[if coord.0 > 0 {
                                (coord.0 as usize + x - 1).min(chart_dimentions.0 as usize - 1)
                            } else {
                                0
                            }][if coord.1 > 0 {
                                (coord.1 as usize + y - 1).min(chart_dimentions.1 as usize - 1)
                            } else {
                                0
                            }],
                        ) {
                            left_hand_nodes.push(
                                &pipe_chart.chart[if coord.0 > 0 {
                                    (coord.0 as usize + x - 1).min(chart_dimentions.0 as usize - 1)
                                } else {
                                    0
                                }][if coord.1 > 0 {
                                    (coord.1 as usize + y - 1).min(chart_dimentions.1 as usize - 1)
                                } else {
                                    0
                                }],
                            );
                        }
                    }
                }
            }
        }
    }

    println!("");
    println!("Number of steps: {step_counter}");
    println!("Solution, part 1: {}", step_counter / 2);
    println!("");
    if right_hand_loop {
        println!("Right hand inside: {}", right_hand_nodes.len());
    } else if left_hand_loop {
        println!("Left hand inside: {}", left_hand_nodes.len());
    }
}

struct PipeChart {
    chart: Vec<Vec<PipeNode>>,
}

impl PipeChart {
    fn new(chart_str: &str) -> Self {
        Self {
            chart: PipeChart::parse_chart(chart_str),
        }
    }

    fn parse_chart(chart_str: &str) -> Vec<Vec<PipeNode>> {
        let mut pipe_chart: Vec<Vec<PipeNode>> = vec![];

        for (line, row_index) in chart_str.split("\n").zip(0..) {
            let mut pipe_line: Vec<PipeNode> = vec![];

            // println!("{line}");
            for (p, col_index) in line.chars().zip(0..) {
                let new_node = PipeNode::new(p, (row_index, col_index));
                pipe_line.push(new_node);
            }
            pipe_chart.push(pipe_line);
        }

        pipe_chart
    }

    fn get_start_node(&self) -> Option<&PipeNode> {
        for row in self.chart.iter() {
            for node in row.iter() {
                if node.pipe_type == PipeType::Start {
                    return Some(node);
                };
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PipeNode {
    pipe_coord: (i32, i32),
    pipe_type: PipeType,
}

impl PipeNode {
    fn new(pipe_char: char, coord: (i32, i32)) -> PipeNode {
        PipeNode {
            pipe_type: PipeType::identify_pipe_type(pipe_char),
            pipe_coord: coord,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PipeType {
    WestEast,
    NorthWest,
    NorthEast,
    NorthSouth,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl PipeType {
    fn identify_pipe_type(pipe: char) -> PipeType {
        match pipe {
            '-' => return PipeType::WestEast,
            'J' => return PipeType::NorthWest,
            'L' => return PipeType::NorthEast,
            '|' => return PipeType::NorthSouth,
            '7' => return PipeType::SouthWest,
            'F' => return PipeType::SouthEast,
            'S' => return PipeType::Start,
            _ => return PipeType::Ground,
        }
    }
}
