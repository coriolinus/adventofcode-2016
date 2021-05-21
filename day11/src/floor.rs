use std::{collections::HashSet, fmt};

use crate::{Device, Element, Gadget};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Floor {
    generators: HashSet<Element>,
    microchips: HashSet<Element>,
}

impl Floor {
    /// A floor is safe if there are no generators or every microchip is accompanied by its generator
    pub fn is_safe(&self) -> bool {
        self.generators.is_empty()
            || self
                .microchips
                .difference(&self.generators)
                .next()
                .is_none()
    }

    pub fn is_empty(&self) -> bool {
        self.microchips.is_empty() && self.generators.is_empty()
    }

    pub fn add_device(&mut self, device: Device) {
        use Gadget::*;
        match device.gadget {
            Generator => self.generators.insert(device.element),
            Microchip => self.microchips.insert(device.element),
        };
    }

    pub fn rm_device(&mut self, device: Device) {
        use Gadget::*;
        let was_present = match device.gadget {
            Generator => self.generators.remove(&device.element),
            Microchip => self.microchips.remove(&device.element),
        };
        // It is a logic error to remove a device which wasn't already
        // present, because removing an item is always followed by
        // inserting it on a neighboring floor.
        debug_assert!(was_present);
    }

    pub fn generators(&self) -> impl '_ + Iterator<Item = Device> + Clone {
        self.generators.iter().copied().map(Device::generator)
    }

    pub fn microchips(&self) -> impl '_ + Iterator<Item = Device> + Clone {
        self.microchips.iter().copied().map(Device::microchip)
    }

    pub fn devices(&self) -> impl '_ + Iterator<Item = Device> + Clone {
        self.generators().chain(self.microchips())
    }

    fn pairs(&self) -> impl '_ + Iterator<Item = &Element> + Clone {
        self.generators.intersection(&self.microchips)
    }

    fn unpaired_chips(&self) -> impl '_ + Iterator<Item = &Element> + Clone {
        self.microchips.difference(&self.generators)
    }

    fn unpaired_generators(&self) -> impl '_ + Iterator<Item = &Element> + Clone {
        self.generators.difference(&self.microchips)
    }

    /// Compute a value which precisely describes this floor while erasing information
    /// about which _particular_ elements are on it.
    ///
    /// Though this produces a `u64`, at most the lowest 16 bits may be set, to ensure
    /// that the state can compute a valid isomorph of all four floors without losing information.
    ///
    /// The 16 bits are used as follows:
    ///
    /// - `0..4` => count of paired microchips and generators
    /// - `4..8` => count of unpaired microchips
    /// - `8..12` => count of unpaired generators
    ///
    /// These fields are 4 bits each because there are 8 possible elements. To store the range
    /// `0..=8` requires 4 bits.
    pub fn isomorph(&self) -> u64 {
        let mut out: u64 = 0;

        let pairs_count = self.pairs().count();
        debug_assert!(pairs_count <= 0b1111);
        out |= (pairs_count as u64) << 0;

        let unpaired_chips_count = self.unpaired_chips().count();
        debug_assert!(unpaired_chips_count <= 0b1111);
        out |= (unpaired_chips_count as u64) << 4;

        let unpaired_generator_count = self.unpaired_generators().count();
        debug_assert!(unpaired_generator_count <= 0b1111);
        out |= (unpaired_generator_count as u64) << 8;

        // ensure we haven't exceeded our allotment of 12 bits
        debug_assert_eq!(out & !0xfff, 0);
        out
    }
}

impl fmt::Display for Floor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut devices: Vec<_> = self.devices().collect();
        devices.sort_unstable();
        for device in devices {
            write!(f, "{} ", device)?;
        }
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod isomorph_tests {
    use super::*;
    use enum_iterator::IntoEnumIterator;
    use rand::{thread_rng, Rng};

    /// Make a Floor with the specified number of pairs, unpaired chips, and unpaired generators.
    ///
    /// This is semi-randomized: the generated floor will always have the right count of each item,
    /// but the specific elements forming the items of each type are randomized.
    pub fn make_case<R: Rng>(
        rng: &mut R,
        pairs: usize,
        unpaired_chips: usize,
        unpaired_generators: usize,
    ) -> Floor {
        let mut floor = Floor::default();

        assert!(pairs + unpaired_chips + unpaired_generators <= Element::VARIANT_COUNT);
        // note: as this is an infinite iterator, its `next()` method can never fail
        let mut iter = Element::into_enum_iter()
            .cycle()
            .skip(rng.gen_range(0..Element::VARIANT_COUNT));

        for _ in 0..pairs {
            let elem = iter.next().unwrap();
            floor.add_device(Device::microchip(elem));
            floor.add_device(Device::generator(elem));
        }

        for _ in 0..unpaired_chips {
            let elem = iter.next().unwrap();
            floor.add_device(Device::microchip(elem));
        }

        for _ in 0..unpaired_generators {
            let elem = iter.next().unwrap();
            floor.add_device(Device::generator(elem));
        }

        assert_eq!(floor.pairs().count(), pairs);
        assert_eq!(floor.unpaired_chips().count(), unpaired_chips);
        assert_eq!(floor.unpaired_generators().count(), unpaired_generators);

        floor
    }

    /// Make an iterator over all legal combinations of `(pairs, unpaired_chips, unpaired_generators)`
    pub fn exhaustive_cases() -> impl Iterator<Item = (usize, usize, usize)> {
        (0..=Element::VARIANT_COUNT)
            .flat_map(|pairs| {
                (0..=(Element::VARIANT_COUNT - pairs))
                    .map(move |unpaired_chips| (pairs, unpaired_chips))
            })
            .flat_map(|(pairs, unpaired_chips)| {
                (0..=(Element::VARIANT_COUNT - pairs - unpaired_chips))
                    .map(move |unpaired_generators| (pairs, unpaired_chips, unpaired_generators))
            })
    }

    /// Make an iterator over distinct (by isomorph) floors
    pub fn exhaustive_floors() -> impl Iterator<Item = Floor> {
        let mut rng = thread_rng();

        exhaustive_cases().map(move |(pairs, unpaired_chips, unpaired_generators)| {
            make_case(&mut rng, pairs, unpaired_chips, unpaired_generators)
        })
    }

    #[test]
    fn test_default() {
        assert_eq!(Floor::default().isomorph(), 0);
    }

    #[test]
    fn test_pairs() {
        let mut rng = thread_rng();
        for n in 0..=Element::VARIANT_COUNT {
            let floor = make_case(&mut rng, n, 0, 0);
            assert_eq!(floor.isomorph(), n as u64);
        }
    }

    #[test]
    fn test_unpaired_chips() {
        let mut rng = thread_rng();
        for n in 0..=Element::VARIANT_COUNT {
            let floor = make_case(&mut rng, 0, n, 0);
            assert_eq!(floor.isomorph(), (n as u64) << 4);
        }
    }

    #[test]
    fn test_unpaired_generators() {
        let mut rng = thread_rng();
        for n in 0..=Element::VARIANT_COUNT {
            let floor = make_case(&mut rng, 0, 0, n);
            assert_eq!(floor.isomorph(), (n as u64) << 8);
        }
    }

    #[test]
    fn test_equivalence() {
        let mut rng = thread_rng();
        for (pairs, unpaired_chips, unpaired_generators) in exhaustive_cases() {
            // doesn't matter what the first test floor is...
            let a = make_case(&mut rng, pairs, unpaired_chips, unpaired_generators);
            // ... but the second one must be different (when possible)
            let mut b = make_case(&mut rng, pairs, unpaired_chips, unpaired_generators);
            let component_sum = pairs + unpaired_chips + unpaired_generators;
            if (1..Element::VARIANT_COUNT).contains(&component_sum) {
                while a == b {
                    b = make_case(&mut rng, pairs, unpaired_chips, unpaired_generators);
                }
                assert_ne!(a, b);
            }
            assert_eq!(a.isomorph(), b.isomorph());
        }
    }
}
