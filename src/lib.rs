#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub fn distance(input: &str) -> u32 {
    let mut chars = input.chars();
    if let (Some(_turn_dir), Some(num_blocks)) = (chars.next(), chars.next()) {
        let num_blocks: u32 = num_blocks.to_string()
                                        .parse()
                                        .expect("Cannot parse");
        return num_blocks
    }
    panic!("oh no!");
}

pub fn turn(initial_direction: Direction, turn_command: char) -> Direction {
    match (turn_command, initial_direction) {
        ('L', Direction::North) => Direction::West,
        ('L', Direction::West) => Direction::South,
        ('L', Direction::South) => Direction::East,
        ('L', Direction::East) => Direction::North,

        ('R', Direction::North) => Direction::East,
        ('R', Direction::East) => Direction::South,
        ('R', Direction::South) => Direction::West,
        ('R', Direction::West) => Direction::North,

        (unknown_turn_command, dir) => panic!("\
            I don't know how to turn {} from {:?}!",
            unknown_turn_command,
            dir
        ),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn right_one_is_one() {
        assert_eq!(distance("R1"), 1);
    }

    #[test]
    fn turn_left() {
        let mut dir = Direction::North;
        dir = turn(dir, 'L');
        assert_eq!(dir, Direction::West);
        dir = turn(dir, 'L');
        assert_eq!(dir, Direction::South);
        dir = turn(dir, 'L');
        assert_eq!(dir, Direction::East);
        dir = turn(dir, 'L');
        assert_eq!(dir, Direction::North);
    }

    #[test]
    fn turn_right() {
        let mut dir = Direction::North;
        dir = turn(dir, 'R');
        assert_eq!(dir, Direction::East);
        dir = turn(dir, 'R');
        assert_eq!(dir, Direction::South);
        dir = turn(dir, 'R');
        assert_eq!(dir, Direction::West);
        dir = turn(dir, 'R');
        assert_eq!(dir, Direction::North);
    }
}