#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Point {
    x: i32,
    y: i32,
}

pub fn distance(input: &str) -> u32 {
    let mut location = Point { x: 0, y: 0 };
    let mut direction = Direction::North;

    for instruction in input.split_whitespace() {
        let mut chars = instruction.chars();
        if let (Some(turn_dir), Some(num_blocks)) = (chars.next(), chars.next()) {
            direction = turn(direction, turn_dir);
            let num_blocks: i32 = num_blocks.to_string()
                                            .parse()
                                            .expect("Cannot parse");
            match direction {
                Direction::North => location.y += num_blocks,
                Direction::East  => location.x += num_blocks,
                Direction::South => location.y -= num_blocks,
                Direction::West  => location.x -= num_blocks,
            }
        } else {
            panic!("Could not get instructions out of {}", instruction);
        }

    }
    (location.x.abs() + location.y.abs()) as u32
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
    fn r1_is_one() {
        assert_eq!(distance("R1"), 1);
    }

    #[test]
    fn r1_r1_is_two() {
        assert_eq!(distance("R1 R1"), 2);
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