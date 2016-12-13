use std::error::Error;
use std::str::FromStr;

pub fn puzzle(input: &str) -> i32 {
    for line in input.lines() {
        let line = line.trim();
    }
    0
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
    Copy(FromLocation, Register),
    Increment(Register),
    Decrement(Register),
    JumpNonZero(Register, i32),
}

impl FromStr for Instruction {
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
            Some("inc") => Ok(
                Instruction::Increment(
                    parts.next().ok_or("no register")?.parse()?
                )
            ),
            Some("dec") => Ok(
                Instruction::Decrement(
                    parts.next().ok_or("no register")?.parse()?
                )
            ),
            Some("jnz") => Ok(
                Instruction::JumpNonZero(
                    parts.next().ok_or("no register")?.parse()?,
                    parts.next().ok_or("no offset")?.parse()?
                )
            ),
            Some(other) => Err(format!("Unknown instruction: {}", other).into()),
            None => Err("Instruction missing".into()),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FromLocation {
    Integer(i32),
    Register(Register),
}

impl FromStr for FromLocation {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(FromLocation::Integer)
                 .or_else(|_| s.parse().map(FromLocation::Register))
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Register {
    A,
    B,
    C,
    D,
}

impl FromStr for Register {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Register::*;
        match s {
            "a" => Ok(A),
            "b" => Ok(B),
            "c" => Ok(C),
            "d" => Ok(D),
            other => Err(format!("Unknown register {}", other).into()),
        }
    }
}

pub struct Machine {
    registers: [i32; 4],
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            registers: [0; 4],
        }
    }

    fn execute(&mut self, instructions: &[Instruction]) {
        for &instruction in instructions {
            match instruction {
                Instruction::Copy(
                    FromLocation::Integer(i),
                    to
                ) => {
                    let index = Machine::register_index(to);
                    self.registers[index] = i;
                },
                Instruction::Copy(FromLocation::Register(from), to) => {
                    let index_from = Machine::register_index(from);
                    let index_to = Machine::register_index(to);
                    self.registers[index_to] = self.registers[index_from];
                },
                Instruction::Increment(register) => {
                    let index = Machine::register_index(register);
                    self.registers[index] += 1;
                },
                Instruction::Decrement(register) => {
                    let index = Machine::register_index(register);
                    self.registers[index] -= 1;
                },
                Instruction::JumpNonZero(_, _) => {},
            }
        }
    }

    fn register_index(register: Register) -> usize {
        use Register::*;
        match register {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
        }
    }

    fn value_of(&self, register: Register) -> i32 {
        let index = Machine::register_index(register);
        self.registers[index]
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
    fn parse_cpy_register() {
        let instr: Instruction = "cpy a c".parse().expect("couldn't parse");
        assert_eq!(
            instr,
            Instruction::Copy(
                FromLocation::Register(Register::A),
                Register::C,
            )
        );
    }

    #[test]
    fn parse_inc() {
        let instr: Instruction = "inc a".parse().expect("couldn't parse");
        assert_eq!(
            instr,
            Instruction::Increment(Register::A)
        );
    }

    #[test]
    fn parse_dec() {
        let instr: Instruction = "dec a".parse().expect("couldn't parse");
        assert_eq!(
            instr,
            Instruction::Decrement(Register::A)
        );
    }

    #[test]
    fn parse_jnz() {
        let instr: Instruction = "jnz a 2".parse().expect("couldn't parse");
        assert_eq!(
            instr,
            Instruction::JumpNonZero(Register::A, 2)
        );
    }

    #[test]
    fn execute_copy() {
        let mut machine = Machine::new();
        machine.execute(
            &[
                Instruction::Copy(
                    FromLocation::Integer(5),
                    Register::A,
                ),
            ]
        );

        assert_eq!(machine.value_of(Register::A), 5);
    }

    #[test]
    fn execute_copy_register() {
        let mut machine = Machine::new();
        machine.execute(
            &[
                Instruction::Copy(
                    FromLocation::Integer(5),
                    Register::A,
                ),
                Instruction::Copy(
                    FromLocation::Register(Register::A),
                    Register::B,
                ),
            ]
        );

        assert_eq!(machine.value_of(Register::B), 5);
    }

    #[test]
    fn execute_increment() {
        let mut machine = Machine::new();
        machine.execute(
            &[
                Instruction::Increment(
                    Register::C,
                ),
            ]
        );

        assert_eq!(machine.value_of(Register::C), 1);
    }

    #[test]
    fn execute_decrement() {
        let mut machine = Machine::new();
        machine.execute(
            &[
                Instruction::Decrement(
                    Register::D,
                ),
            ]
        );

        assert_eq!(machine.value_of(Register::D), -1);
    }

    #[test]
    fn execute_jnz_when_zero() {
        let mut machine = Machine::new();
        machine.execute(
            &[
                // D should be 0, jump doesn't happen
                Instruction::JumpNonZero(Register::D, 2),
                Instruction::Increment(
                    Register::A,
                ),
            ]
        );

        assert_eq!(machine.value_of(Register::A), 1);
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
