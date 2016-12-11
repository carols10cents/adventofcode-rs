use std::collections::HashMap;

pub fn puzzle(input: &str) -> usize {
    0
}

pub enum BotOrOutput {
    Bot,
    Output
}

pub struct Command {
    low_value_type: BotOrOutput,
    low_value_key: usize,
    high_value_type: BotOrOutput,
    high_value_key: usize,
}

pub struct BotCommandCenter {
    bots: HashMap<usize, Bot>,
    outputs: HashMap<usize, Vec<usize>>,
    saved_commands: HashMap<usize, Command>,
}

pub struct Bot {
    key: usize,
    chip1: Option<usize>,
    chip2: Option<usize>,
}

impl Bot {
    pub fn new(key: usize) -> Bot {
        Bot {
            key: key,
            chip1: None, chip2: None,
        }
    }

    pub fn has_two_chips(&self) -> bool {
        match (self.chip1, self.chip2) {
            (Some(_), Some(_)) => true,
            _ => false,
        }
    }

    pub fn receive_chip(&mut self, chip: usize) {
        if self.chip1.is_none() {
            println!("bot {}, chip1 = {}", self.key, chip);
            self.chip1 = Some(chip);
        } else if self.chip2.is_none() {
            println!("bot {}, chip2 = {}", self.key, chip);
            self.chip2 = Some(chip);
        } else {
            panic!("Cannot hold more than two chips");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;


    #[test]
    fn new_bot_does_not_have_two_chips() {
        let bot = Bot::new(0);
        assert!( ! bot.has_two_chips() );
    }

    #[test]
    fn bot_gets_a_chip_still_not_two() {
        let mut bot = Bot::new(0);
        bot.receive_chip(1);
        assert!( ! bot.has_two_chips() );
    }

    #[test]
    fn bot_gets_two_chips() {
        let mut bot = Bot::new(0);
        bot.receive_chip(1);
        bot.receive_chip(4);
        assert!(bot.has_two_chips());
    }
        // br.exec_command("value 5 goes to bot 2");
        // br.exec_command("bot 2 gives low to bot 1 and high to bot 0");
        // br.exec_command("value 3 goes to bot 1");
        // br.exec_command("bot 1 gives low to output 1 and high to bot 0");
        // br.exec_command("bot 0 gives low to output 2 and high to output 0");
        // br.exec_command("value 2 goes to bot 2");
}
