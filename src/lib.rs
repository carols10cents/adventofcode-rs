
pub fn puzzle(input: &str) -> u32 {
    let initial_world_state = WorldState {
        steps: 0,
        elevator_floor: 0,
    };
    let mut queue = vec![initial_world_state];
    while !queue.is_empty() {
        let world = queue.remove(0);
        if world.in_end_state() {
            return world.steps;
        }

    }

    panic!("Exhausted all possible moves without finding end condition!");

}

pub struct WorldState {
    steps: u32,
    elevator_floor: usize,
}

impl WorldState {
    pub fn in_end_state(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

}
