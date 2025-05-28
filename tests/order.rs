#![cfg(test)]

use temperature_converter::*;

// PartialEq + Eq
#[test]
fn test_equality_same_measure() {
    let t1 = Temperature::new(25.0, Measure::Celsius);
    let t2 = Temperature::new(25.0, Measure::Celsius);
    assert_eq!(t1, t2);
}

#[test]
fn test_equality_different_measures() {
    let t_celsius = Temperature::new(0.0, Measure::Celsius);
    let t_kelvin = Temperature::new(273.15, Measure::Kelvin);
    assert_eq!(t_celsius, t_kelvin); // 0°C == 273.15K

    let t_fahrenheit = Temperature::new(32.0, Measure::Fahrenheit);
    assert_eq!(t_celsius, t_fahrenheit); // 0°C == 32°F
}

#[test]
fn test_inequality() {
    let t1 = Temperature::new(10.0, Measure::Celsius);
    let t2 = Temperature::new(20.0, Measure::Celsius);
    assert_ne!(t1, t2);
}

// PartialOrd + Ord
#[test]
fn test_ordering_same_measure() {
    let t_low = Temperature::new(10.0, Measure::Celsius);
    let t_high = Temperature::new(20.0, Measure::Celsius);
    assert!(t_low < t_high);
    assert!(t_high > t_low);
}

#[test]
fn test_ordering_different_measures() {
    let t_celsius = Temperature::new(0.0, Measure::Celsius);
    let t_kelvin = Temperature::new(283.15, Measure::Kelvin); // 10°C
    assert!(t_celsius < t_kelvin); // 0°C < 10°C (283.15K)

    let t_fahrenheit = Temperature::new(50.0, Measure::Fahrenheit); // 10°C
    assert!(t_kelvin == t_fahrenheit); // 283.15K == 50°F
}

#[test]
fn test_ordering_symmetry() {
    let t_c = Temperature::new(0.0, Measure::Celsius);
    let t_f = Temperature::new(32.0, Measure::Fahrenheit);
    let t_k = Temperature::new(273.15, Measure::Kelvin);
    assert!(t_c == t_f);
    assert!(t_f == t_c);
    assert!(t_f == t_k);
    assert!(t_c == t_k);
}
