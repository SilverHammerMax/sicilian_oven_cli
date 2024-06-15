#[derive(Default)]
pub enum Tire {
    #[default]
    TireOne,
    TireTwo,
    TireThree,
    TireFour,
    TireFive,
}

impl Tire {
    pub fn agc(&self) -> f64 {
        match self {
            Tire::TireOne => 1.0,
            Tire::TireTwo => 4.0,
            Tire::TireThree => 6.0,
            Tire::TireFour => 8.0,
            Tire::TireFive => 10.0,
        }
    }

    pub fn ggc(&self) -> f64 {
        match self {
            Tire::TireOne => 10.0,
            Tire::TireTwo => 8.0,
            Tire::TireThree => 6.0,
            Tire::TireFour => 4.0,
            Tire::TireFive => 1.0,
        }
    }
}