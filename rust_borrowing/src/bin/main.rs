// Problem #1: String Concatenation with Borrowing

fn concat_strings(s1: &String, s2: &String) -> String {
    // Create a new, empty String.
    let mut result = String::new();
    
    // Append the contents of s1 (a string slice) to the new string.
    result.push_str(s1);
    
    // Append the contents of s2 (a string slice) to the new string.
    result.push_str(s2);

    // Return the new, concatenated string.
    result
}

// Problem #2: Clone and Modify

fn clone_and_modify(s: &String) -> String {
    // `.clone()` creates a new, owned String with the same data as `s`.
    // The original `s` remains unchanged.
    let mut modified_string = s.clone();
    
    // We can now append a new string slice to our owned, mutable copy.
    modified_string.push_str("World!");
    
    // Return the new, modified string.
    modified_string
}

// Problem #3: Mutable Reference Sum

fn sum(total: &mut i32, low: i32, high: i32) {
    // Iterate through the range from `low` to `high`, including `high`.
    for i in low..=high {
        // Dereference `total` with `*` to access the value it points to.
        // We can then add `i` to that value directly.
        *total += i;
    }
}

fn main() {
    // Problem #1 Test
    println!("--- Problem 1: String Concatenation ---");
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("Concatenated: {}", result); // Should print: "Hello, World!"
    println!("Original s1: {}", s1); // Original s1 is still valid
    println!("Original s2: {}", s2); // Original s2 is still valid
    println!();

    // Problem #2 Test
    println!("--- Problem 2: Clone and Modify ---");
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s);       // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
    println!();

    // Problem #3 Test
    println!("--- Problem 3: Mutable Reference Sum ---");
    let mut total = 0;
    let low = 0;
    let high = 100;

    println!("Initial total: {}", total);
    sum(&mut total, low, high);

    // The `total` variable has been modified directly by the `sum` function.
    // The `println!` macro can now access the updated value.
    println!("Sum from {} to {}: {}", low, high, total); // Should print: "Sum from 0 to 100: 5050"
}
