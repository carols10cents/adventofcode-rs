use std::collections::HashMap;

pub fn puzzle(input: &str) -> String {
    String::from("no")
}

pub struct Column {
    freqs: HashMap<char, u32>,
}

impl Column {
    fn add_char(&mut self, ch: char) {
        let count = self.freqs.entry(ch).or_insert(0);
        *count += 1;
    }

    fn most_common_char(&self) -> char {
        let mut f: Vec<(&char, &u32)> = self.freqs.iter().collect();
        f.sort_by(|a, b| b.1.cmp(&a.1));
        *f[0].0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn most_common_char_seen_in_column() {
        let mut c = Column { freqs: HashMap::new() };
        c.add_char('a');
        c.add_char('b');
        c.add_char('a');
        assert_eq!(c.most_common_char(), 'a');
    }
}
