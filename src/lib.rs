use std::collections::HashSet;

extern crate md5;

pub fn puzzle(door_id: &str) -> String {
    let door_id = door_id.trim();

    let mut seen_indices = HashSet::new();
    let mut answer = vec![' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    let mut index = 0;

    while seen_indices.len() < 8 {
        let (new_index, char_index, next_char) = next_interesting_md5(door_id, index);
        index = new_index + 1;

        if answer[char_index] == ' ' {
            answer[char_index] = next_char;
            seen_indices.insert(char_index);

            println!("found {} char, answer[{}] = {}", seen_indices.len(), char_index, next_char);
            println!("answer is now {:?}", answer);
        }
    }

    let mut ans_str = String::new();
    for c in answer {
        ans_str.push(c);
    }
    ans_str
}

pub fn next_interesting_md5(door_id: &str, mut index: u32) -> (u32, usize, char) {
    let mut digest = String::with_capacity(2 * 16);

    loop {
        if index % 1000000 == 0 {
            println!("at index {}", index);
        }
        let candidate = format!("{}{}", door_id, index);
        digest.clear();

        for b in &md5::compute(candidate.as_bytes())[..] {
            digest.push_str(&format!("{:02x}", b));
        }

        if digest.starts_with("00000") {
            let mut chars = digest.chars();
            let char_index = chars.nth(5).expect("no 5th char");

            if is_valid_index(char_index) {
                let mut s = String::new();
                s.push(char_index);
                return (
                    index,
                    s.parse().expect("can't parse"),
                    chars.next().expect("no 6th char")
                )
            } else {
                index += 1;
            }
        } else {
            index += 1;
        }
    }
}

pub fn is_valid_index(candidate: char) -> bool {
    match candidate {
        '0'...'7' => true,
        _ => false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn once() {
    //     assert_eq!(next_interesting_md5("abc", 0), (3231929, 1, '5'));
    // }

    #[test]
    fn valid_index() {
        assert!( ! is_valid_index('9') );
        assert!( ! is_valid_index('8') );
        assert!(is_valid_index('7'));
    }

    #[test]
    fn sample() {
        assert_eq!(puzzle("abc"), String::from("05ace8e3"));
    }

}
