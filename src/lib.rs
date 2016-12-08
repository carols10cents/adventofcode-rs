
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_screen(){
        let s = Screen::new(1, 2);
        assert_eq!(s, Screen {
            display: vec![vec!['.'], vec!['.']],
        });
    }

}
