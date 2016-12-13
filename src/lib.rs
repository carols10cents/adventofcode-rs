use std::error::Error;

pub fn puzzle(input: &str) -> i32 {
    for line in input.lines() {
        let line = line.trim();
    }
    0
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Copy(FromLocation, Register),
}

impl std::str::FromStr for Instruction {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        match parts.next() {
            Some("cpy") => Ok(
                Instruction::Copy(
                    parts.next().ok_or("no from location")?.parse()?,
                    parts.next().ok_or("no register")?.parse()?
                )
            ),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FromLocation {
    Integer(i32),
}

impl std::str::FromStr for FromLocation {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FromLocation::Integer(s.parse()?))
    }
}

#[derive(Debug, PartialEq)]
pub enum Register {
    A,
}

impl std::str::FromStr for Register {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Register::A),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_cpy() {
        let instr: Instruction = "cpy 41 a".parse().expect("couldn't parse");
        assert_eq!(
            instr,
            Instruction::Copy(FromLocation::Integer(41), Register::A)
        );
    }

    #[test]
    fn sample() {
        let input = "cpy 41 a
            inc a
            inc a
            dec a
            jnz a 2
            dec a";
        assert_eq!(puzzle(input), 42);
    }
}
