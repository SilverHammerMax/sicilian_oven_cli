#[derive(Default, Clone, Copy)]
pub enum Tire {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Tire {
    pub fn agc(&self) -> f64 {
        match self {
            Tire::One => 1.0,
            Tire::Two => 4.0,
            Tire::Three => 6.0,
            Tire::Four => 8.0,
            Tire::Five => 10.0,
        }
    }

    pub fn ggc(&self) -> f64 {
        match self {
            Tire::One => 10.0,
            Tire::Two => 8.0,
            Tire::Three => 6.0,
            Tire::Four => 4.0,
            Tire::Five => 1.0,
        }
    }
}
