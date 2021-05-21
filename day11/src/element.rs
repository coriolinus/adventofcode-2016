use std::fmt;

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

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Cobalt => "Co",
                Self::Curium => "Cu",
                Self::Hydrogen => "H",
                Self::Lithium => "Li",
                Self::Plutonium => "Pu",
                Self::Promethium => "Pm",
                Self::Ruthenium => "Ru",
            }
        )
    }
}
