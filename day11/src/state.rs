use crate::{Device, Element, Floor};
use std::{collections::HashSet, fmt, rc::Rc};

pub const FLOORS: usize = 4;

/// an Isomorph is a value which corresponds to a given state, regardless of
/// which particular elements are where.
pub type Isomorph = u64;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct State {
    parent: Option<Rc<State>>,
    elevator: usize,
    floors: [Floor; FLOORS],
}

impl State {
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

    pub fn next(&self, visited: &HashSet<Isomorph>) -> Vec<State> {
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

    fn isomorph(&self) -> Isomorph {
        // abandon generality all ye who enter here!
        //
        // most of the rest of this code just works no matter how many floors
        // or how many elements are present. However, this function is strictly
        // limited to 4 floors and 8 elements. This suffices for part 1 of the
        // problem; hopefully it does as well for part 2.

        // we segment the 64 bits of Isomorph into four 16-bit sequences, one
        // per floor. Of those 16 bits, the low 8 identify the potential presence
        // of up to 8 microchips; the high 8 identify the potential presence
        // of up to 8 generators.
        //
        // This is key: there is no fixed mapping between an element and the
        // isomorph position used to represent it. Instead, the first element
        // encountered gets index 0, the next one index 1, etc.

        let mut encountered_elements = [None; 8];
        let mut next_ee_idx = 0;
        let mut isomorph = 0;

        let mut element_index = |element: Element| match encountered_elements
            .iter()
            .enumerate()
            .find(|(_, &ee)| ee == Some(element))
        {
            None => {
                let idx = next_ee_idx;
                if idx >= 8 {
                    panic!("too many elements discovered")
                }
                next_ee_idx += 1;
                encountered_elements[idx] = Some(element);
                idx
            }
            Some((idx, _)) => idx,
        };

        for (floor_idx, floor) in self.floors.iter().enumerate() {
            for g_el in floor.generators.iter() {
                let offset = (16 * floor_idx) + 8 + element_index(*g_el);
                isomorph |= 1 << offset;
            }
            for m_el in floor.microchips.iter() {
                let offset = (16 * floor_idx) + element_index(*m_el);
                isomorph |= 1 << offset;
            }
        }

        isomorph
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for floor in (0..FLOORS).rev() {
            write!(
                f,
                "F{} {} ",
                floor,
                if self.elevator == floor { 'E' } else { '.' }
            )?;
            for device in self.floors[floor].devices() {
                write!(f, "{:?} ", device)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
