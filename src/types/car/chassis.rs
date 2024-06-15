#[derive(Default, Clone)]
pub enum Chassis {
    #[default]
    ChassisOne,
    ChassisTwo,
    ChassisThree,
    ChassisFour,
    ChassisFive,
}

impl Chassis {
    pub fn weight(&self) -> f64 {
        match self {
            Chassis::ChassisOne => 705.0,
            Chassis::ChassisTwo => 778.0,
            Chassis::ChassisThree => 861.0,
            Chassis::ChassisFour => 914.0,
            Chassis::ChassisFive => 996.0,
        }
    }

    pub fn tank_size(&self) -> f64 {
        match self {
            Chassis::ChassisOne => 33.6,
            Chassis::ChassisTwo => 37.0,
            Chassis::ChassisThree => 41.0,
            Chassis::ChassisFour => 43.5,
            Chassis::ChassisFive => 47.4,
        }
    }
}