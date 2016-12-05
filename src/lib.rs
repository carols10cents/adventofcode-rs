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
        let candidate = format!("{}{}", door_id, index);

        let digest = String::from_utf8(
            md5::compute(candidate.as_bytes()).into_iter().cloned().collect()
        ).expect("creating string failed");

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

}
