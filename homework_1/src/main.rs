// The constant for the freezing point of water in Fahrenheit
const FREEZE_POINT_F: f64 = 32.0;

// Converts Fahrenheit to Celsius: C = (F - 32) * 5/9
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZE_POINT_F) * 5.0 / 9.0    
}

// Converts Celsius to Fahrenheit: F = C * 9/5 + 32
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZE_POINT_F
}

fn main() {
    println!("Assignment 1: Temperature Converter");
    
    // Declare a mutable variable with a temperature in Fahrenheit
    let mut temperature_f: f64 = 68.0;
    
    // Convert it to Celsius using your function and print the result
    let temperature_c = fahrenheit_to_celsius(temperature_f);
    println!("{}°F is {:.2}°C", temperature_f, temperature_c);
    
    // Use a loop to convert and print the next 5 integer temperatures
    let mut counter = 0;
    
    // Using a simple 'loop' with a counter to iterate 5 times
    loop {
        if counter >= 5 {
            break; // Exit the loop after 5 iterations
        }

        // Increment the temperature for the next iteration
        temperature_f = temperature_f + 1.0; 
        
        let next_c = fahrenheit_to_celsius(temperature_f);
        println!("Next: {}°F is {:.2}°C", temperature_f, next_c);
        
        counter = counter + 1;
    }
}