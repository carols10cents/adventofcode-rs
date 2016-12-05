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
        let digest = md5::compute(candidate);
        if digest.starts_with("00000") {
            return (index, digest.char_at(5))
        } else {
            index += 1;
        }
    }

    (0, 'a')
}

#[cfg(test)]
mod test {
    use super::*;

}
