// Implements a function to check if a number is even
fn is_even(n: i32) -> bool {
    // If the remainder when divided by 2 is 0, the number is even
    n % 2 == 0
}

fn main() {
    println!("\nAssignment 2: Number Analyzer");
    
    // Create an array of 10 integer numbers
    let numbers: [i32; 10] = [1, 2, 3, 5, 8, 10, 15, 21, 30, 42];
    
    
    
    // Use a for loop to iterate through the array for analysis
    for number in numbers {
        // Check for FizzBuzz condition first (divisible by 3 and 5)
        if number % 3 == 0 && number % 5 == 0 {
            println!("{}: FizzBuzz", number);
        } 
        // Check for Fizz (divisible by 3)
        else if number % 3 == 0 {
            println!("{}: Fizz", number);
        } 
        // Check for Buzz (divisible by 5)
        else if number % 5 == 0 {
            println!("{}: Buzz", number);
        } 
        // Otherwise, print whether it's even or odd
        else {
            if is_even(number) {
                println!("{}: Even", number);
            } else {
                println!("{}: Odd", number);
            }
        }
    }
    
    
    
    // Use a while loop to find and print the sum of all numbers
    let mut sum = 0;
    let mut index = 0;
    let array_length = numbers.len(); // Get the length of the array
    
    while index < array_length {
        sum = sum + numbers[index];
        index = index + 1;
    }
    
    println!("The sum of all numbers is: {}", sum);
    
    
    
    // Use a loop to find and print the largest number
    // We start by assuming the first element is the largest
    let mut largest = numbers[0]; 
    let mut current_index = 1; // Start checking from the second element
    
    // Using a simple 'loop' for finding the max
    loop {
        if current_index >= array_length {
            break; // Exit when we've checked all elements
        }

        let current_number = numbers[current_index];
        
        if current_number > largest {
            largest = current_number; // Update if the current number is larger
        }

        current_index = current_index + 1;
    }
    
    println!("The largest number is: {}", largest);
}