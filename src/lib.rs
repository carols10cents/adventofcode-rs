
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
        self.building.in_end_state()
    }

    pub fn next_moves(&self) -> Vec<WorldState> {
        self.building.next_moves().into_iter().map(|b| {
            WorldState {
                steps: self.steps + 1,
                building: b,
            }
        }).collect()
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

impl BuildingState {
    pub fn in_end_state(&self) -> bool {
        self.floors[0].is_empty() &&
            self.floors[1].is_empty() &&
            self.floors[2].is_empty()
    }

    pub fn next_moves(&self) -> Vec<BuildingState> {
        // TODO: actually determine valid next moves
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn not_in_end_state() {
        let world_state = WorldState {
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
        assert!( ! world_state.in_end_state() );
    }

    #[test]
    fn in_end_state() {
        let world_state = WorldState {
            steps: 0,
            building: BuildingState {
                elevator_floor: 0,
                floors: [
                    vec![],
                    vec![],
                    vec![],
                    vec![
                        Component::Generator(Element::Hydrogen),
                        Component::Microchip(Element::Hydrogen),
                        Component::Microchip(Element::Lithium),
                        Component::Generator(Element::Lithium),
                    ],
                ],
            }
        };

        assert!(world_state.in_end_state());
    }
}
