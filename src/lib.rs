
pub fn puzzle(input: &str) -> u32 {
    let initial_world_state = WorldState {
        steps: 0,
        elevator_floor: 0,
        // TODO: actually specify initial state
    };
    let mut queue = vec![initial_world_state];
    while !queue.is_empty() {
        let world = queue.remove(0);
        if world.in_end_state() {
            return world.steps;
        }

        let mut valid_next_moves = world.next_moves();
        queue.append(&mut valid_next_moves);
    }

    panic!("Exhausted all possible moves without finding end condition!");
}

pub struct WorldState {
    steps: u32,
    elevator_floor: usize,
}

impl WorldState {
    pub fn in_end_state(&self) -> bool {
        // TODO: actually check that everything is on the 4th floor
        true
    }

    pub fn next_moves(&self) -> Vec<WorldState> {
        // TODO: actually determine valid next moves
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

}
