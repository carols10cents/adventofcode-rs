
pub fn puzzle(input: &str) -> u32 {
    0
}

pub fn supports_tls(candidate: &str) -> bool {
    let mut start_index = 0;
    let end_index = candidate.len();

    let mut abba_outside = false;

    while let Some((i, c)) = candidate.match_indices(|c| c == '[' || c == ']').next() {
        if c == "[" {
            abba_outside = abba_outside || contains_abba(&candidate[start_index..i]);
        } else {
            if contains_abba(&candidate[start_index..i]) {
                return false;
            }
        }
        start_index = i + 1;
    }

    abba_outside = abba_outside || contains_abba(&candidate[start_index..end_index]);

    abba_outside
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
