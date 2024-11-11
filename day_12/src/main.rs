#[allow(dead_code)]

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

fn evaluate_spring_line(mut spring_line: Vec<SpringElement>, mut broken_array: Vec<u32>, mut permutation_counter : u32) -> u32 {

    let mut marked_for_recursion = false;

    //Remove Operational group - Left
    if spring_line.first().unwrap().element_type == SpringTypes::Operational {
        spring_line.remove(0);
    }

    //Remove Operational - Right
    if spring_line.last().unwrap().element_type == SpringTypes::Operational {
        spring_line.pop();
    }
    println!("{:?}", spring_line);

    //Check left - Broken -> collapses next posibility
    if spring_line.get(0).unwrap().element_type == SpringTypes::Damaged
        //&& spring_line.get(1).unwrap().element_type == SpringTypes::Operational
        //&& spring_line.first().unwrap().length == *broken_array.first().unwrap()
    {
        println!("Broken left");
        marked_for_recursion = true;

        let mut broken_line_length = spring_line.first().unwrap().length;
        let mut broken_array_length = *broken_array.first().unwrap();

        while broken_line_length < broken_array_length{
            println!("Longer");
            broken_array_length -= broken_line_length;
            spring_line.remove(0);
            broken_line_length = spring_line.first().unwrap().length;
        }

        if broken_line_length == broken_array_length{
            spring_line.remove(0);
        }
        else {
            spring_line.first_mut().unwrap().length -= broken_array_length;
        }
        broken_array.remove(0);
    }
    println!("{:?}, {:?}", spring_line, broken_array);

    //Check right - Broken -> collapses next posibility
    if spring_line.last().unwrap().element_type == SpringTypes::Damaged
    {
        println!("Broken right");
        marked_for_recursion = true;

        let mut broken_line_length = spring_line.last().unwrap().length;
        let mut broken_array_length = *broken_array.last().unwrap();

        while broken_line_length < broken_array_length{
            println!("Longer");
            broken_array_length -= broken_line_length;
            spring_line.pop();
            broken_line_length = spring_line.last().unwrap().length;
        }

        if broken_line_length == broken_array_length{
            spring_line.pop();
        }
        else {
            spring_line.last_mut().unwrap().length -= broken_array_length;
        }
        broken_array.pop();
    }
    println!("{:?}, {:?}", spring_line, broken_array);

    if marked_for_recursion{
        permutation_counter = evaluate_spring_line(spring_line, broken_array, permutation_counter);
    }
    else if broken_array.len() == 1 {
        println!("Just one posibility");
        let mut seg_summer = 0;
        for seg in spring_line.into_iter().filter(|s| s.element_type == SpringTypes::Unknown){
            seg_summer += 1 + seg.length - broken_array[0];
        }
        permutation_counter *= seg_summer;
    }
    else if broken_array.len() == 2 && spring_line.iter().filter(|s| s.element_type == SpringTypes::Unknown).collect::<Vec<&SpringElement>>().len() == 1{
        let num_unknowns = spring_line.into_iter().filter(|s| s.element_type == SpringTypes::Unknown).collect::<Vec<SpringElement>>().first().unwrap().length;
        let sum = (1 ..= &num_unknowns - broken_array.iter().sum::<u32>()).fold(0, |a, b| b);
        println!("{}", sum );
    }
    else if broken_array.len() == 2{
        println!("Fractured 2");
        let mut seg_summer = 0;
        let mut working_line = spring_line.clone().into_iter().filter(|s| s.element_type == SpringTypes::Unknown);
        for seg in working_line{
            seg_summer += 1 + seg.length - broken_array[0];
        }
        //let num_unknowns = spring_line.into_iter().filter(|s| s.element_type == SpringTypes::Unknown).collect::<Vec<SpringElement>>().first().unwrap().length;
        //let sum = (1 ..= &num_unknowns - broken_array.iter().sum::<u32>()).fold(0, |a, b| b);
        //println!("{}", num_unknowns - broken_array[0], );
        println!("{}", 0 );
    }

    permutation_counter
}

fn collaps_to_valid_spot(spring_size : u32, spring_line : Vec<SpringElement>) -> Option<Vec<SpringElement>> {
    let mut spring_counter = 0;
    let mut working_spring_line = spring_line.clone();
    let mut working_spring_size = spring_size;
    for spring in spring_line.clone(){

        match spring.element_type{
            SpringTypes::Operational => {
                working_spring_size = spring_size;
                working_spring_line.remove(0);
            },
            SpringTypes::Damaged => {
                if working_spring_size > spring.length{
                    working_spring_size -= spring.length;
                    working_spring_line.remove(0);

                }
                else if working_spring_size < spring.length - 1{
                    working_spring_line.remove(0);
                    return Some(working_spring_line)
                }
            },
            SpringTypes::Unknown => {
                if working_spring_size > spring.length{
                    working_spring_size -= spring.length;
                    working_spring_line.remove(0);
                }
            }
        }
    }
    

    None
}
#[cfg(test)]
mod tests {
    use crate::{evaluate_spring_line, parse_spring_line};

    
    #[test]
    fn group_2_1() {
        let spring_line = ".#?.???.. 2,1";

        let (s, a) = parse_spring_line(spring_line);

        let mut solu = 1;

        solu = evaluate_spring_line(s, a, solu);

        assert_eq!(solu, 3);
    }

    #[test]
    fn group_1_1_3() {
        let spring_line = "..???.###.. 1,1,3";

        let (s, a) = parse_spring_line(spring_line);

        let mut solu = 1;

        solu = evaluate_spring_line(s, a, solu);

        assert_eq!(solu, 1);
    }
#[test]
    fn group_4_1_1() {
        let spring_line = "????.#...#... 4,1,1";

        let (s, a) = parse_spring_line(spring_line);

        let mut solu = 1;

        solu = evaluate_spring_line(s, a, solu);

        assert_eq!(solu, 1);
    }
#[test]
    fn group_1_6_5() {
        let spring_line = "????.######..#####. 1,6,5";

        let (s, a) = parse_spring_line(spring_line);

        let mut solu = 1;

        solu = evaluate_spring_line(s, a, solu);

        assert_eq!(solu, 4);
    }
    #[test]
    fn fractured_1_1_3() {
        let spring_line = ".??..??...?##. 1,1,3";

        let (s, a) = parse_spring_line(spring_line);

        let mut solu = 1;

        solu = evaluate_spring_line(s, a, solu);

        assert_eq!(solu, 4);
    }
} 
