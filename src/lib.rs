pub struct Keypad {
    width: usize,
    height: usize,
    layout: Vec<Vec<char>>,
    x: usize,
    y: usize,
}

impl Keypad {
    fn new(layout_description: &str) -> Keypad {
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

    fn current_value(&self) -> char {
        self.layout[self.y][self.x]
    }
}

pub fn puzzle(input: &str) -> String {
    let mut keypad = Keypad::new("1");
    let mut answer = String::new();

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
}
