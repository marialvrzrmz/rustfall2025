const FREEZE_POINT_FAHRENHEIT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZE_POINT_FAHRENHEIT) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZE_POINT_FAHRENHEIT
}

fn main() {
    let mut temp_fahrenheit = 32.0;

    // Convert and print the initial temperature
    let temp_celsius = fahrenheit_to_celsius(temp_fahrenheit);
    println!("{}°F is {:.2}°C", temp_fahrenheit, temp_celsius);

    // Convert and print the next 5 integer temperatures
    println!("\nConverting the next 5 temperatures:");
    for i in 1..=5 {
        let next_temp_fahrenheit = temp_fahrenheit + i as f64;
        let next_temp_celsius = fahrenheit_to_celsius(next_temp_fahrenheit);
        println!("{}°F is {:.2}°C", next_temp_fahrenheit, next_temp_celsius);
    }
}