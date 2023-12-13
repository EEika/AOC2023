use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();

    let pipe_chart = PipeChart::new(&puzzle_input);

    let chart_dimentions = (
        pipe_chart.chart.len(),
        pipe_chart.chart.first().unwrap().len(),
    );

    let mut next_node: &PipeNode = pipe_chart.get_start_node().unwrap();
    let mut prev_node: &PipeNode = pipe_chart.get_start_node().unwrap();

    let mut step_counter = 0;

    let mut pipes_in_the_loop: Vec<&PipeNode> = vec![];

    'outer: while step_counter < chart_dimentions.0 * chart_dimentions.1 {
        step_counter += 1;

        //Check north
        if next_node.pipe_coord.0 > 0 {
            let neighbor = &pipe_chart.chart[next_node.pipe_coord.0 - 1][next_node.pipe_coord.1];
            if [
                PipeType::NorthSouth,
                PipeType::SouthEast,
                PipeType::SouthWest,
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
            let neighbor = &pipe_chart.chart[next_node.pipe_coord.0 + 1][next_node.pipe_coord.1];
            if [
                PipeType::NorthSouth,
                PipeType::NorthEast,
                PipeType::NorthWest,
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
            let neighbor = &pipe_chart.chart[next_node.pipe_coord.0][next_node.pipe_coord.1 + 1];
            if [PipeType::NorthWest, PipeType::WestEast, PipeType::SouthWest]
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
            let neighbor = &pipe_chart.chart[next_node.pipe_coord.0][next_node.pipe_coord.1 - 1];
            if [PipeType::NorthEast, PipeType::SouthEast, PipeType::WestEast]
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
    println!("The pipes in the loop: {:?}", pipes_in_the_loop);
    println!("");
    println!("Number of steps: {step_counter}");
    println!("Solution, part 1: {}", step_counter / 2);

    println!("End of pipe");
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

        for (row_index, line) in chart_str.split("\n").enumerate() {
            let mut pipe_line: Vec<PipeNode> = vec![];
            // println!("{line}");
            for (col_index, p) in line.chars().enumerate() {
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
    pipe_coord: (usize, usize),
    pipe_type: PipeType,
    pipe_status: PipeStatus,
}

impl PipeNode {
    fn new(pipe_char: char, coord: (usize, usize)) -> PipeNode {
        PipeNode {
            pipe_type: PipeType::identify_pipe_type(pipe_char),
            pipe_status: PipeStatus::Undetermined,
            pipe_coord: coord,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
enum PipeStatus {
    Connected,
    Disconnected,
    Undetermined,
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
