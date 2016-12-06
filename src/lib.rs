extern crate md5;

pub fn puzzle(door_id: &str) -> String {
    let door_id = door_id.trim();

    let mut answer = String::new();
    let mut index = 0;

    for i in 0..9 {
        let (new_index, next_char) = next_interesting_md5(door_id, index);
        index = new_index + 1;
        answer.push(next_char);
        println!("found {} char: {}", i + 1, next_char);
    }
    answer
}

pub fn next_interesting_md5(door_id: &str, mut index: u32) -> (u32, char) {
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
            return (index, digest.chars().nth(5).expect("no 5th char"))
        } else {
            index += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn once() {
        assert_eq!(next_interesting_md5("abc", 0), (3231929, '1'));
    }

}
