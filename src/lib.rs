
pub fn puzzle(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        if supports_tls(line) {
            total += 1;
        }
    }
    total
}

pub fn supports_tls(candidate: &str) -> bool {
    println!("considering {}", candidate);
    let mut start_index = 0;
    let end_index = candidate.len();

    let mut abba_outside = false;

    for (i, c) in candidate.match_indices(|c| c == '[' || c == ']') {
        println!("looking at slice ending in {}, [{}..{}]", c, start_index, i);
        println!("slice is {}", &candidate[start_index..i]);
        println!("abba_outside = {}", abba_outside);
        if c == "[" {
            abba_outside = abba_outside || contains_abba(&candidate[start_index..i]);
            println!("after looking, abba_outside = {}", abba_outside);
        } else {
            if contains_abba(&candidate[start_index..i]) {
                println!("womp womp, found an abba inside");
                return false;
            }
        }
        start_index = i + 1;
        println!("start_index = {}", start_index);
    }
    println!("looking at final slice [{}..{}]: '{}", start_index, end_index, &candidate[start_index..end_index]);

    abba_outside = abba_outside || contains_abba(&candidate[start_index..end_index]);

    println!("final answer: {}", abba_outside);

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

pub fn supports_ssl(candidate: &str) -> bool {
    let mut start_index = 0;
    let end_index = candidate.len();

    let mut potential_abas: Vec<&str> = vec![];
    let mut potential_babs = vec![];

    for (i, c) in candidate.match_indices(|c| c == '[' || c == ']') {
        let mut f = find_abas(&candidate[start_index..i]);
        if c == "[" {
            potential_abas.append(&mut f);
        } else {
            potential_babs.append(&mut f);
        }
        start_index = i + 1;
    }
    let mut f = find_abas(&candidate[start_index..end_index]);
    potential_abas.append(&mut f);

    for aba in potential_abas {
        let mut by_chars = aba.chars();
        let first = by_chars.next().expect("no first char");
        let second = by_chars.next().expect("no second char");
        let bab = format!("{}{}{}", second, first, second);
        if potential_babs.contains(&&bab[..]) {
            return true;
        }
    }
    false
}

fn find_abas(s: &str) -> Vec<&str> {
   let mut abas: Vec<&str> = vec![];

   let by_chars: Vec<char> = s.chars().collect();

   for (i, &c) in by_chars.iter().enumerate() {
       if i + 2 < by_chars.len() &&
           c != by_chars[i + 1] &&
           by_chars[i + 2] == c {
           abas.push(&s[i..i+3]);
       }
   }

   abas
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn does_support_ssl() {
        assert!(supports_ssl("aba[bab]xyz"));
        assert!(supports_ssl("aaa[kek]eke"));
        assert!(supports_ssl("zazbz[bzb]cdb"));
        assert!(supports_ssl("zazaaabz[bzb]cdb[foo]zbz"));
    }

    #[test]
    fn does_not_support_ssl() {
        assert!( ! supports_ssl("xyx[xyx]xyx") );
    }

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
