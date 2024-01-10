fn main() {
    let p_i = &std::fs::read_to_string("puzzle_input.txt").unwrap()[..];

    println!("{p_i}");
}

#[cfg(test)]
mod tests {

    #[test]
    fn group_1_2_3() {
        let solu = 0;
        assert_eq!(solu, 1);
    }
}
