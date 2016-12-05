use std::cmp;

pub struct Keypad {
    width: usize,
    height: usize,
    layout: Vec<Vec<char>>,
    x: usize,
    y: usize,
}

impl Keypad {
    pub fn new(layout_description: &str) -> Keypad {
        let layout: Vec<Vec<char>> = layout_description
                         .lines()
                         .map(|line| line.chars().collect::<Vec<_>>() )
                         .collect();

        let mut x = 0;
        let mut y = 0;

        for (yi, yval) in layout.iter().enumerate() {
            for (xi, &xval) in yval.iter().enumerate() {
                if xval == '5' {
                    x = xi;
                    y = yi;
                    break;
                }
            }
        }

        Keypad {
            width: layout[0].len(),
            height: layout.len(),
            layout: layout,
            x: x,
            y: y,
        }
    }

    fn move_a_key(&mut self, instruction: char) {
        println!("{} {}", self.current_value(), instruction);
        match instruction {
            'U' => self.y = self.y.saturating_sub(1),
            'D' => self.y = cmp::min(self.height - 1, self.y + 1),
            'L' => self.x = self.x.saturating_sub(1),
            'R' => self.x = cmp::min(self.width - 1, self.x + 1),
            other => panic!("Don't know how to move ({})!", other),
        }
        println!("({}, {}) = {}", self.x, self.y, self.current_value());
    }

    fn current_value(&self) -> char {
        self.layout[self.y][self.x]
    }
}

pub fn puzzle(keypad: &mut Keypad, input: &str) -> String {
    let mut answer = String::new();
    for line in input.lines() {
        for c in line.chars() {
            keypad.move_a_key(c);
        }
        answer.push(keypad.current_value());
    }
    answer
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small_keypad_construction() {
        let keypad = Keypad::new("1");
        assert_eq!(keypad.width, 1);
        assert_eq!(keypad.height, 1);
        assert_eq!(keypad.current_value(), '1');
    }

    #[test]
    fn tall_keypad_construction() {
        let keypad = Keypad::new("1\n2\n3");
        assert_eq!(keypad.width, 1);
        assert_eq!(keypad.height, 3);
        assert_eq!(keypad.current_value(), '1');
    }

    #[test]
    fn desired_keypad_construction() {
        let keypad = Keypad::new("123\n456\n789");
        assert_eq!(keypad.width, 3);
        assert_eq!(keypad.height, 3);
        assert_eq!(keypad.current_value(), '5');
    }

    #[test]
    fn regular_move() {
        let mut keypad = Keypad::new("123\n456\n789");

        keypad.move_a_key('U');
        assert_eq!(keypad.current_value(), '2');
    }

    #[test]
    fn ignored_move() {
        let mut keypad = Keypad::new("123\n456\n789");

        keypad.move_a_key('U');
        keypad.move_a_key('U');
        assert_eq!(keypad.current_value(), '2');
    }

    #[test]
    fn overall_puzzle() {
        let mut keypad = Keypad::new("123\n456\n789");
        let input = "ULL\nRRDDD\nLURDL\nUUUUD\n";
        let answer = puzzle(&mut keypad, input);
        assert_eq!(String::from("1985"), answer);
    }
}
