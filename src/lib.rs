use std::collections::HashMap;

pub fn puzzle(input: &str) -> usize {
    let mut br = BotRouter {
        bots: HashMap::new(),
        output_bins: HashMap::new(),
    };

    for line in input.lines() {
        br.exec_command(line);
    }
    0
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BotOrOutput {
    Bot,
    Output,
}

#[derive(PartialEq, Debug)]
pub struct Bot {
    key: usize,
    chip1: Option<usize>,
    chip2: Option<usize>,
    low: Option<usize>,
    low_bot_or_output: Option<BotOrOutput>,
    high: Option<usize>,
    high_bot_or_output: Option<BotOrOutput>,
}

impl Bot {
    pub fn new(key: usize) -> Bot {
        Bot {
            key: key,
            chip1: None, chip2: None,
            low: None, high: None,
            low_bot_or_output: None,
            high_bot_or_output: None,
        }
    }

    pub fn receive_chip(&mut self, chip: usize)
           -> Vec<(usize, BotOrOutput, usize)> {
        if self.chip1.is_none() {
            self.chip1 = Some(chip);
            vec![]
        } else if self.chip2.is_none() {
            self.chip2 = Some(chip);
            if let (Some(lo), Some(hi)) = (self.low, self.high) {
                let return_commands = if self.chip1 < self.chip2 {
                    if self.chip1 == Some(17) && self.chip2 == Some(61) {
                        panic!("The bot you are looking for is {}", self.key);
                    }

                    vec![
                        (
                            lo,
                            self.low_bot_or_output.unwrap(),
                            self.chip1.unwrap()
                        ),
                        (
                            hi,
                            self.high_bot_or_output.unwrap(),
                            self.chip2.unwrap()
                        ),
                    ]
                } else {
                    if self.chip2 == Some(17) && self.chip1 == Some(61) {
                        panic!("The bot you are looking for is {}", self.key);
                    }

                    vec![
                        (
                            lo,
                            self.low_bot_or_output.unwrap(),
                            self.chip2.unwrap()
                        ),
                        (
                            hi,
                            self.high_bot_or_output.unwrap(),
                            self.chip1.unwrap()
                        ),
                    ]
                };
                self.chip1 = None;
                self.chip2 = None;
                self.low = None;
                self.high = None;
                self.low_bot_or_output = None;
                self.high_bot_or_output = None;
                return_commands
            } else {
                vec![]
            }
        } else {
            panic!("Cannot hold more than two chips");
        }
    }

    pub fn receive_command(&mut self, low_bot_or_output: &str, low: usize,
                                      high_bot_or_output: &str, high: usize) {
        self.low = Some(low);
        if low_bot_or_output == "bot" {
            self.low_bot_or_output = Some(BotOrOutput::Bot);
        } else {
            self.low_bot_or_output = Some(BotOrOutput::Output);
        }
        self.high = Some(high);
        if high_bot_or_output == "bot" {
            self.high_bot_or_output = Some(BotOrOutput::Bot);
        } else {
            self.high_bot_or_output = Some(BotOrOutput::Output);
        }
    }
}

pub struct BotRouter {
    bots: HashMap<usize, Bot>,
    output_bins: HashMap<usize, Vec<usize>>,
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
                pieces[5],
                pieces[6].parse().expect("can't parse low key"),
                pieces[10],
                pieces[11].parse().expect("can't parse high key"),
            );
        }
    }

    pub fn value_to_bot(&mut self, bot_key: usize, value: usize) {
        let return_commands = self.bots
                                  .entry(bot_key)
                                  .or_insert(Bot::new(bot_key))
                                  .receive_chip(value);
        for &(key, bot_or_output, chip_value) in return_commands.iter() {
            match bot_or_output {
                BotOrOutput::Bot => self.value_to_bot(key, chip_value),
                BotOrOutput::Output => self.value_to_output(key, chip_value),
            }
        }
    }

    pub fn value_to_output(&mut self, output_key: usize, value: usize) {
        self.output_bins.entry(output_key).or_insert(Vec::new()).push(value);
    }

    pub fn command_to_bot(&mut self, bot_key: usize,
                          low_bot_or_output: &str, low_key: usize,
                          high_bot_or_output: &str, high_key: usize) {
        let bot = self.bots.entry(bot_key).or_insert(Bot::new(bot_key));
        bot.receive_command(
            low_bot_or_output,
            low_key,
            high_bot_or_output,
            high_key
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn router_can_give_values() {
        let mut br = BotRouter {
            bots: HashMap::new(),
            output_bins: HashMap::new(),
        };
        br.exec_command("value 5 goes to bot 2");
        assert_eq!(
            br.bots.get(&2).unwrap().chip1,
            Some(5)
        );
    }

    #[test]
    fn router_can_give_commands_that_get_stored() {
        let mut br = BotRouter {
            bots: HashMap::new(),
            output_bins: HashMap::new(),
        };
        br.exec_command("bot 2 gives low to bot 1 and high to bot 0");
        assert_eq!(
            br.bots.get(&2).unwrap().low,
            Some(1)
        );
        assert_eq!(
            br.bots.get(&2).unwrap().high,
            Some(0)
        );
    }

    #[test]
    fn router_adds_two_values_so_commands_get_executed() {
        let mut br = BotRouter {
            bots: HashMap::new(),
            output_bins: HashMap::new(),
        };
        br.exec_command("value 5 goes to bot 2");
        br.exec_command("bot 2 gives low to bot 1 and high to bot 0");
        br.exec_command("value 2 goes to bot 2");

        assert_eq!(
            br.bots.get(&2).unwrap().chip1,
            None
        );
        assert_eq!(
            br.bots.get(&2).unwrap().chip2,
            None
        );

        assert_eq!(
            br.bots.get(&1).unwrap().chip1,
            Some(2)
        );
        assert_eq!(
            br.bots.get(&1).unwrap().chip2,
            None
        );

        assert_eq!(
            br.bots.get(&0).unwrap().chip1,
            Some(5)
        );
        assert_eq!(
            br.bots.get(&0).unwrap().chip2,
            None
        );
    }

    #[test]
    fn test_sample() {
        let mut br = BotRouter {
            bots: HashMap::new(),
            output_bins: HashMap::new(),
        };
        br.exec_command("value 5 goes to bot 2");
        br.exec_command("bot 2 gives low to bot 1 and high to bot 0");
        br.exec_command("value 3 goes to bot 1");
        br.exec_command("bot 1 gives low to output 1 and high to bot 0");
        br.exec_command("bot 0 gives low to output 2 and high to output 0");
        br.exec_command("value 2 goes to bot 2");

        assert_eq!(
            br.output_bins.get(&0),
            Some(&vec![5])
        );

        assert_eq!(
            br.output_bins.get(&1),
            Some(&vec![2])
        );

        assert_eq!(
            br.output_bins.get(&2),
            Some(&vec![3])
        )
    }
}
