
pub fn puzzle(input: &str) -> usize {
    let initial_world_state = WorldState {
        steps: 0,
        elevator_floor: 0,
    };
    let mut queue = vec![initial_world_state];
    while !queue.is_empty() {
        let world = queue.remove(0);

    }

    panic!("Exhausted all possible moves without finding end condition!");

}

pub struct WorldState {
    steps: u32,
    elevator_floor: usize,
}

#[cfg(test)]
mod test {
    use super::*;

}
