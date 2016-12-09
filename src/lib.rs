
pub fn puzzle(input: &str) -> usize {
    decompressed_length(input)
}

pub fn decompressed_length(input: &str) -> usize {
    let mut chars = input.chars();
    let mut answer = 0;

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

            let mut further_decompress = String::new();
            for _ in 0..num_chars {
                further_decompress.push(chars.next().expect("no next chars"));
            }
            let further_decompress_length = decompressed_length(&further_decompress);
            answer += further_decompress_length * num_times;
        }
        else {
            answer += 1;
        }
    }
    answer
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
    fn decompressed_twice() {
        assert_eq!(decompressed_length("X(8x2)(3x3)ABCY"), 20);
    }

    #[test]
    fn lots_of_a() {
        assert_eq!(decompressed_length("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
    }

    #[test]
    fn variety() {
        assert_eq!(decompressed_length("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"), 445);
    }
}
