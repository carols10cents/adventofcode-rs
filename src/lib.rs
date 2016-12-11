
pub fn puzzle(input: &str) -> u32 {
    let initial_world_state = WorldState {
        steps: 0,
        building: BuildingState {
            elevator_floor: 0,
            floors: [
                vec![
                    Component::Microchip(Element::Hydrogen),
                    Component::Microchip(Element::Lithium),
                ],
                vec![
                    Component::Generator(Element::Hydrogen),
                ],
                vec![
                    Component::Generator(Element::Lithium),
                ],
                vec![],
            ],
        }
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
    building: BuildingState,
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

pub enum Component {
    Microchip(Element),
    Generator(Element),
}

pub enum Element {
    Hydrogen,
    Lithium,
}

pub struct BuildingState {
    elevator_floor: usize,
    floors: [Vec<Component>; 4],
}

#[cfg(test)]
mod test {
    use super::*;

}
