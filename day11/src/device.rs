use crate::{Element, Gadget};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Device {
    pub element: Element,
    pub gadget: Gadget,
}

impl Device {
    pub const fn new(element: Element, gadget: Gadget) -> Device {
        Device { element, gadget }
    }

    pub const fn generator(element: Element) -> Device {
        Device {
            element,
            gadget: Gadget::Generator,
        }
    }

    pub const fn microchip(element: Element) -> Device {
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
