pub enum Measure {
    Celsius,
    Fahrenheit,
    Kelvin,
}

pub struct Temperature {
    value: f64,
    measure: Measure,
}

impl Temperature {
    pub fn new(value: f64, measure: Measure) -> Self {
        Self { value, measure }
    }

    pub fn convert_to(&self, temp_measure: Measure) -> f64 {
        match self.measure {
            Measure::Celsius => match temp_measure {
                Measure::Celsius => self.value,
                Measure::Fahrenheit => self.value * 9.0 / 5.0 + 32.0,
                Measure::Kelvin => self.value + 273.15,
            },
            Measure::Fahrenheit => match temp_measure {
                Measure::Fahrenheit => self.value,
                Measure::Celsius => self.value * 9.0 / 5.0 + 32.0,
                Measure::Kelvin => (self.value + 459.67) * 5.0 / 9.0,
            },
            Measure::Kelvin => match temp_measure {
                Measure::Kelvin => self.value,
                Measure::Celsius => self.value - 273.15,
                Measure::Fahrenheit => self.value * 9.0 / 5.0 - 459.67,
            },
        }
    }
}
