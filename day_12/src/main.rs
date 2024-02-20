#[derive(Debug, Clone, Copy)]
struct SpringElement {
    element_type: SpringTypes,
    length: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringTypes {
    Operational,
    Damaged,
    Unknown,
}
fn main() {
    let p_i = &std::fs::read_to_string("puzzle_input.txt").unwrap()[..];

    println!("{p_i}");
}

fn parse_spring_line(spring_line: &str) -> (Vec<SpringElement>, Vec<u32>) {
    let (marks, broken_springs) = spring_line.split_once(" ").unwrap();

    let broken_array: Vec<u32> = broken_springs
        .split(',')
        .map(|f| f.parse::<u32>().unwrap())
        .collect();

    let mut spring_elements: Vec<SpringElement> = vec![];

    let (first_mark, marks) = (&marks[0..1], &marks[1..]);

    let mut current_spring_element: SpringElement = SpringElement {
        element_type: (match first_mark {
            "?" => SpringTypes::Unknown,
            "." => SpringTypes::Operational,
            _ => SpringTypes::Damaged,
        }),
        length: (1),
    };

    for mark in marks.chars() {
        match (mark, current_spring_element.element_type) {
            ('?', SpringTypes::Unknown) => {
                current_spring_element.length += 1;
            }
            ('?', _) => {
                spring_elements.push(current_spring_element.clone());
                current_spring_element.length = 1;
                current_spring_element.element_type = SpringTypes::Unknown
            }
            ('.', SpringTypes::Operational) => {
                current_spring_element.length += 1;
            }
            ('.', _) => {
                spring_elements.push(current_spring_element.clone());
                current_spring_element.length = 1;
                current_spring_element.element_type = SpringTypes::Operational
            }
            ('#', SpringTypes::Damaged) => {
                current_spring_element.length += 1;
            }
            ('#', _) => {
                spring_elements.push(current_spring_element.clone());
                current_spring_element.length = 1;
                current_spring_element.element_type = SpringTypes::Damaged
            }
            (_, _) => {}
        }

        // println!("{mark}");
    }
    spring_elements.push(current_spring_element.clone());

    (spring_elements, broken_array)
}

fn evaluate_spring_line(mut spring_line: Vec<SpringElement>, mut broken_array: Vec<u32>) -> u32 {
    println!("{:?}", spring_line);

    //Remove Operational - Left
    if spring_line.first().unwrap().element_type == SpringTypes::Operational {
        spring_line.remove(0);
    }
    println!("{:?}", spring_line);

    //Remove Operational - Right
    if spring_line.last().unwrap().element_type == SpringTypes::Operational {
        spring_line.pop();
    }
    println!("{:?}", spring_line);

    //Check left - exact
    if spring_line.get(0).unwrap().element_type == SpringTypes::Damaged
        && spring_line.get(1).unwrap().element_type == SpringTypes::Operational
        && spring_line.first().unwrap().length == *broken_array.first().unwrap()
    {
        spring_line.remove(0);
        broken_array.remove(0);
    }
    println!("{:?}", spring_line);

    //Check right - exact
    if spring_line
        .get(spring_line.len().wrapping_sub(1))
        .unwrap()
        .element_type
        == SpringTypes::Damaged
        && spring_line
            .get(spring_line.len().wrapping_sub(2))
            .unwrap()
            .element_type
            == SpringTypes::Operational
        && spring_line.last().unwrap().length == *broken_array.last().unwrap()
    {
        spring_line.pop();
        broken_array.pop();
    }
    println!("{:?}", spring_line);

    0
}

#[cfg(test)]
mod tests {
    use crate::{evaluate_spring_line, parse_spring_line};

    #[test]
    fn group_1_2_3() {
        let spring_line = "..???.###.. 1,1,3";

        let (s, a) = parse_spring_line(spring_line);

        evaluate_spring_line(s, a);

        let solu = 0;
        assert_eq!(solu, 1);
    }
}
