
pub fn puzzle(input: &str) -> usize {
    0
}

#[derive(Clone, PartialEq)]
pub enum WhichChip {
    Low,
    High,
}

pub struct Bot {
    chip1: Option<usize>,
    chip2: Option<usize>,
    commands: Vec<(usize, WhichChip)>
}

impl Bot {
    pub fn new() -> Bot {
        Bot { chip1: None, chip2: None, commands: vec![], }
    }

    pub fn has_two_chips(&self) -> bool {
        match (self.chip1, self.chip2) {
            (Some(_), Some(_)) => true,
            _ => false,
        }
    }

    pub fn receive_chip(&mut self, chip: usize) {
        if self.chip1.is_none() {
            self.chip1 = Some(chip);
        } else if self.chip2.is_none() {
            self.chip2 = Some(chip);
        } else {
            panic!("Cannot hold more than two chips");
        }
    }

    pub fn receive_command(&mut self, which_bot: usize, which_chip: WhichChip) {
        self.commands.push((which_bot, which_chip));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_bot_does_not_have_two_chips() {
        let bot = Bot::new();
        assert!( ! bot.has_two_chips() );
    }

    #[test]
    fn bot_gets_a_chip_still_not_two() {
        let mut bot = Bot::new();
        bot.receive_chip(1);
        assert!( ! bot.has_two_chips() );
    }

    #[test]
    fn bot_gets_two_chips() {
        let mut bot = Bot::new();
        bot.receive_chip(1);
        bot.receive_chip(4);
        assert!(bot.has_two_chips());
    }
}
