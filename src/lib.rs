
pub fn puzzle(input: &str) -> usize {
    decompressed_length(input)
}

pub fn decompressed_length(input: &str) -> usize {
    let mut chars = input.chars();
    let mut decompressed_length = 0;

    while let Some(c) = chars.next() {
        if c == '(' {
            let mut num_chars = String::new();
            let mut next_char = chars.next().expect("nothing after open paren");
            while next_char != 'x' {
                num_chars.push(next_char);
                next_char = chars.next().expect("nothing after open paren");
            }
            let num_chars: usize = num_chars.parse().expect("couldn't parse number of chars");

            let mut num_times = String::new();

            next_char = chars.next().expect("nothing after x");
            while next_char != ')' {
                num_times.push(next_char);
                next_char = chars.next().expect("nothing after x");
            }
            let num_times: usize = num_times.parse().expect("couldn't parse number of times");

            for _ in 0..num_chars {
                chars.next();
            }
            decompressed_length += num_chars * num_times;
        }
        else {
            decompressed_length += 1;
        }
    }
    decompressed_length
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
