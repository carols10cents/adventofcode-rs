use std::collections::HashMap;

pub fn puzzle(input: &str) -> String {
    let mut lines = input.lines();
    let first_line = lines.next().expect("There were no lines!");

    let mut columns: Vec<Column> = first_line.chars().map(|ch| {
        let mut c = Column { freqs: HashMap::new() };
        c.add_char(ch);
        c
    }).collect();

    for line in lines {
        for (i, ch) in line.chars().enumerate() {
            columns[i].add_char(ch);
        }
    }

    let mut answer = String::new();
    for col in columns {
        answer.push(col.least_common_char())
    }
    answer
}

pub struct Column {
    freqs: HashMap<char, u32>,
}

impl Column {
    fn add_char(&mut self, ch: char) {
        let count = self.freqs.entry(ch).or_insert(0);
        *count += 1;
    }

    fn least_common_char(&self) -> char {
        let mut f: Vec<(&char, &u32)> = self.freqs.iter().collect();
        f.sort_by(|a, b| a.1.cmp(&b.1));
        *f[0].0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn least_common_char_seen_in_column() {
        let mut c = Column { freqs: HashMap::new() };
        c.add_char('a');
        c.add_char('b');
        c.add_char('a');
        assert_eq!(c.least_common_char(), 'b');
    }

    #[test]
    fn sample() {
        let input = "\
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";
        assert_eq!(puzzle(input), String::from("advent"));
    }
}
