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
        let regex = Regex::new(r"
(?P<name>[a-z-])
-
(?P<sector_id>[0-9]+)
\[(?P<checksum>[a-z]{5})\]
").expect("Regex is invalid");

        let caps = regex.captures(description).expect("Can't capture");

        Room {
            name: caps.name("name").expect("No name").to_string(),
            sector_id: caps.name("sector_id").expect("No sector id").parse().expect("Can't parse"),
            checksum: caps.name("checksum").expect("No checksum").to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extraction() {
        let description = "aaaaa-bbb-z-y-x-123[abxyz]";
        let room = Room::new(description);

        assert_eq!(room.name, "aaaaa-bbb-z-y-x");
        assert_eq!(room.sector_id, 123);
        assert_eq!(room.checksum, "abxyz");
    }
}
