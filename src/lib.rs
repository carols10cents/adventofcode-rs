
pub fn puzzle(input: &str) -> u32 {
    0
}

pub fn contains_abba(candidate: &str) -> bool {
    let by_chars: Vec<char> = candidate.chars().collect();

    for (i, &c) in by_chars.iter().enumerate() {
        if c != by_chars[i + 1] &&
            by_chars[i + 1] == by_chars[i + 2] &&
            by_chars[i + 3] == c {
            return true
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn does_contain_abba() {
        assert!(contains_abba("abba"));
        assert!(contains_abba("ioxxoj"));
    }

    #[test]
    fn does_not_contain_abba() {
        assert!( ! contains_abba("mnop") );
        assert!( ! contains_abba("aaaa") );
    }
}
