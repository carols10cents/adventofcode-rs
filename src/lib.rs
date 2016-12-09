
pub fn puzzle(input: &str) -> usize {
    decompressed_length(input)
}

pub fn decompressed_length(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_change() {
        assert_eq!(decompressed_length("ADVENT"), 6);
    }

    #[test]
    fn one_char() {
        assert_eq!(decompressed_length("A(1x5)BC"), 7);
    }

    #[test]
    fn three_chars() {
        assert_eq!(decompressed_length("(3x3)XYZ"), 9);
    }

    #[test]
    fn two_chars_twice() {
        assert_eq!(decompressed_length("A(2x2)BCD(2x2)EFG"), 11);
    }
    #[test]
    fn no_double_processing() {
        assert_eq!(decompressed_length("(6x1)(1x3)A"), 6);
    }
    #[test]
    fn another_no_double_processing() {
        assert_eq!(decompressed_length("X(8x2)(3x3)ABCY"), 18);
    }
}
