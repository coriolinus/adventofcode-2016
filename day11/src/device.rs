use crate::{Element, Gadget};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Device {
    pub element: Element,
    pub gadget: Gadget,
}

impl Device {
    pub fn new(element: Element, gadget: Gadget) -> Device {
        Device { element, gadget }
    }

    pub fn generator(element: Element) -> Device {
        Device {
            element,
            gadget: Gadget::Generator,
        }
    }

    pub fn microchip(element: Element) -> Device {
        Device {
            element,
            gadget: Gadget::Microchip,
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.element, self.gadget)
    }
}

impl Ord for Device {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.element
            .cmp(&other.element)
            .then_with(|| self.gadget.cmp(&other.gadget))
    }
}

impl PartialOrd for Device {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
