use std::io; // Import the io module from the standard library

// Define a function to get input from the user
fn get_input() -> String {
    let mut input = String::new(); // Create an empty String to hold the input
    io::stdin().read_line(&mut input).unwrap(); // Read input from stdin into the String
    input // Return the input String
}

// Define a function to convert a string of whitespace-separated integers to a sorted vector of integers
fn str_to_sorted_vec(input_str: &str) -> Vec<i32> {
    let mut a: Vec<i32> = input_str // Convert the input string to a vector of i32 integers
        .split_whitespace() // Split the input string into whitespace-separated substrings
        .map(|s| s.parse().unwrap()) // Convert each substring to an i32 integer
        .collect(); // Collect the i32 integers into a vector
    a.sort(); // Sort the vector in ascending order
    a // Return the sorted vector
}

fn main() {
    let input = get_input(); // Get input from the user
    let a = str_to_sorted_vec(&input); // Convert the input string to a sorted vector of integers

    let sum1: i32 = a.iter().take(4).sum(); // Calculate the sum of the first 4 elements in the vector
    let sum2: i32 = a.iter().skip(1).sum(); // Calculate the sum of all elements in the vector except the first one
    println!("{} {}", sum1, sum2); // Output the two sums
}
