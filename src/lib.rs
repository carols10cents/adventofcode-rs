
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

#[derive(PartialEq, Copy, Clone)]
pub enum Component {
    Microchip(Element),
    Generator(Element),
}

impl Component {
    fn is_fried(&self, others: &Vec<Component>) -> bool {
        match *self {
            Component::Generator(my_element) => {
                // There's microchip(s) that are not my element
                // and no generator that is that microchip's element
                (*others).iter().any(|&c|
                    match c {
                        Component::Microchip(e) if e != my_element && !(*others).contains(&Component::Generator(e)) => true,
                        _ => false,
                    }
                )
            },
            Component::Microchip(my_element) => {
                // There's some generator that is not my element
                (*others).iter().find(|&c| {
                    match *c {
                        Component::Generator(e) if e != my_element => true,
                        _ => false,
                    }
                }).is_some() &&
                // There is no generator that is my element
                (*others).iter().find(|&c| {
                    match *c {
                        Component::Generator(e) if e == my_element => true,
                        _ => false,
                    }
                }).is_none()
            }
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
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

    pub fn next_floors(&self) -> Vec<usize> {
        match self.elevator_floor {
            0 => vec![1],
            1 => vec![0, 2],
            2 => vec![1, 3],
            3 => vec![2],
            other => panic!("How did you get on floor {}, elevator?", other),
        }
    }

    pub fn next_moves(&self) -> Vec<BuildingState> {
        // TODO: actually determine valid next moves
        vec![]
    }

    pub fn has_fried_chips(&self) -> bool {
        for floor in self.floors.iter() {
            for component in floor {
                // This is doing an extra comparison to itself :shrug:
                if component.is_fried(&floor) {
                    return true
                }
            }
        }
        false
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

    #[test]
    fn two_microchips_not_fried() {
        assert!(
            ! Component::Microchip(Element::Hydrogen).is_fried(
                &vec![Component::Microchip(Element::Lithium)]
            )
        )
    }

    #[test]
    fn microchip_alone_not_fried() {
        assert!(
            ! Component::Microchip(Element::Hydrogen).is_fried(
                &vec![]
            )
        )
    }

    #[test]
    fn microchip_with_itself_not_fried() {
        assert!(
            ! Component::Microchip(Element::Hydrogen).is_fried(
                &vec![Component::Microchip(Element::Hydrogen)]
            )
        )
    }

    #[test]
    fn two_generators_not_fried() {
        assert!(
            ! Component::Generator(Element::Hydrogen).is_fried(
                &vec![Component::Generator(Element::Lithium)]
            )
        )
    }

    #[test]
    fn microchip_and_matching_generator_not_fried() {
        assert!(
            ! Component::Microchip(Element::Hydrogen).is_fried(
                &vec![Component::Generator(Element::Hydrogen)]
            )
        )
    }

    #[test]
    fn microchip_attached_to_matching_generator_not_fried_by_other_generator() {
        assert!(
            ! Component::Microchip(Element::Hydrogen).is_fried(
                &vec![
                    Component::Generator(Element::Lithium),
                    Component::Generator(Element::Hydrogen),
                ]
            )
        )
    }

    #[test]
    fn microchip_without_its_generator_with_another_generator_fried() {
        assert!(
            Component::Microchip(Element::Hydrogen).is_fried(
                &vec![Component::Generator(Element::Lithium)]
            )
        )
    }

    #[test]
    fn generator_with_a_microchip_without_its_generator_fried() {
        assert!(
            Component::Generator(Element::Hydrogen).is_fried(
                &vec![Component::Microchip(Element::Lithium)]
            )
        )
    }

    #[test]
    fn generator_with_a_generator_not_fried() {
        assert!(
            ! Component::Generator(Element::Hydrogen).is_fried(
                &vec![Component::Generator(Element::Lithium)]
            )
        )
    }

    #[test]
    fn generator_with_its_microchip_not_fried() {
        assert!(
            ! Component::Generator(Element::Hydrogen).is_fried(
                &vec![Component::Microchip(Element::Hydrogen)]
            )
        )
    }

    #[test]
    fn generator_with_itself_not_fried() {
        assert!(
            ! Component::Generator(Element::Hydrogen).is_fried(
                &vec![Component::Generator(Element::Hydrogen)]
            )
        )
    }

    #[test]
    fn generator_with_a_microchip_with_its_generator_not_fried() {
        assert!(
            ! Component::Generator(Element::Hydrogen).is_fried(
                &vec![
                    Component::Generator(Element::Lithium),
                    Component::Microchip(Element::Lithium),
                ]
            )
        )
    }

}
