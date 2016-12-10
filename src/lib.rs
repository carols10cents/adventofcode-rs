
pub fn puzzle(input: &str) -> usize {
    0
}

pub struct Bot {
    chip1: Option<usize>,
    chip2: Option<usize>,
}

impl Bot {
    pub fn new() -> Bot {
        Bot { chip1: None, chip2: None }
    }

    pub fn has_two_chips(&self) -> bool {
        match (self.chip1, self.chip2) {
            (Some(_), Some(_)) => true,
            _ => false,
        }
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
}
