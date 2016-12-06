extern crate md5;

pub fn puzzle(door_id: &str) -> String {
    let mut answer = String::new();
    let mut index = 0;

    for _ in 0..9 {
        let (new_index, next_char) = next_interesting_md5(door_id, index);
        index = new_index + 1;
        answer.push(next_char);
    }
    answer
}

pub fn next_interesting_md5(door_id: &str, mut index: u32) -> (u32, char) {
    loop {
        if index % 1000000 == 0 {
            println!("at index {}", index);
        }
        let candidate = format!("{}{}", door_id, index);

        let digest = md5::compute(candidate.as_bytes());

        if digest.starts_with(b"00000") {
            return (index, *digest.iter().nth(5).expect("no 5th char") as char)
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
