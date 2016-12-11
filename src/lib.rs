use std::collections::HashMap;

pub fn puzzle(input: &str) -> usize {
    let mut br = BotCommandCenter::new();

    for line in input.lines() {
        br.exec_command(line);
    }

    br.outputs.get(&0).unwrap().first().unwrap() *
        br.outputs.get(&1).unwrap().first().unwrap() *
        br.outputs.get(&2).unwrap().first().unwrap()
}

#[derive(Debug)]
pub enum BotOrOutput {
    Bot,
    Output
}

impl<'a> From<&'a str> for BotOrOutput {
    fn from(s: &'a str) -> BotOrOutput {
        match s {
            "bot" => BotOrOutput::Bot,
            "output" => BotOrOutput::Output,
            _ => panic!("Unknown string given to BotOrOutput"),
        }
    }
}

#[derive(Debug)]
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
    bots_to_check: Vec<usize>,
}

impl BotCommandCenter {
    pub fn new() -> BotCommandCenter {
        BotCommandCenter {
            bots: HashMap::new(),
            outputs: HashMap::new(),
            saved_commands: HashMap::new(),
            bots_to_check: vec![],
        }
    }

    pub fn exec_command(&mut self, input: &str) {
        println!("COMMAND: {}", input);
        let pieces: Vec<&str> = input.split_whitespace().collect();

        if pieces[0] == "value" {
            self.value_to_bot(
                pieces[5].parse().expect("can't parse bot key"),
                pieces[1].parse().expect("can't parse chip value")
            );
            self.process_bots();
        } else if pieces[0] == "bot" {
            self.command_to_bot(
                pieces[1].parse().expect("can't parse bot key"),
                pieces[5],
                pieces[6].parse().expect("can't parse low key"),
                pieces[10],
                pieces[11].parse().expect("can't parse high key"),
            );
            self.process_bots();
        }
    }

    pub fn value_to_bot(&mut self, bot_key: usize, value: usize) {
        println!("COMMAND CTR sending value {} to bot {}", value, bot_key);
        self.bots
            .entry(bot_key)
            .or_insert(Bot::new(bot_key))
            .receive_chip(value);
        self.bots_to_check.push(bot_key);
        println!("COMMAND CTR bots to check = {:?}", self.bots_to_check);
    }

    pub fn value_to_output(&mut self, output_key: usize, value: usize) {
        println!("COMMAND CTR sending value {} to output {}", value, output_key);
        self.outputs
            .entry(output_key)
            .or_insert(vec![])
            .push(value);
    }

    pub fn get_next_bot_to_check(&mut self) -> usize {
        self.bots_to_check.remove(0)
    }

    pub fn bot_has_two_chips(&mut self, check_bot_key: usize) -> bool {
        let bot = self.bots.entry(check_bot_key)
                           .or_insert(Bot::new(check_bot_key));
        bot.has_two_chips()
    }

    pub fn bot_low_high(&mut self, check_bot_key: usize) -> (usize, usize) {
        let bot = self.bots.entry(check_bot_key)
                           .or_insert(Bot::new(check_bot_key));
        bot.low_high()
    }

    pub fn process_bots(&mut self) {
        while !self.bots_to_check.is_empty() {
            println!("COMMAND CTR process_bots, {} bots to process", self.bots_to_check.len());
            let check_bot_key = self.get_next_bot_to_check();
            println!("COMMAND CTR processing bot {}", check_bot_key);
            if self.bot_has_two_chips(check_bot_key) {
                println!("COMMAND CTR found bot {} has two chips", check_bot_key);

                if let Some(command) = self.saved_commands.remove(&check_bot_key) {
                    println!("COMMAND CTR found command: {:?}", command);
                    let (low_value, high_value) = self.bot_low_high(check_bot_key);
                    println!("COMMAND CTR found low = {}, high = {}", low_value, high_value);
                    match command.low_value_type {
                        BotOrOutput::Bot => self.value_to_bot(
                            command.low_value_key,
                            low_value
                        ),
                        BotOrOutput::Output => self.value_to_output(
                            command.low_value_key,
                            low_value
                        ),
                    }

                    match command.high_value_type {
                        BotOrOutput::Bot => self.value_to_bot(
                            command.high_value_key,
                            high_value
                        ),
                        BotOrOutput::Output => self.value_to_output(
                            command.high_value_key,
                            high_value
                        ),
                    }
                } else {
                    println!("COMMAND CTR found no saved command for bot {} that has two chips", check_bot_key);
                }

            }
        }
        println!("COMMAND CTR bots_to_check is empty");
    }

    pub fn command_to_bot(&mut self, bot_key: usize,
                              low_bot_or_output: &str, low_key: usize,
                              high_bot_or_output: &str, high_key: usize) {

        let command = Command {
            low_value_type: low_bot_or_output.into(),
            low_value_key: low_key,
            high_value_type: high_bot_or_output.into(),
            high_value_key: high_key,
        };
        self.saved_commands.insert(bot_key, command);
        self.bots_to_check.push(bot_key);
        println!("COMMAND CTR bots to check = {:?}", self.bots_to_check);
    }
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

    pub fn low_high(&mut self) -> (usize, usize) {
        let (low, high) = match (self.chip1, self.chip2) {
            (Some(c1), Some(c2)) => {
                if c1 < c2 {
                    println!("Bot {} found {} < {}", self.key, c1, c2);
                    (c1, c2)
                } else {
                    println!("Bot {} found {} < {}", self.key, c2, c1);
                    (c2, c1)
                }
            }
            _ => panic!("Tried to get low and high from bot, something is wrong"),
        };
        self.chip1 = None;
        self.chip2 = None;
        (low, high)
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



    #[test]
    fn test_sample() {
        let mut br = BotCommandCenter::new();

        br.exec_command("value 5 goes to bot 2");
        br.exec_command("bot 2 gives low to bot 1 and high to bot 0");
        br.exec_command("value 3 goes to bot 1");
        br.exec_command("bot 1 gives low to output 1 and high to bot 0");
        br.exec_command("bot 0 gives low to output 2 and high to output 0");
        br.exec_command("value 2 goes to bot 2");


        assert_eq!(
            br.outputs.get(&0),
            Some(&vec![5])
        );

        assert_eq!(
            br.outputs.get(&1),
            Some(&vec![2])
        );

        assert_eq!(
            br.outputs.get(&2),
            Some(&vec![3])
        );
    }
}
