use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance_to_origin(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

pub fn distance(input: &str) -> u32 {
    let mut visited_locations = HashSet::new();

    let mut location = Point { x: 0, y: 0 };
    let mut direction = Direction::North;

    for instruction in input.split(", ") {
        let (turn_dir, num_blocks) = instruction.split_at(1);

        println!("Now processing ({}) ({})", turn_dir, num_blocks);
        direction = turn(direction, turn_dir);
        let num_blocks: i32 = num_blocks.to_string()
                                        .trim()
                                        .parse()
                                        .expect("Cannot parse");
        println!("Going {:?} {} blocks", direction, num_blocks);
        match direction {
            Direction::North => location.y += num_blocks,
            Direction::East  => location.x += num_blocks,
            Direction::South => location.y -= num_blocks,
            Direction::West  => location.x -= num_blocks,
        }

        if visited_locations.contains(&location) {
            return location.distance_to_origin();
        } else {
            visited_locations.insert(location);
        }
        println!("Location: ({}, {})", location.x, location.y);
    }
    panic!("No location visited twice.");
}

pub fn turn(initial_direction: Direction, turn_command: &str) -> Direction {
    match (turn_command, initial_direction) {
        ("L", Direction::North) => Direction::West,
        ("L", Direction::West) => Direction::South,
        ("L", Direction::South) => Direction::East,
        ("L", Direction::East) => Direction::North,

        ("R", Direction::North) => Direction::East,
        ("R", Direction::East) => Direction::South,
        ("R", Direction::South) => Direction::West,
        ("R", Direction::West) => Direction::North,

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
    fn hq_is_first_location_visited_twice() {
        assert_eq!(distance("R2, L1, L1, L1, L1, R5"), 2);
    }

    #[test]
    fn turn_left() {
        let mut dir = Direction::North;
        dir = turn(dir, "L");
        assert_eq!(dir, Direction::West);
        dir = turn(dir, "L");
        assert_eq!(dir, Direction::South);
        dir = turn(dir, "L");
        assert_eq!(dir, Direction::East);
        dir = turn(dir, "L");
        assert_eq!(dir, Direction::North);
    }

    #[test]
    fn turn_right() {
        let mut dir = Direction::North;
        dir = turn(dir, "R");
        assert_eq!(dir, Direction::East);
        dir = turn(dir, "R");
        assert_eq!(dir, Direction::South);
        dir = turn(dir, "R");
        assert_eq!(dir, Direction::West);
        dir = turn(dir, "R");
        assert_eq!(dir, Direction::North);
    }


}