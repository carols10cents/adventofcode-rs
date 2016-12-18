
pub fn puzzle(input: &str) -> u32 {
    0
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Safe,
    Trap,
}

fn next_tile(left: Tile, center: Tile, right: Tile) -> Tile {
    Tile::Safe
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn safe() {
        assert_eq!(next_tile(Tile::Safe, Tile::Safe, Tile::Safe), Tile::Safe);
    }

}
//
// ...
// ..^
// .^.
// ^..
// .^^
// ^.^
// ^^.
// ^^^
//
// Its left and center tiles are traps, but its right tile is not.
// Its center and right tiles are traps, but its left tile is not.
// Only its left tile is a trap.
// Only its right tile is a trap.
