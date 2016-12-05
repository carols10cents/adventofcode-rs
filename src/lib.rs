use std::collections::HashMap;
use std::cmp::Ordering;

extern crate regex;

use regex::Regex;

pub fn puzzle(input: &str) -> u32 {
    0
}

pub struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn new(description: &str) -> Room {
        let regex = Regex::new(r"(?x)
            (?P<name>[a-z-]+)
            -
            (?P<sector_id>[0-9]+)
            \[(?P<checksum>[a-z]{5})\]").expect("Regex is invalid");

        let caps = regex.captures(description).expect("Can't capture");

        Room {
            name: caps.name("name").expect("No name").to_string(),
            sector_id: caps.name("sector_id").expect("No sector id").parse().expect("Can't parse"),
            checksum: caps.name("checksum").expect("No checksum").to_string(),
        }
    }

    fn is_real(&self) -> bool {
        self.checksum == self.computed_checksum()
    }

    fn computed_checksum(&self) -> String {
        let mut most_common = self.five_most_common_chars();
        most_common.sort();
        let mut s = String::new();
        for &c in most_common.iter() {
            s.push(c);
        }
        s
    }

    fn five_most_common_chars(&self) -> Vec<char> {
        self.chars_by_frequency_desc().into_iter().take(5).collect()
    }

    fn chars_by_frequency_desc(&self) -> Vec<char> {
        let mut tuples: Vec<(char, u32)> = self.chars_by_frequency()
                                               .into_iter()
                                               .collect();
        tuples.sort_by(|a, b| {
            match b.1.cmp(&a.1) {
                Ordering::Equal => a.0.cmp(&b.0),
                other => other,
            }
        });
        tuples.iter().map( |&(k, _)| k ).collect()
    }

    fn chars_by_frequency(&self) -> HashMap<char, u32> {
        let mut freqs = HashMap::new();
        for c in self.name.chars() {
            if c != '-' {
                let count = freqs.entry(c).or_insert(0);
                *count += 1;
            }
        }
        freqs
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_extraction() {
        let description = "aaaaa-bbb-z-y-x-123[abxyz]";
        let room = Room::new(description);

        assert_eq!(room.name, "aaaaa-bbb-z-y-x");
        assert_eq!(room.sector_id, 123);
        assert_eq!(room.checksum, "abxyz");
    }

    #[test]
    fn test_chars_by_freq() {
        let room = Room::new("aaaaa-bbb-z-y-x-123[abxyz]");
        let mut expected = HashMap::new();
        expected.insert('a', 5);
        expected.insert('b', 3);
        expected.insert('z', 1);
        expected.insert('y', 1);
        expected.insert('x', 1);
        assert_eq!(room.chars_by_frequency(), expected);
    }

    #[test]
    fn test_chars_by_freq_desc() {
        let room = Room::new("aaaaa-bbb-z-y-x-123[abxyz]");
        assert_eq!(
            room.chars_by_frequency_desc(),
            vec!['a', 'b', 'x', 'y', 'z']
        );
    }

    #[test]
    fn test_five_most_common_chars() {
        let room = Room::new("aaaaa-bbb-z-y-x-123[abxyz]");
        assert_eq!(
            room.chars_by_frequency_desc(),
            vec!['a', 'b', 'x', 'y', 'z']
        );
    }

    #[test]
    fn real_rooms() {
        assert!(Room::new("aaaaa-bbb-z-y-x-123[abxyz]").is_real());
        assert!(Room::new("a-b-c-d-e-f-g-h-987[abcde]").is_real());
        assert!(Room::new("not-a-real-room-404[oarel]").is_real());
    }

    #[test]
    fn decoy_rooms() {
        assert!( ! Room::new("totally-real-room-200[decoy]").is_real() );
    }
}
