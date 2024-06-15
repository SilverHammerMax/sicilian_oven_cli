#[derive(Default, Clone)]
pub enum Engine {
    #[default]
    EngineOne,
    EngineTwo,
    EngineThree,
    EngineFour,
    EngineFive,
}

impl Engine {
    pub fn brake_horsepower(&self) -> f64 {
        match self {
            Engine::EngineOne => 112.0,
            Engine::EngineTwo => 200.0,
            Engine::EngineThree => 220.0,
            Engine::EngineFour => 276.0,
            Engine::EngineFive => 320.0,
        }
    }

    pub fn weight(&self) -> f64 {
        match self {
            Engine::EngineOne => 235.0,
            Engine::EngineTwo => 357.0,
            Engine::EngineThree => 372.0,
            Engine::EngineFour => 406.0,
            Engine::EngineFive => 432.0,
        }
    }

    pub fn fuel_usage(&self) -> f64 {
        match self {
            Engine::EngineOne => 2.4,
            Engine::EngineTwo => 3.2,
            Engine::EngineThree => 3.8,
            Engine::EngineFour => 4.6,
            Engine::EngineFive => 5.2,
        }
    }

    pub fn engine_type(&self) -> &str {
        match self {
            Engine::EngineOne => "I6",
            Engine::EngineTwo => "V6",
            Engine::EngineThree => "V8",
            Engine::EngineFour => "V10",
            Engine::EngineFive => "V12",
        }
    }
}