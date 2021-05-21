use std::{collections::HashSet, fmt};

use crate::{Device, Element, Gadget};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Floor {
    pub(crate) generators: HashSet<Element>,
    pub(crate) microchips: HashSet<Element>,
}

impl Floor {
    /// A floor is safe if it contains no generators without a corresponding microchip.
    pub fn is_safe(&self) -> bool {
        self.generators
            .difference(&self.microchips)
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
        match device.gadget {
            Generator => self.generators.remove(&device.element),
            Microchip => self.microchips.remove(&device.element),
        };
    }

    pub fn devices(&self) -> impl '_ + Iterator<Item = Device> + Clone {
        self.generators
            .iter()
            .copied()
            .map(Device::generator)
            .chain(self.microchips.iter().copied().map(Device::microchip))
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
