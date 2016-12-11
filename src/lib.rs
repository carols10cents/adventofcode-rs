use std::collections::HashMap;

pub fn puzzle(input: &str) -> usize {
    0
}

#[derive(PartialEq, Debug)]
pub struct Bot {
    chip1: Option<usize>,
    chip2: Option<usize>,
    commands: Vec<(usize, usize)>
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

    pub fn receive_command(&mut self, low_bot: usize, high_bot: usize) {
        self.commands.push((low_bot, high_bot));
    }
}

pub struct BotRouter {
    bots: HashMap<usize, Bot>,
}

impl BotRouter {
    pub fn exec_command(&mut self, input: &str) {
        let pieces: Vec<&str> = input.split_whitespace().collect();

        if pieces[0] == "value" {
            self.value_to_bot(
                pieces[5].parse().expect("can't parse bot key"),
                pieces[1].parse().expect("can't parse chip value")
            );
        } else if pieces[0] == "bot" {
            self.command_to_bot(
                pieces[1].parse().expect("can't parse bot key"),
                pieces[6].parse().expect("can't parse low bot key"),
                pieces[11].parse().expect("can't parse high bot key"),
            );
        }
    }

    pub fn value_to_bot(&mut self, bot_key: usize, value: usize) {
        let bot = self.bots.entry(bot_key).or_insert(Bot::new());
        bot.receive_chip(value);
    }

    pub fn command_to_bot(&mut self, bot_key: usize,
                          low_bot_key: usize, high_bot_key: usize) {
        let bot = self.bots.entry(bot_key).or_insert(Bot::new());
        bot.receive_command(low_bot_key, high_bot_key);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

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

    #[test]
    fn router_can_give_values() {
        let mut br = BotRouter { bots: HashMap::new() };
        br.exec_command("value 5 goes to bot 2");
        assert_eq!(
            br.bots.get(&2),
            Some(&Bot { chip1: Some(5), chip2: None, commands: vec![] })
        );
    }

    #[test]
    fn router_can_give_commands_that_get_stored() {
        let mut br = BotRouter { bots: HashMap::new() };
        br.exec_command("bot 2 gives low to bot 1 and high to bot 0");
        assert_eq!(
            br.bots.get(&2),
            Some(&Bot {
                chip1: None,
                chip2: None,
                commands: vec![(1, 0)]
            })
        );
    }

    // #[test]
    // fn router_adds_two_values_so_commands_get_executed() {
    //     let mut br = BotRouter { bots: HashMap::new() };
    //     br.exec_command("value 5 goes to bot 2");
    //     br.exec_command("bot 2 gives low to bot 1 and high to bot 0");
    //     br.exec_command("value 2 goes to bot 2");
    //
    //     assert_eq!(
    //         br.bots.get(&2),
    //         Some(&Bot {
    //             chip1: None,
    //             chip2: None,
    //             commands: vec![]
    //         })
    //     );
    //
    //     assert_eq!(
    //         br.bots.get(&1),
    //         Some(&Bot {
    //             chip1: Some(2),
    //             chip2: None,
    //             commands: vec![]
    //         })
    //     );
    //
    //     assert_eq!(
    //         br.bots.get(&0),
    //         Some(&Bot {
    //             chip1: Some(5),
    //             chip2: None,
    //             commands: vec![]
    //         })
    //     );
    // }

    // #[test]
    // fn test_sample() {
    //     let mut br = BotRouter { bots: HashMap::new() };
    //     br.exec_command("value 5 goes to bot 2");
    //     br.exec_command("bot 2 gives low to bot 1 and high to bot 0");
    //     br.exec_command("value 3 goes to bot 1");
    //     br.exec_command("bot 1 gives low to output 1 and high to bot 0");
    //     br.exec_command("bot 0 gives low to output 2 and high to output 0");
    //     br.exec_command("value 2 goes to bot 2");
    // }
}
