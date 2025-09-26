// Implements a function to check the guess against the secret
// Returns: 0 (Correct), 1 (Too High), -1 (Too Low)
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    println!("\nAssignment 3: Guessing Game");

    // Secret number
    let secret_number: i32 = 56; 
    
    // Variable to track the number of guesses
    let mut guess_count: i32 = 0;
    
    // Array to hold simulated guesses
    let simulated_guesses: [i32; 5] = [30, 45, 60, 83, 56]; 
    let mut guess_index: usize = 0; // Index for the array of simulated guesses
    
    // Use a loop to repeatedly guess
    loop {
        // Break if we run out of simulated guesses (failsafe)
        if guess_index >= simulated_guesses.len() {
            println!("Ran out of simulated guesses!");
            break; 
        }

        // Increment the guess count
        guess_count = guess_count + 1;
        
        // Set a mutable guess variable (simulating user input from the array)
        let guess: i32 = simulated_guesses[guess_index];
        
        // Call the check_guess function
        let result = check_guess(guess, secret_number);
        
        println!("\nGuess #{}: {}", guess_count, guess);
        
        // Use an if-else expression to print the result
        if result == 0 {
            println!("You guessed correctly!");
            break; // Exit the loop if the guess was correct
        } else if result == 1 {
            println!("The guess is too high.");
        } else { // result must be -1
            println!("The guess is too low.");
        }
        
        // Move to the next simulated guess
        guess_index = guess_index + 1;
    }
    
    // After the loop ends, print how many guesses it took
    println!("\nIt took {} guesses to find the secret number ({}).", guess_count, secret_number);
}