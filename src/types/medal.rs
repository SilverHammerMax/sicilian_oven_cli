use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub enum Medal {
    None,
    Bronze,
    Silver,
    Gold,
    Author,
}

impl Display for Medal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Medal::None => write!(f, ""),
            Medal::Bronze => write!(f, "[BRONZE]"),
            Medal::Silver => write!(f, "[SILVER]"),
            Medal::Gold => write!(f, "[GOLD]"),
            Medal::Author => write!(f, "[AUTHOR]"),
        }
    }
}
