#![feature(alloc_system)]
extern crate alloc_system;

use std::collections::{BTreeSet, VecDeque};

pub fn puzzle(input: &str) -> u32 {
    // let mut floor1 = BTreeSet::new();
    // floor1.insert(Component::Generator(Element::Thulium));
    // floor1.insert(Component::Microchip(Element::Thulium));
    // floor1.insert(Component::Generator(Element::Plutonium));
    // floor1.insert(Component::Generator(Element::Strontium));
    //
    // let mut floor2 = BTreeSet::new();
    // floor2.insert(Component::Microchip(Element::Plutonium));
    // floor2.insert(Component::Microchip(Element::Strontium));
    //
    // let mut floor3 = BTreeSet::new();
    // floor3.insert(Component::Generator(Element::Promethium));
    // floor3.insert(Component::Microchip(Element::Promethium));
    // floor3.insert(Component::Generator(Element::Ruthenium));
    // floor3.insert(Component::Microchip(Element::Ruthenium));

    let mut floor1 = BTreeSet::new();
    floor1.insert(Component::Microchip(Element::Hydrogen));
    floor1.insert(Component::Microchip(Element::Lithium));

    let mut floor2 = BTreeSet::new();
    floor2.insert(Component::Generator(Element::Hydrogen));

    let mut floor3 = BTreeSet::new();
    floor3.insert(Component::Generator(Element::Lithium));

    let initial_building_state = BuildingState {
        elevator_floor: 0,
        floors: [
            floor1,
            floor2,
            floor3,
            BTreeSet::new(),
        ],
    };
    let initial_world_state = WorldState {
        steps: 0,
        building: initial_building_state.clone()
    };
    let mut queue = VecDeque::new();
    queue.push_back(initial_world_state);
    let mut seen = BTreeSet::new();
    seen.insert(initial_building_state);
    let mut steps = 0;

    while let Some(world) = queue.pop_front() {
        if world.steps != steps {
            println!("{} step(s), queue: {}", world.steps, queue.len());
            steps = world.steps;
        }

        seen.insert(world.building.clone());

        if world.steps == 100 {
            panic!("too far!");
        }
        if world.in_end_state() {
            return world.steps;
        }

        for next_move in world.next_moves(&seen) {
            queue.push_back(next_move);
        }
    }

    panic!("Exhausted all possible moves without finding end condition!");
}

#[derive(Debug)]
pub struct WorldState {
    steps: u32,
    building: BuildingState,
}

impl WorldState {
    pub fn in_end_state(&self) -> bool {
        self.building.in_end_state()
    }

    pub fn next_moves(&self, seen: &BTreeSet<BuildingState>) -> Vec<WorldState> {
        self.building.next_moves(seen).into_iter().map(|b| {
            WorldState {
                steps: self.steps + 1,
                building: b,
            }
        }).collect()
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum Component {
    Microchip(Element),
    Generator(Element),
}

impl Component {
    fn is_fried(&self, others: &BTreeSet<Component>) -> bool {
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

#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum Element {
    Hydrogen,
    Lithium,
    Thulium,
    Plutonium,
    Strontium,
    Promethium,
    Ruthenium,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BuildingState {
    elevator_floor: usize,
    floors: [BTreeSet<Component>; 4],
}

impl Clone for BuildingState {
    fn clone(&self) -> BuildingState {
        BuildingState {
            elevator_floor: self.elevator_floor,
            floors: [
                self.floors[0].clone(),
                self.floors[1].clone(),
                self.floors[2].clone(),
                self.floors[3].clone(),
            ]
        }
    }
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

    pub fn elevator_combos(&self) -> Vec<Vec<Component>> {
        // It sure would be nice to be able to have BTreeSet<BTreeSet<Component>>
        // here so that I don't have to filter out duplicates myself.

        let mut result = vec![];
        let mut components = self.floors[self.elevator_floor].iter();
        while let Some(&component) = components.next() {
            // Could take just this item on the elevator
            result.push(vec![component]);
            // Could take this item and one other, if they don't fry
            for &remaining_component in components.clone() {
                let mut hs = BTreeSet::new();
                hs.insert(remaining_component);
                if !component.is_fried(&hs) {
                    result.push(vec![component, remaining_component]);
                }
            }
        }
        result
    }

    pub fn next_moves(&self, seen: &BTreeSet<BuildingState>)
                      -> Vec<BuildingState> {
        let mut valid_next_moves = vec![];
        for next_floor in self.next_floors() {
            for combo in self.elevator_combos() {
                let mut bs = (*self).clone();
                for &c in combo.iter() {
                    bs.floors[self.elevator_floor].remove(&c);
                    bs.floors[next_floor].insert(c);
                }
                bs.elevator_floor = next_floor;
                if !bs.has_fried_chips() && !seen.contains(&bs) {
                    valid_next_moves.push(bs);
                }
            }
        }
        valid_next_moves
    }

    pub fn has_fried_chips(&self) -> bool {
        for floor in self.floors.iter() {
            for component in floor {
                // This is doing an extra comparison to itself :shrug:
                if component.is_fried(floor) {
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
    use std::collections::BTreeSet;

    #[test]
    fn not_in_end_state() {
        let mut floor1 = BTreeSet::new();
        floor1.insert(Component::Microchip(Element::Hydrogen));
        floor1.insert(Component::Microchip(Element::Lithium));

        let mut floor2 = BTreeSet::new();
        floor2.insert(Component::Generator(Element::Hydrogen));

        let mut floor3 = BTreeSet::new();
        floor3.insert(Component::Generator(Element::Lithium));

        let world_state = WorldState {
            steps: 0,
            building: BuildingState {
                elevator_floor: 0,
                floors: [
                    floor1,
                    floor2,
                    floor3,
                    BTreeSet::new(),
                ],
            }
        };
        assert!( ! world_state.in_end_state() );
    }

    #[test]
    fn in_end_state() {
        let mut floor4 = BTreeSet::new();
        floor4.insert(Component::Generator(Element::Hydrogen));
        floor4.insert(Component::Microchip(Element::Hydrogen));
        floor4.insert(Component::Microchip(Element::Lithium));
        floor4.insert(Component::Generator(Element::Lithium));
        let world_state = WorldState {
            steps: 0,
            building: BuildingState {
                elevator_floor: 0,
                floors: [
                    BTreeSet::new(),
                    BTreeSet::new(),
                    BTreeSet::new(),
                    floor4,
                ],
            }
        };

        assert!(world_state.in_end_state());
    }

    #[test]
    fn two_microchips_not_fried() {
        let mut hs = BTreeSet::new();
        hs.insert(Component::Microchip(Element::Lithium));
        assert!(
            ! Component::Microchip(Element::Hydrogen).is_fried(
                &hs
            )
        )
    }

    #[test]
    fn microchip_alone_not_fried() {
        assert!(
            ! Component::Microchip(Element::Hydrogen).is_fried(
                &BTreeSet::new()
            )
        )
    }

    #[test]
    fn microchip_with_itself_not_fried() {
        let mut hs = BTreeSet::new();
        hs.insert(Component::Microchip(Element::Hydrogen));

        assert!(
            ! Component::Microchip(Element::Hydrogen).is_fried(
                &hs
            )
        )
    }

    #[test]
    fn two_generators_not_fried() {
        let mut hs = BTreeSet::new();
        hs.insert(Component::Generator(Element::Lithium));

        assert!(
            ! Component::Generator(Element::Hydrogen).is_fried(
                &hs
            )
        )
    }

    #[test]
    fn microchip_and_matching_generator_not_fried() {
        let mut hs = BTreeSet::new();
        hs.insert(Component::Generator(Element::Hydrogen));

        assert!(
            ! Component::Microchip(Element::Hydrogen).is_fried(
                &hs
            )
        )
    }

    #[test]
    fn microchip_attached_to_matching_generator_not_fried_by_other_generator() {
        let mut hs = BTreeSet::new();
        hs.insert(Component::Generator(Element::Lithium));
        hs.insert(Component::Generator(Element::Hydrogen));

        assert!(
            ! Component::Microchip(Element::Hydrogen).is_fried(
                &hs
            )
        )
    }

    #[test]
    fn microchip_without_its_generator_with_another_generator_fried() {
        let mut hs = BTreeSet::new();
        hs.insert(Component::Generator(Element::Lithium));

        assert!(
            Component::Microchip(Element::Hydrogen).is_fried(
                &hs
            )
        )
    }

    #[test]
    fn generator_with_a_microchip_without_its_generator_fried() {
        let mut hs = BTreeSet::new();
        hs.insert(Component::Microchip(Element::Lithium));

        assert!(
            Component::Generator(Element::Hydrogen).is_fried(
                &hs
            )
        )
    }

    #[test]
    fn generator_with_a_generator_not_fried() {
        let mut hs = BTreeSet::new();
        hs.insert(Component::Generator(Element::Lithium));
        assert!(
            ! Component::Generator(Element::Hydrogen).is_fried(
                &hs
            )
        )
    }

    #[test]
    fn generator_with_its_microchip_not_fried() {
        let mut hs = BTreeSet::new();
        hs.insert(Component::Microchip(Element::Hydrogen));

        assert!(
            ! Component::Generator(Element::Hydrogen).is_fried(
                &hs
            )
        )
    }

    #[test]
    fn generator_with_itself_not_fried() {
        let mut hs = BTreeSet::new();
        hs.insert(Component::Generator(Element::Hydrogen));

        assert!(
            ! Component::Generator(Element::Hydrogen).is_fried(
                &hs
            )
        )
    }

    #[test]
    fn generator_with_a_microchip_with_its_generator_not_fried() {
        let mut hs = BTreeSet::new();
        hs.insert(Component::Generator(Element::Lithium));
        hs.insert(Component::Microchip(Element::Lithium));

        assert!(
            ! Component::Generator(Element::Hydrogen).is_fried(
                &hs
            )
        )
    }
}
