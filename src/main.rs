use std::env::args;

use temperature_converter::{Measure, Temperature};
fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 4 {
        println!("⛔ Invalid input. You must enter two arguments.\n`Examples:\n# 20 fahrenheit to_celsius\n# 40 celsius to_fahrenheit");
        return;
    }

    let digit: f64 = args
        .get(1)
        .unwrap()
        .parse::<f64>()
        .expect("⛔ Invalid input. You must enter number like 3.14, 4.14");

    let temperature: Temperature;

    match &args.get(2).unwrap().to_lowercase()[0..] {
        "celsius" => temperature = Temperature::new(digit, Measure::Celsius),
        "fahrenheit" => temperature = Temperature::new(digit, Measure::Fahrenheit),
        "kelvin" => temperature = Temperature::new(digit, Measure::Kelvin),
        _ => panic!(
            "⛔ Invalid input. You must type a current temperature measure (celsius or fahrenheit)"
        ),
    }

    // convert String argument to &'static str with slices [0..] for faster parse
    match &args.get(3).unwrap().to_lowercase()[0..] {
        "to_celsius" | "celsius" => println!("{:.2}°C", temperature.convert_to(Measure::Celsius)),
        "to_fahrenheit" | "fahrenheit" => {
            println!("{:.2}°F", temperature.convert_to(Measure::Fahrenheit))
        }
        "to_kelvin" | "kelvin" => {
            println!("{:.2}°K", temperature.convert_to(Measure::Kelvin))
        }
        _ => panic!(
            "⛔ Invalid input. You must type a temperature measure (to_celsius or to_fahrenheit)"
        ),
    };
}
