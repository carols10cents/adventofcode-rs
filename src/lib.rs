
pub fn puzzle(door_id: &str) -> String {
    let answer = String::new();
    let mut index = 0;

    for _ in 0..9 {
        let (new_index, next_char) = next_interesting_md5(door_id, index);
        index = new_index;
        answer.push(next_char);
    }
    answer
}

#[cfg(test)]
mod test {
    use super::*;

}
