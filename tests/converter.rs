#![cfg(test)]

use temperature_converter::*;

#[test]
fn test_absolute_zero() -> () {
    let temp: Temperature = Temperature::new(0.0, Measure::Celsius);
    assert_eq!(temp.convert_to(Measure::Celsius), 0.0);
    assert_eq!(temp.convert_to(Measure::Fahrenheit), 32.0);
    assert_eq!(temp.convert_to(Measure::Kelvin), 273.15);
}

#[test]
fn test_water_boiling() -> () {
    let temp: Temperature = Temperature::new(100.0, Measure::Celsius);
    assert_eq!(temp.convert_to(Measure::Celsius), 100.0);
    assert_eq!(temp.convert_to(Measure::Fahrenheit), 212.0);
    assert_eq!(temp.convert_to(Measure::Kelvin), 373.15);
}

#[test]
fn test_reversible_conversion() -> () {
    use rand::random_range;
    let random_value: f64 = random_range(-100.0..100.0);

    const EPSILON: f64 = 1e-10;

    let original: Temperature = Temperature::new(random_value, Measure::Celsius);
    let to_kelvin: f64 = original.convert_to(Measure::Kelvin);
    let to_fahrenheit: f64 = original.convert_to(Measure::Fahrenheit);

    let kelvin: Temperature = Temperature::new(to_kelvin, Measure::Kelvin);
    let fahrenheit: Temperature = Temperature::new(to_fahrenheit, Measure::Fahrenheit);

    assert!(
        (kelvin.convert_to(Measure::Celsius) - random_value).abs() < EPSILON,
        "Celsius-Kelvin conversion failed"
    );
    assert!(
        (fahrenheit.convert_to(Measure::Celsius) - random_value).abs() < EPSILON,
        "Celsius-Fahrenheit conversion failed"
    );
}
