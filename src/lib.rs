
pub fn puzzle(input: &str) -> u32 {
    0
}

pub fn supports_tls(candidate: &str) -> bool {
    false
}

pub fn contains_abba(candidate: &str) -> bool {
    let by_chars: Vec<char> = candidate.chars().collect();

    for (i, &c) in by_chars.iter().enumerate() {
        if i + 3 < by_chars.len() &&
            c != by_chars[i + 1] &&
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
    fn does_support_tls() {
        assert!(supports_tls("abba[mnop]qrst"));
        assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));
        assert!(supports_tls("abcd[efgh]ijk[lmn]oppo"))
    }

    #[test]
    fn does_not_support_tls() {
        assert!( ! supports_tls("abcd[bddb]xyyx") );
        assert!( ! supports_tls("aaaa[qwer]tyui") );
        assert!( ! supports_tls("aaaa[qwer]tyui[foof]poop") );
    }

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
