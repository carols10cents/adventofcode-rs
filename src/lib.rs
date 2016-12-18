
pub fn puzzle(input: &str) -> u32 {
    0
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Safe,
    Trap,
}

fn next_tile(left: Tile, center: Tile, right: Tile) -> Tile {
    use Tile::*;
    match (left, center, right) {
        (Trap, Trap, Safe) => Trap,
        (Safe, Trap, Trap) => Trap,
        (Trap, Safe, Safe) => Trap,
        (Safe, Safe, Trap) => Trap,
        _ => Safe,
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Row(Vec<Tile>);

impl Row {
    fn next(&self) -> Row {
        let mut x = vec![Tile::Safe];
        x.extend_from_slice(&self.0);
        x.push(Tile::Safe);

        Row(x.windows(3).map(|tiles| {
            next_tile(tiles[0], tiles[1], tiles[2])
        }).collect())
    }
}

fn string_to_row(s: &str) -> Row {
    Row(s.chars().map(|c| {
        match c {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => panic!("unknown tile character {}", c),
        }
    }).collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn safe() {
        assert_eq!(next_tile(Tile::Safe, Tile::Safe, Tile::Safe), Tile::Safe);
        assert_eq!(next_tile(Tile::Safe, Tile::Trap, Tile::Safe), Tile::Safe);
        assert_eq!(next_tile(Tile::Trap, Tile::Safe, Tile::Trap), Tile::Safe);
        assert_eq!(next_tile(Tile::Trap, Tile::Trap, Tile::Trap), Tile::Safe);
    }

    #[test]
    fn first_rule() {
        assert_eq!(next_tile(Tile::Trap, Tile::Trap, Tile::Safe), Tile::Trap);
    }

    #[test]
    fn second_rule() {
        assert_eq!(next_tile(Tile::Safe, Tile::Trap, Tile::Trap), Tile::Trap);
    }

    #[test]
    fn third_rule() {
        assert_eq!(next_tile(Tile::Trap, Tile::Safe, Tile::Safe), Tile::Trap);
    }

    #[test]
    fn fourth_rule() {
        assert_eq!(next_tile(Tile::Safe, Tile::Safe, Tile::Trap), Tile::Trap);
    }

    #[test]
    fn row_at_a_time() {
        assert_eq!(string_to_row("..^^.").next(), string_to_row(".^^^^"));
    }

}
