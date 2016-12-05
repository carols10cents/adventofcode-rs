use std::collections::HashMap;

struct Keypad {
    layout: HashMap<u8, Key>,
    current_value: u8,
}

impl Keypad {
    fn new() -> Keypad {
        let mut layout = HashMap::new();

        layout.insert(1, Key {
           value: 1,
           left: None,
           up: None,
           right: Some(2),
           down: Some(4),
        });

        layout.insert(2, Key {
           value: 2,
           left: Some(1),
           up: None,
           right: Some(3),
           down: Some(5),
        });

        layout.insert(3, Key {
           value: 3,
           left: Some(2),
           up: None,
           right: None,
           down: Some(6),
        });

        layout.insert(4, Key {
           value: 4,
           left: None,
           up: Some(1),
           right: Some(5),
           down: Some(7),
        });

        layout.insert(5, Key {
           value: 5,
           left: Some(4),
           up: Some(2),
           right: Some(6),
           down: Some(8),
        });

        layout.insert(6, Key {
           value: 6,
           left: Some(5),
           up: Some(3),
           right: None,
           down: Some(9),
        });

        layout.insert(7, Key {
           value: 7,
           left: None,
           up: Some(4),
           right: Some(8),
           down: None,
        });

        layout.insert(8, Key {
           value: 1,
           left: Some(7),
           up: Some(5),
           right: Some(9),
           down: None,
        });

        layout.insert(9, Key {
           value: 9,
           left: Some(8),
           up: Some(6),
           right: None,
           down: None,
        });

        Keypad {
            layout: layout,
            current_value: 5,
        }
    }
}

struct Key {
    value: u8,
    left: Option<u8>,
    right: Option<u8>,
    up: Option<u8>,
    down: Option<u8>,
}

pub fn puzzle(input: &str) -> u8 {
    let keypad = Keypad::new();
    1
}

#[cfg(test)]
mod test {
    use super::*;
}
