use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Gadget {
    Generator,
    Microchip,
}

impl fmt::Display for Gadget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Generator => "G",
                Self::Microchip => "M",
            }
        )
    }
}
