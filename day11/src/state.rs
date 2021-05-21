use crate::{Device, Floor};
use itertools::Itertools;
use std::{
    array,
    borrow::Borrow,
    collections::HashSet,
    fmt,
    hash::{Hash, Hasher},
    ops::{Index, IndexMut},
    rc::Rc,
};

pub const FLOORS: usize = 4;

#[derive(Default, Debug, Clone, Eq)]
pub struct State {
    parent: Option<Rc<State>>,
    elevator: u8,
    floors: [Floor; FLOORS],
}

// Because we want to ignore the parent and only check isomorphism,
// we need to manually implement `PartialEq` and `Hash`.

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.elevator == other.elevator && self.isomorph() == other.isomorph()
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_u8(self.elevator);
        hasher.write_u64(self.isomorph());
    }
}

impl Index<u8> for State {
    type Output = Floor;

    fn index(&self, index: u8) -> &Self::Output {
        &self.floors[index as usize]
    }
}

impl IndexMut<u8> for State {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.floors[index as usize]
    }
}

impl State {
    pub fn parent(&self) -> Option<&State> {
        self.parent.as_ref().map(|rc| rc.borrow())
    }

    fn floors_below(&self) -> impl Iterator<Item = &Floor> {
        (0..(self.elevator as usize)).map(move |floor| &self.floors[floor])
    }

    pub fn is_safe(&self) -> bool {
        self.floors.iter().all(|floor| floor.is_safe())
    }

    pub fn is_goal(&self) -> bool {
        self.floors[..FLOORS - 1]
            .iter()
            .all(|floor| floor.is_empty())
    }

    pub fn add_device(&mut self, floor: usize, device: Device) {
        self.floors[floor].add_device(device);
    }

    pub fn steps(&self) -> usize {
        if let Some(ref parent) = self.parent {
            1 + parent.steps()
        } else {
            0
        }
    }

    // Compute all reasonable children of this state.
    //
    // Follows these rules:
    //
    // - don't include unsafe children
    // - if all floors below the current floor are empty, don't move anything down
    // - if possible to move a pair upstairs, don't bother bringing single items upstairs
    // - if possible to move a single item downstairs, don't bother bringing pairs downstairs
    // - exclude child states isomorphic to visited states
    pub fn children(&self, visited: &HashSet<State>) -> Vec<State> {
        let parent = Some(Rc::new(self.clone()));
        let mut children = Vec::new();

        let pairs = self[self.elevator]
            .devices()
            .cartesian_product(self[self.elevator].devices())
            .filter(|(a, b)| a != b);

        // consider moving pairs or single items upstairs
        if (self.elevator as usize) < FLOORS - 1 {
            let mut moved_pair = false;

            let make_child = || State {
                parent: parent.clone(),
                elevator: self.elevator + 1,
                floors: self.floors.clone(),
            };
            let move_device = |child: &mut State, device| {
                child[self.elevator].rm_device(device);
                child[self.elevator + 1].add_device(device);
            };

            for (a, b) in pairs.clone() {
                let mut child = make_child();
                for device in array::IntoIter::new([a, b]) {
                    move_device(&mut child, device);
                }
                if !visited.contains(&child) && child.is_safe() {
                    children.push(child);
                    moved_pair = true;
                }
            }

            // only move single items up if we didn't manage to move a pair
            if !moved_pair {
                for device in self[self.elevator].devices() {
                    let mut child = make_child();
                    move_device(&mut child, device);

                    if !visited.contains(&child) && child.is_safe() {
                        children.push(child);
                    }
                }
            }
        }

        // consider moving single items or pairs downstairs
        if self.elevator > 0
            && !self
                .floors_below()
                .all(|floor| floor.devices().next().is_none())
        {
            let mut moved_single = false;

            let make_child = || State {
                parent: parent.clone(),
                elevator: self.elevator - 1,
                floors: self.floors.clone(),
            };
            let move_device = |child: &mut State, device| {
                child[self.elevator].rm_device(device);
                child[self.elevator - 1].add_device(device);
            };

            for device in self[self.elevator].devices() {
                let mut child = make_child();
                move_device(&mut child, device);

                if !visited.contains(&child) && child.is_safe() {
                    children.push(child);
                    moved_single = true;
                }
            }

            // only move pairs down if we didn't manage to move a single
            if !moved_single {
                for (a, b) in pairs {
                    let mut child = make_child();
                    for device in array::IntoIter::new([a, b]) {
                        move_device(&mut child, device);
                    }

                    if !visited.contains(&child) && child.is_safe() {
                        children.push(child);
                    }
                }
            }
        }

        children
    }

    /// Compute a single value corresponding to the distribution of devices among
    /// the floors of this state.
    ///
    /// This intentially erases the distinction between different elements; the only
    /// information of interest are the numbers of unpaired generators,
    fn isomorph(&self) -> u64 {
        let isomorph = self
            .floors
            .iter()
            .enumerate()
            .map(|(idx, floor)| floor.isomorph() << (idx * 64 / FLOORS))
            .fold(0, |acc, elem| acc | elem);
        // 12 bits per floor isomorph; 4 floors
        debug_assert_eq!(isomorph & !0x0fff_0fff_0fff_0fff, 0);
        isomorph
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for floor in (0..FLOORS).rev() {
            writeln!(
                f,
                "F{} {} {}",
                floor + 1,
                if self.elevator == floor as u8 {
                    'E'
                } else {
                    '|'
                },
                self.floors[floor],
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod isomorph_tests {
    use super::*;
    use crate::{floor::isomorph_tests::exhaustive_floors, Element::*};

    fn example() -> State {
        let mut s = State::default();
        s.add_device(0, Device::microchip(Hydrogen));
        s.add_device(0, Device::microchip(Lithium));
        s.add_device(1, Device::generator(Hydrogen));
        s.add_device(2, Device::generator(Lithium));

        s
    }

    #[test]
    fn test_simple_isomorph_equivalence() {
        let mut s1 = State::default();
        let mut s2 = State::default();

        assert_eq!(s1.isomorph(), s2.isomorph());

        s1.add_device(0, Device::microchip(Hydrogen));
        s2.add_device(0, Device::microchip(Lithium));
        assert_eq!(s1.isomorph(), s2.isomorph());

        s1.add_device(1, Device::generator(Hydrogen));
        s2.add_device(1, Device::generator(Lithium));
        assert_eq!(s1.isomorph(), s2.isomorph());
    }

    #[test]
    fn test_isomorph_equivalence() {
        let equiv = {
            let mut s = State::default();
            s.add_device(0, Device::microchip(Plutonium));
            s.add_device(0, Device::microchip(Cobalt));
            s.add_device(1, Device::generator(Plutonium));
            s.add_device(2, Device::generator(Cobalt));

            s
        };

        assert_eq!(example().isomorph(), equiv.isomorph());
    }

    #[test]
    fn test_floor_deconfliction() {
        for floor_idx in 0..FLOORS {
            for floor in exhaustive_floors() {
                let floor_isomorph = floor.isomorph();

                let mut s = State::default();
                s.floors[floor_idx] = floor;

                let shift = floor_idx * 16;
                assert_eq!(s.isomorph() & !(0xfff << shift), 0);
                assert_eq!(s.isomorph(), floor_isomorph << shift);
            }
        }
    }
}
