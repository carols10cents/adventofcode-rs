use std::str::FromStr;
use std::error::Error;

pub fn puzzle(input: &str) -> Result<usize, Box<Error>> {
    let mut row: Row = input.trim().parse()?;
    let mut result = 0;

    for _ in 0..40 {
        result += row.num_safe_tiles();
        row = row.next();
    }

    Ok(result)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tile {
    Safe,
    Trap,
}

pub fn next_tile(left: Tile, center: Tile, right: Tile) -> Tile {
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
pub struct Row(Vec<Tile>);

impl Row {
    fn next(&self) -> Row {
        let mut x = vec![Tile::Safe];
        x.extend_from_slice(&self.0);
        x.push(Tile::Safe);

        Row(x.windows(3).map(|tiles| {
            next_tile(tiles[0], tiles[1], tiles[2])
        }).collect())
    }

    fn num_safe_tiles(&self) -> usize {
        self.0.iter().filter(|&&t| t == Tile::Safe).count()
    }
}

impl Tile {
    fn from_char(c: char) -> Result<Tile, Box<Error>> {
        Ok(match c {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => return Err(format!("unknown tile character {}", c).into()),
        })
    }
}

impl FromStr for Row {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Row, Self::Err> {
        s.chars().map(Tile::from_char).collect::<Result<_, _>>().map(Row)
    }
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

    fn string_to_row(s: &str) -> Row {
        s.parse().expect("Couldn't create Row")
    }

    #[test]
    fn row_at_a_time() {
        assert_eq!(string_to_row("..^^.").next(), string_to_row(".^^^^"));
    }

}
