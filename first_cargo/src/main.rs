use std::io;                                            // Library for handeling standard input/output
use rand::Rng;                                          // Library for simulating randomness
use std::cmp::Ordering;                                 // Library for handling comparisons

fn main() {
    println!("Guess the number!");                      // Prints to the console line

    let secret_number = rand::thread_rng()
        .gen_range(1..101);                             // Creates a variable called secret_number and asigns it a random value between 1-100

    println!("The secret number is {}", secret_number);
    println!("Please iput your guess.");

    let mut guess = String::new();                      // Creates a mutable variable called guess and assigns it to a new empty string

    io::stdin()                                         // Calls stdin function that handles standard input for the terminal.
        .read_line(&mut guess)                          // Calls the read_line function that takes the input and appends it to a string. &mut guess is a mutable reference to the guess variable
        .expect("Failed to read line");                 // If the read_line function would fail expect would crash the program and display the error message

    let guess: u32 = guess                              // Converts guess into an unsigned 32 bit number 
        .trim().parse()
        .expect("Please type a numbrer!");
    
    println!("You guessed: {}", guess);                 // {} is a countainer for a variable. In this case guess

    match guess.cmp(&secret_number) {                   // A match function looks trough its so called arns and decides on the one matching the results of cmp (compare)
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}