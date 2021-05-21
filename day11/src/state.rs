use crate::{Device, Floor};
use std::{
    collections::HashSet,
    fmt,
    hash::{Hash, Hasher},
    rc::Rc,
};

pub const FLOORS: usize = 4;

#[derive(Default, Debug, Clone)]
pub struct State {
    parent: Option<Rc<State>>,
    elevator: u8,
    floors: [Floor; FLOORS],
}

// `State` is not `Eq` because we ignore their parent when checking for equality.
// For the same reason, we need to manually implement `PartialEq` and `Hash`.

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

impl State {
    fn current_floor(&self) -> &Floor {
        &self.floors[self.elevator as usize]
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
        if let Some(ref state) = self.parent {
            1 + state.steps()
        } else {
            0
        }
    }

    pub fn next(&self, visited: &HashSet<State>) -> Vec<State> {
        // the subsequent code is way too complicated and should not be considered trustworthy
        unimplemented!()

        // let mut out = Vec::new();
        // let devices = self.floors[self.elevator].devices();

        // let heapself = Rc::new(self.clone());
        // let child = || {
        //     let mut c = self.clone();
        //     c.parent = Some(heapself.clone());
        //     c
        // };

        // if self.elevator < (FLOORS - 1) {
        //     let mut took_two_up = false;

        //     // for each pair of devices, take them up one floor (if possible)
        //     for (d1, d2) in devices.clone().tuple_combinations() {
        //         let mut next = child();
        //         next.elevator += 1;
        //         next.floors[self.elevator].rm_device(*d1);
        //         next.floors[self.elevator].rm_device(*d2);
        //         next.floors[next.elevator].add_device(*d1);
        //         next.floors[next.elevator].add_device(*d2);
        //         if next.is_safe() && !visited.contains(&next.isomorph()) {
        //             out.push(next);
        //             took_two_up = true;
        //         }
        //     }

        //     // only take one thing upstairs if we can't take two things upstairs
        //     if !took_two_up {
        //         // for each device, take it up one floor (if possible)
        //         for d in devices {
        //             let mut next = child();
        //             next.elevator += 1;
        //             next.floors[self.elevator].rm_device(*d);
        //             next.floors[next.elevator].add_device(*d);
        //             if next.is_safe() && !visited.contains(&next.isomorph()) {
        //                 out.push(next);
        //             }
        //         }
        //     }
        // }
        // if self.elevator > 0
        //     && self.floors[..self.elevator]
        //         .iter()
        //         .any(|floor| !floor.is_empty())
        // {
        //     let mut took_one_down = false;

        //     // for each device, take it down one floor (if possible)
        //     for d in devices.iter() {
        //         let mut next = child();
        //         next.elevator -= 1;
        //         next.floors[self.elevator].rm_device(*d);
        //         next.floors[next.elevator].add_device(*d);
        //         if next.is_safe() && !visited.contains(&next.isomorph()) {
        //             out.push(next);
        //             took_one_down = true;
        //         }
        //     }

        //     // only take two down if we can't take one down
        //     if !took_one_down {
        //         // for each pair of devices, take them down one floor (if possible)
        //         for (d1, d2) in devices.iter().tuple_combinations() {
        //             let mut next = child();
        //             next.elevator -= 1;
        //             next.floors[self.elevator].rm_device(*d1);
        //             next.floors[self.elevator].rm_device(*d2);
        //             next.floors[next.elevator].add_device(*d1);
        //             next.floors[next.elevator].add_device(*d2);
        //             if next.is_safe() && !visited.contains(&next.isomorph()) {
        //                 out.push(next);
        //             }
        //         }
        //     }
        // }

        // out
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
                floor,
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
