//! Advent of Code - Day 11 Instructions

use std::collections::{BTreeSet, HashSet, VecDeque};
use std::fmt;

extern crate itertools;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Element {
    Cobalt,
    Curium,
    Hydrogen,
    Lithium,
    Plutonium,
    Promethium,
    Ruthenium,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gadget {
    Generator,
    Microchip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Device {
    pub element: Element,
    pub gadget: Gadget,
}

impl Device {
    pub fn new(element: Element, gadget: Gadget) -> Device {
        Device { element, gadget }
    }
}

pub const FLOORS: usize = 4;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
struct Floor {
    generators: BTreeSet<Element>,
    microchips: BTreeSet<Element>,
}

impl Floor {
    fn is_safe(&self) -> bool {
        if self.microchips.len() == 0 || self.generators.len() == 0 {
            return true;
        }

        // true if every generator is paired
        let mut gen_iter = self.generators.iter();
        let mut chip_iter = self.microchips.iter();

        while let Some(generator) = gen_iter.next() {
            let mut generator_paired = false;
            while let Some(chip) = chip_iter.next() {
                if chip == generator {
                    generator_paired = true;
                    break;
                }
            }
            if !generator_paired {
                return false;
            }
        }
        true
    }

    fn is_empty(&self) -> bool {
        self.microchips.is_empty() && self.generators.is_empty()
    }

    fn add_device(&mut self, device: Device) {
        use Gadget::*;
        match device.gadget {
            Generator => self.generators.insert(device.element),
            Microchip => self.microchips.insert(device.element),
        };
    }

    fn rm_device(&mut self, device: Device) {
        use Gadget::*;
        match device.gadget {
            Generator => self.generators.remove(&device.element),
            Microchip => self.microchips.remove(&device.element),
        };
    }

    fn devices(&self) -> Vec<Device> {
        let mut out = Vec::with_capacity(self.generators.len() + self.microchips.len());
        for elem in self.generators.iter() {
            out.push(Device {
                gadget: Gadget::Generator,
                element: *elem,
            });
        }
        for elem in self.microchips.iter() {
            out.push(Device {
                gadget: Gadget::Microchip,
                element: *elem,
            });
        }

        out
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
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

    pub fn next(&self, visited: &HashSet<State>) -> Vec<State> {
        let mut out = Vec::new();
        let devices = self.floors[self.elevator].devices();

        if self.elevator < (FLOORS - 1) {
            // for each device, take it up one floor (if possible)
            for d in devices.iter() {
                let mut next = self.clone();
                next.elevator += 1;
                next.floors[self.elevator].rm_device(*d);
                next.floors[next.elevator].add_device(*d);
                if next.is_safe() && !visited.contains(&next) {
                    out.push(next);
                }
            }

            // for each pair of devices, take them up one floor (if possible)
            for (d1, d2) in devices.iter().tuple_combinations() {
                let mut next = self.clone();
                next.elevator += 1;
                next.floors[self.elevator].rm_device(*d1);
                next.floors[self.elevator].rm_device(*d2);
                next.floors[next.elevator].add_device(*d1);
                next.floors[next.elevator].add_device(*d2);
                if next.is_safe() && !visited.contains(&next) {
                    out.push(next);
                }
            }
        }
        if self.elevator > 0 {
            // for each device, take it down one floor (if possible)
            for d in devices.iter() {
                let mut next = self.clone();
                next.elevator -= 1;
                next.floors[self.elevator].rm_device(*d);
                next.floors[next.elevator].add_device(*d);
                if next.is_safe() && !visited.contains(&next) {
                    out.push(next);
                }
            }

            // for each pair of devices, take them down one floor (if possible)
            for (d1, d2) in devices.iter().tuple_combinations() {
                let mut next = self.clone();
                next.elevator -= 1;
                next.floors[self.elevator].rm_device(*d1);
                next.floors[self.elevator].rm_device(*d2);
                next.floors[next.elevator].add_device(*d1);
                next.floors[next.elevator].add_device(*d2);
                if next.is_safe() && !visited.contains(&next) {
                    out.push(next);
                }
            }
        }

        out
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

pub fn goalseek(initial: State) -> Option<u32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_front((0, initial));

    let mut nsteps = 0;
    let mut count = 0;

    while let Some((steps, state)) = queue.pop_front() {
        if steps == nsteps {
            count += 1;
        } else {
            println!("visited {} states with {} steps", count, nsteps);
            nsteps = steps;
            count = 0;
        }

        if state.is_goal() {
            println!("{}", state);
            return Some(steps);
        }

        visited.insert(state.clone());

        for child in state.next(&visited) {
            queue.push_back((steps + 1, child));
        }
    }

    None
}

pub fn input() -> State {
    use Element::*;
    use Gadget::*;

    let mut s = State::default();

    s.add_device(0, Device::new(Promethium, Generator));
    s.add_device(0, Device::new(Promethium, Microchip));
    s.add_device(1, Device::new(Cobalt, Generator));
    s.add_device(1, Device::new(Curium, Generator));
    s.add_device(1, Device::new(Ruthenium, Generator));
    s.add_device(1, Device::new(Plutonium, Generator));
    s.add_device(2, Device::new(Cobalt, Microchip));
    s.add_device(2, Device::new(Curium, Microchip));
    s.add_device(2, Device::new(Ruthenium, Microchip));
    s.add_device(2, Device::new(Plutonium, Microchip));

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> State {
        use Element::*;
        use Gadget::*;

        let mut s = State::default();
        s.add_device(0, Device::new(Hydrogen, Microchip));
        s.add_device(0, Device::new(Lithium, Microchip));
        s.add_device(1, Device::new(Hydrogen, Generator));
        s.add_device(2, Device::new(Lithium, Generator));

        s
    }

    #[test]
    fn test_example() {
        assert_eq!(Some(11), goalseek(example()));
    }
}
