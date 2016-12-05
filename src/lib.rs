
pub fn puzzle(input: &str) -> u32 {
    0
}

pub fn is_a_triangle(a: u32, b: u32, c: u32) -> bool {
    a + b > c && b + c > a && c + a > b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn equilateral_triangle() {
        assert!(is_a_triangle(3, 3, 3));
    }

    #[test]
    fn not_a_triangle() {
        assert!( ! is_a_triangle(5, 10, 25) );
        assert!( ! is_a_triangle(5, 25, 10) );
        assert!( ! is_a_triangle(10, 5, 25) );
        assert!( ! is_a_triangle(10, 25, 5) );
        assert!( ! is_a_triangle(25, 5, 10) );
        assert!( ! is_a_triangle(25, 10, 5) );
    }
}
