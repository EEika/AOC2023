fn main() {
    let p_i = &std::fs::read_to_string("puzzle_input.txt").unwrap()[..];

    println!("{p_i}");
}

fn parse_spring_line(spring_line: &str) -> Vec<SpringElement> {
    let (marks, springs) = spring_line.split_once(" ").unwrap();

    let mut spring_elements: Vec<SpringElement> = vec![];

    let mut current_spring_element: SpringElement;

    for mark in marks.chars() {
        println!("{mark}");
    }

    spring_elements
}

struct SpringElement {
    elementType: SpringTypes,
    length: u32,
}

enum SpringTypes {
    Operational,
    Damaged,
    Unknown,
}
#[cfg(test)]
mod tests {
    use crate::parse_spring_line;

    #[test]
    fn group_1_2_3() {
        let spring_line = "???.### 1,1,3";

        parse_spring_line(spring_line);

        let solu = 0;
        assert_eq!(solu, 1);
    }
}
