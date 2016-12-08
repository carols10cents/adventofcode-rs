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

    pub fn rect(&mut self, rect_width: usize, rect_height: usize) {
        for h in 0..rect_height {
            for w in 0..rect_width {
                self.display[h][w] = '#';
            }
        }
    }

    pub fn rotate_column(&mut self, col_index: usize, how_many: usize) {
        let col: Vec<_> = self.display.iter()
            .map(|row| row[col_index] ).collect();

        for i in 0..self.display.len() {
            let from_index = (i + self.display.len() - how_many) % self.display.len();
            self.display[i][col_index] = col[from_index];
        }
    }

    pub fn rotate_row(&mut self, row_index: usize, how_many: usize) {
        let row = self.display[row_index].clone();

        for i in 0..row.len() {
            let from_index = (i + row.len() - how_many) % row.len();
            self.display[row_index][i] = row[from_index];
        }
    }

    pub fn num_lit_pixels(&self) -> usize {
        self.display.iter().map(|row| {
            row.iter().filter(|&&c| c == '#').count()
        }).sum()
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

    #[test]
    fn rect_drawing() {
        let mut s = Screen::new(7, 3);
        s.rect(3, 2);
        assert_eq!(s.to_string(), "###....\n###....\n.......\n");
    }

    #[test]
    fn rotate_column() {
        let mut s = Screen::new(7, 3);
        s.rect(3, 2);
        s.rotate_column(1, 1);
        assert_eq!(s.to_string(), "#.#....\n###....\n.#.....\n");
    }

    #[test]
    fn rotate_row() {
        let mut s = Screen::new(7, 3);
        s.rect(3, 2);
        s.rotate_column(1, 1);
        s.rotate_row(0, 4);
        assert_eq!(s.to_string(), "....#.#\n###....\n.#.....\n");
    }

    #[test]
    fn count_pixels() {
        let mut s = Screen::new(7, 3);
        s.rect(3, 2);
        s.rotate_column(1, 1);
        s.rotate_row(0, 4);
        assert_eq!(s.num_lit_pixels(), 6);
    }
}
