use strum;

#[derive(strum::EnumIter)]
pub enum CarType {
    Lancia,
    Maserati,
    Ferrari,
    Fiat,
    ModifiedLancia,
}

pub struct Car {
    car_type: CarType,
    fuel_level: f64,
}

impl CarType {
    pub fn get_name(&self) -> &str {
        match self {
            CarType::Lancia => "Lancia D24",
            CarType::Maserati => "Maserati A6GCS",
            CarType::Ferrari => "Ferrari 500 Mondial",
            CarType::Fiat => "Fiat 8V Zagato",
            CarType::ModifiedLancia => "Lancia D24, but with a specially modified fuel tank. You fit 3 extra gallons of fuel in, but you lose some top speed",
        }
    }

    pub fn get_tank_size(&self) -> f64 {
        match self {
            CarType::Lancia => 26.0,
            CarType::Maserati => 34.0,
            CarType::Ferrari => 26.0,
            CarType::Fiat => 49.0,
            CarType::ModifiedLancia => 29.0,
        }
    }

    pub fn get_mileage(&self) -> f64 {
        match self {
            CarType::Lancia => 3.2,
            CarType::Maserati => 2.0,
            CarType::Ferrari => 2.0,
            CarType::Fiat => 2.0,
            CarType::ModifiedLancia => 3.2,
        }
    }

    pub fn get_horsepower(&self) -> f64 {
        match self {
            CarType::Lancia => 260.0,
            CarType::Maserati => 170.0,
            CarType::Ferrari => 170.0,
            CarType::Fiat => 127.0,
            CarType::ModifiedLancia => 260.0,
        }
    }
}

impl Car {
    pub fn new(car_type: CarType) -> Car {
        Car {
            fuel_level: car_type.get_tank_size(),
            car_type,
        }
    }

    pub fn get_fuel_level(&self) -> f64 {
        self.fuel_level
    }

    pub fn get_car_type(&self) -> &CarType {
        &self.car_type
    }
    pub fn refuel(&mut self) {
        self.fuel_level = self.car_type.get_tank_size();
        println!("All filled up!");
    }

    pub fn travel(&mut self) {
        self.fuel_level -= self.get_car_type().get_mileage();
    }
}
