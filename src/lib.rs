pub fn distance(input: &str) -> u32 {
    let mut chars = input.chars();
    if let (Some(turn_dir), Some(num_blocks)) = (chars.next(), chars.next()) {
        let num_blocks: u32 = num_blocks.to_string()
                                        .parse()
                                        .expect("Cannot parse");
        return num_blocks
    }
    panic!("oh no!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn right_one_is_one() {
        assert_eq!(distance("R1"), 1);
    }

    #[test]
    fn r1_r1_is_two() {
        assert_eq!(distance("R1 R1"), 2);
    }

}