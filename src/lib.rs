
pub fn puzzle(input: &str) -> u32 {
    0
}

pub fn is_a_triangle(a: u32, b: u32, c: u32) -> bool {
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn equilateral_triangle() {
        assert!(is_a_triangle(3, 3, 3));
    }

}
