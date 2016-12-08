use std::fmt;
use std::iter::FromIterator;

pub fn puzzle(input: &str) -> u32 {
    0
}

#[derive(Debug, PartialEq)]
pub struct Screen {
    display: Vec<Vec<char>>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        Screen {
            display: vec![vec!['.'; width]; height],
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.display.iter() {
            write!(f, "{}\n", String::from_iter(row.iter().cloned()))?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_screen(){
        let s = Screen::new(1, 2);
        assert_eq!(s, Screen {
            display: vec![vec!['.'], vec!['.']],
        });
        assert_eq!(s.to_string(), ".\n.\n");
    }

}
