const EPSILON: f64 = 1e-10;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Measure {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[derive(Debug)]
pub struct Temperature {
    value: f64,
    measure: Measure,
}

impl Temperature {
    pub fn new(value: f64, measure: Measure) -> Self {
        Self { value, measure }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    fn compare_same_measure(&self, other: &Self) -> (f64, f64) {
        if self.measure == other.measure { return (self.get_value(), other.get_value()) }
        (self.get_value(), other.convert_to(self.measure))
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
                Measure::Celsius => (self.value - 32.0) * 5.0 / 9.0,
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

impl PartialEq for Temperature {
    fn eq(&self, other: &Self) -> bool {
        let (a, b) = self.compare_same_measure(other);
        (a - b).abs() <= EPSILON
    }
}

impl Eq for Temperature {}

impl PartialOrd for Temperature {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (a, b) = self.compare_same_measure(other);
        a.partial_cmp(&b)
    }
}

impl Ord for Temperature {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (a, b) = self.compare_same_measure(other);
        a.total_cmp(&b)
    }
}